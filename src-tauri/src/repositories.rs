use std::collections::HashMap;
use std::vec;

use dotenv_enum::EnvironmentVariable;
use serde::{Serialize, Deserialize};
use crate::toml_file_serializer::{load_all_files, remove_all_toml, save_with_folders};
use crate::git_command::{BranchDetails, Branches};
use crate::LocationsEnv;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, Default, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub struct Repositories {
    pub path: String,
    pub selected_branch: Option<String>,
    pub branches: Vec<Branches>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct RepositoriesToml {
    pub path: String,
    pub selected_branch: Option<String>,
    pub branches: HashMap<String, BranchDetails>,
}

impl Repositories {
    fn to_toml_formattable(self) -> RepositoriesToml {
        RepositoriesToml {
            path: self.path,
            selected_branch: self.selected_branch,
            branches: self.branches.iter()
                .fold(HashMap::new(), |mut acc, branch| {
                    let entry = branch.to_hashmap_entry();
                    acc.insert(entry.0, entry.1);
                    acc
                }),
        }
    }

    fn from_toml_formattable(repo: RepositoriesToml) -> Self {
        Self {
            path: repo.path,
            selected_branch: repo.selected_branch,
            branches: repo.branches.iter()
                .fold(Vec::new(), |mut acc: Vec<Branches>, branch| {
                    acc.push(Branches::from_branch_details(branch.0.clone(), branch.1.clone()));
                    acc
                }),
        }
    }
}

#[tauri::command]
pub fn save_repositories(cached_repositories: HashMap<String, Repositories>) -> Result<(), String> {
    let paths = vec![LocationsEnv::CacheFolder.unwrap_value()];
    remove_all_toml(paths.clone()).expect("Failed to remove");
    cached_repositories.iter().for_each(|(file_name, repo)| {
        save_with_folders::<RepositoriesToml>(
            paths.clone(),
            vec![format!("{file_name}.toml")],
            &repo.clone().to_toml_formattable(),
        ).expect(format!(
            "Cannot save to './{}/{}.toml'",
            LocationsEnv::CacheFolder.unwrap_value(),
            file_name
        ).as_str());
    });
    Ok(())
}

#[tauri::command]
pub fn load_repositories() -> Result<HashMap<String, Repositories>, String> {
    Ok(load_all_files::<RepositoriesToml>(
        LocationsEnv::CacheFolder.unwrap_value()
    )?.iter().fold(HashMap::new(), |mut acc, (name, repo)| {
        acc.insert(name.clone(), Repositories::from_toml_formattable(repo.clone()));
        acc
    }))
}
