use chrono::offset::Local;
use dotenv_enum::EnvironmentVariable;
use crate::LocationsEnv;
use crate::settings::DebugLevel;
use crate::string_file_serializer::{load, save};

#[tauri::command]
pub fn save_logging(logs: String, debug_level: DebugLevel) -> Result<(), String> {
    save(LocationsEnv::LogFile.unwrap_value(), &create_loggable_string(logs, debug_level))
}

#[tauri::command]
pub fn load_logging() -> Result<String, String> {
    load(LocationsEnv::LogFile.unwrap_value())
}

fn create_loggable_string(logs: String, debug_level: DebugLevel) -> String {
    format!(
        "[{}][{}]: {logs}\r",
        Local::now(),
        get_debug_string_from_level(debug_level)
    )
}

fn get_debug_string_from_level(debug_level: DebugLevel) -> String {
    match debug_level {
        DebugLevel::Debug => "DEBUG".to_string(),
        DebugLevel::Warning => "WARNING".to_string(),
        DebugLevel::Error => "ERROR".to_string(),
        _ => "UNKNOWN".to_string()
    }
}