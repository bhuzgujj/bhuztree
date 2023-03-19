use std::collections::HashMap;
use std::fs::create_dir;
use std::path::PathBuf;
use dotenv_enum::EnvironmentVariable;
use serde::{Deserialize, Serialize};
use crate::commands::CliCommand;
use crate::commands::errors::InvalidCommandError;
use ts_rs::TS;
use crate::LocationsEnv;
use crate::repositories::Repositories;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GitCommand {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub struct BranchDetails {
    pub is_origin: bool,
}

impl BranchDetails {
    pub fn from_fetch() -> Self {
        Self {
            is_origin: true
        }
    }
}

#[tauri::command]
pub fn get_branch(name: String) -> Result<HashMap<String, Repositories>, String> {
    match send_to_git(GitCommand { command: "branch".to_string(), args: vec![] }, vec![get_base_repos_dir().unwrap().join(&name)]) {
        Ok(response) => Ok(parse_branches(&response, Some(name))),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn clone_repo(link: String, path: String, name: String) -> Result<HashMap<String, Repositories>, String> {
    let full_path = get_base_repos_dir().unwrap();
    match send_to_git(GitCommand { command: "clone".to_string(), args: vec!["--bare".to_string(), link, name.clone()] }, vec![full_path.clone()]) {
        Ok(_) => {
            let _ = create_dir(format!("{}/{}", path, name.clone()));
            Ok(HashMap::new())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn add_worktree(name: String, path: String, repo: Repositories) -> Result<HashMap<String, Repositories>, String> {
    let full_path = get_base_repos_dir().unwrap().join(&path);
    match send_to_git(GitCommand {
        command: "worktree".to_string(),
        args: vec![
            "add".to_string(),
            get_worktree_path(&name, repo),
            name.clone(),
        ],
    }, vec![full_path.clone()]) {
        Ok(response) => Ok(parse_branches(&response, Some(name))),
        Err(err) => Err(err.to_string()),
    }
}

fn get_worktree_path(name: &String, repo: Repositories) -> String {
    let mut path = repo.worktrees_path.unwrap_or_else(|| ".".to_string());
    if path != "." {
        let _ = path.push_str(format!("\\\\{}", name.clone()).as_str());
    }
    path
}

fn send_to_git(command: GitCommand, paths: Vec<PathBuf>) -> error_stack::Result<HashMap<String, String>, InvalidCommandError> {
    CliCommand::new(command.command, command.args)
        .execute_catching_stdout(paths)
}

fn get_base_repos_dir() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(home) => {
            let dir = home
                .join(LocationsEnv::Folder.unwrap_value())
                .join(LocationsEnv::CacheFolder.unwrap_value());
            let _ = create_dir(&dir);
            Some(dir)
        }
        None => None,
    }
}

fn parse_branches(stdouts: &HashMap<String, String>, name: Option<String>) -> HashMap<String, Repositories> {
    stdouts.iter()
        .map(|(path, stdout)| (path.clone(), stdout.split("\n")
            .map(|branch| branch.to_string())
            .map(|branch| branch.trim().to_string())
            .filter(|branch| !branch.is_empty())
            .map(|branch| (branch.replace("*", "").trim().to_string(), BranchDetails::from_fetch()))
            .collect::<Vec<(String, BranchDetails)>>()))
        .fold(HashMap::new(), |mut acc, entry| {
            acc.insert(name.clone().unwrap_or_else(|| entry.0.clone()), Repositories::from_entry(entry));
            acc
        })
}