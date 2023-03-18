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
pub struct Branches {
    pub name: String,
    pub details: BranchDetails,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub struct BranchDetails {
    pub is_current: bool,
}

impl Branches {
    fn from_string(branch: String) -> Self {
        Self {
            details: BranchDetails {
                is_current: branch.contains("*")
            },
            name: branch.replace("*", "").trim().to_string(),
        }
    }

    pub fn to_hashmap_entry(&self) -> (String, BranchDetails) {
        (self.name.clone(), self.details.clone())
    }

    pub fn from_branch_details(name: String, details: BranchDetails) -> Self {
        Self { details, name }
    }
}

#[tauri::command]
pub fn get_branch(paths: Vec<String>) -> Result<HashMap<String, Vec<Branches>>, String> {
    let path: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    match send_to_git(GitCommand { command: "branch".to_string(), args: vec![] }, path.clone()) {
        Ok(response) => Ok(parse_branches(&response)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn clone_repo(link: String, path: String, name: String) -> Result<HashMap<String, Vec<Branches>>, String> {
    match send_to_git(GitCommand { command: "clone".to_string(), args: vec!["--bare".to_string(), link, name] }, vec![PathBuf::from(path)]) {
        Ok(response) => Ok(parse_branches(&response)),
        Err(err) => Err(err.to_string()),
    }
}

fn send_to_git(command: GitCommand, paths: Vec<PathBuf>) -> error_stack::Result<HashMap<String, String>, InvalidCommandError> {
    CliCommand::new(command.command, command.args)
        .execute_catching_stdout(paths)
}

fn parse_branches(stdouts: &HashMap<String, String>) -> HashMap<String, Vec<Branches>> {
    let map: Vec<(String, Vec<Branches>)> = stdouts.iter()
        .map(|(path, stdout)| (path.clone(), stdout.split("\n")
            .map(|branch| branch.to_string())
            .map(|branch| branch.trim().to_string())
            .filter(|branch| !branch.is_empty())
            .map(|branch| Branches::from_string(branch))
            .collect::<Vec<Branches>>()))
        .collect();

    map.iter().fold(HashMap::new(), |mut acc, entry| {
        acc.insert(entry.0.clone(), entry.1.clone());
        acc
    })
}