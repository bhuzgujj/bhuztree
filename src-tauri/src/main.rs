#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#![feature(custom_test_frameworks)]
extern crate dotenv_enum;

mod serializer;
mod string_file_serializer;
mod toml_file_serializer;

mod settings;
mod git_command;
mod repositories;
mod logging;
mod commands;

use crate::settings::{save_settings, load_settings};
use crate::repositories::{save_repositories, load_repositories};
use crate::git_command::get_branch;
use crate::git_command::clone_repo;
use crate::logging::{save_logging, load_logging};

use dotenv_enum::{env_enum, EnvironmentVariable};
use tauri::{Manager};

env_enum!(LocationsEnv, locations_test, [
    Folder,
    SettingsFile,
    CacheFolder,
    LogFile
]);

env_enum!(SettingsEnv, settings_test, [
    Language,
    DebugLevel
]);

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // TODO: Remove this block, it is for debugging
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            save_repositories,
            load_repositories,
            get_branch,
            save_logging,
            load_logging,
            clone_repo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
