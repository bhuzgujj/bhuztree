use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait SaveAndLoad<T>
    where T: Serialize + DeserializeOwned
{
    fn save(&self, data: &T, file_name: String) -> Result<(), String>;
    fn load(&self, file_name: String) -> Result<T, String>;
}

pub fn load_error<T>(file: String) -> Result<T, String> {
    Err(format!("File {} not found", file))
}

pub fn save_error<T>(file: String) -> Result<T, String> {
    Err(format!("Cannot save to {}", file))
}
