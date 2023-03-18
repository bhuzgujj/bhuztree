use std::str::FromStr;
use serde::{Deserialize, Serialize};
use dotenv_enum::EnvironmentVariable;
use crate::{LocationsEnv, SettingsEnv};
use crate::toml_file_serializer::{load, save};
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub enum Language {
    English,
    French
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "French" => Ok(Language::French),
            _ => Ok(Language::English)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub enum DebugLevel {
    Debug = 0,
    Warning = 1,
    Error = 2,
    None = 3
}

impl FromStr for DebugLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Debug" => Ok(DebugLevel::Debug),
            "Error" => Ok(DebugLevel::Error),
            "None" => Ok(DebugLevel::None),
            _ => Ok(DebugLevel::Warning)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/backend/types/")]
pub struct Settings {
    pub language: Language,
    pub debug_level: DebugLevel
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: SettingsEnv::Language.unwrap_casted_value(),
            debug_level: SettingsEnv::DebugLevel.unwrap_casted_value()
        }
    }
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), String> {
    save(LocationsEnv::SettingsFile.unwrap_value(), &settings)
}

#[tauri::command]
pub fn load_settings() -> Result<Settings, String> {
    load(LocationsEnv::SettingsFile.unwrap_value())
}