use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::commands::CliCommand;
use crate::commands::errors::InvalidCommandError;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GitCommand {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub struct BranchDetails {
    pub is_origin: bool,
    pub worktree_path: Option<String>
}

impl BranchDetails {
    pub fn from_fetch() -> Self {
        Self {
            worktree_path: None,
            is_origin: true
        }
    }
}

#[tauri::command]
pub fn get_branch(paths: Vec<String>) -> Result<HashMap<String, HashMap<String, BranchDetails>>, String> {
    let path: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    match send_to_git(GitCommand { command: "branch".to_string(), args: vec![] }, path.clone()) {
        Ok(response) => Ok(parse_branches(&response)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn clone_repo(link: String, path: String, name: String) -> Result<HashMap<String, HashMap<String, BranchDetails>>, String> {
    match send_to_git(GitCommand { command: "clone".to_string(), args: vec!["--bare".to_string(), link, name] }, vec![PathBuf::from(path)]) {
        Ok(response) => Ok(parse_branches(&response)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn add_worktree(name: String, path: String) -> Result<HashMap<String, HashMap<String, BranchDetails>>, String> {
    match send_to_git(GitCommand { command: "worktree".to_string(), args: vec!["add".to_string(), name] }, vec![PathBuf::from(path)]) {
        Ok(response) => Ok(parse_branches(&response)),
        Err(err) => Err(err.to_string()),
    }
}

fn send_to_git(command: GitCommand, paths: Vec<PathBuf>) -> error_stack::Result<HashMap<String, String>, InvalidCommandError> {
    CliCommand::new(command.command, command.args)
        .execute_catching_stdout(paths)
}

fn parse_branches(stdouts: &HashMap<String, String>) -> HashMap<String, HashMap<String, BranchDetails>> {
    let map: Vec<(String, Vec<(String, BranchDetails)>)> = stdouts.iter()
        .map(|(path, stdout)| (path.clone(), stdout.split("\n")
            .map(|branch| branch.to_string())
            .map(|branch| branch.trim().to_string())
            .filter(|branch| !branch.is_empty())
            .map(|branch| (branch.replace("*", "").trim().to_string(), BranchDetails::from_fetch()))
            .collect::<Vec<(String, BranchDetails)>>()))
        .collect();

    map.iter().fold(HashMap::new(), |mut acc, entry| {
        acc.insert(entry.0.clone(), entry.1.iter().fold(HashMap::new(), |mut branch_acc, branch_entry| {
            branch_acc.insert(branch_entry.0.clone().replace("+ ", ""), branch_entry.1.clone());
            branch_acc
        }));
        acc
    })
}