use std::collections::HashMap;
use std::vec;

use dotenv_enum::EnvironmentVariable;
use serde::{Serialize, Deserialize};
use crate::toml_file_serializer::{load_all_files, remove_all_toml, save_with_folders};
use crate::git_command::{BranchDetails};
use crate::LocationsEnv;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, Default, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub struct Repositories {
    pub path: String,
    pub selected_branch: Option<String>,
    pub branches: HashMap<String, BranchDetails>,
}

#[tauri::command]
pub fn save_repositories(cached_repositories: HashMap<String, Repositories>) -> Result<(), String> {
    let paths = vec![LocationsEnv::CacheFolder.unwrap_value()];
    remove_all_toml(paths.clone()).expect("Failed to remove");
    cached_repositories.iter().for_each(|(file_name, repo)| {
        save_with_folders::<Repositories>(
            paths.clone(),
            vec![format!("{file_name}.toml")],
            &repo.clone(),
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
    Ok(load_all_files::<Repositories>(
        LocationsEnv::CacheFolder.unwrap_value()
    )?)
}
