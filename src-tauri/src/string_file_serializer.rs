use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufReader, Read, Write};
use dotenv_enum::EnvironmentVariable;
use crate::LocationsEnv;
use crate::serializer::{SaveAndLoad, save_error, load_error};

#[derive(Debug)]
pub struct StringFileSerializer {
    file: File,
}

impl StringFileSerializer {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl SaveAndLoad<String> for StringFileSerializer {
    fn save(&self, log: &String, file_name: String) -> Result<(), String> {
        self.file.try_clone().ok()
            .map_or_else(|| None, |mut file| file.write_all(log.as_bytes()).ok())
            .map_or_else(|| Err(format!("Failed to deserialize {file_name}")), Ok)
    }

    fn load(&self, file_name: String) -> Result<String, String> {
        let mut string = String::new();
        self.file.try_clone().ok()
            .map_or_else(|| None, |file| BufReader::new(file).read_to_string(&mut string).ok())
            .map_or_else(|| Err(format!("Failed to deserialize {file_name}")), |_| Ok(string))
    }
}

pub fn save(path_from_home: String, data: &String) -> Result<(), String> {
    match dirs::home_dir() {
        None => Err("Cannot find home directory".to_string()),
        Some(home) => {
            let _ = create_dir(home.join(LocationsEnv::Folder.unwrap_value()));
            match OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(home.join(LocationsEnv::Folder.unwrap_value()).join(&path_from_home)) {
                Ok(file) => StringFileSerializer::new(file).save(data, path_from_home),
                Err(_) => save_error(path_from_home)
            }
        }
    }
}

pub fn load(path_from_home: String) -> Result<String, String> {
    match dirs::home_dir() {
        None => Err("Cannot find home directory".to_string()),
        Some(home) => {
            match OpenOptions::new()
                .write(true)
                .open(home.join(LocationsEnv::Folder.unwrap_value()).join(&path_from_home)) {
                Ok(file) => StringFileSerializer::new(file).load(path_from_home),
                Err(_) => {
                    save(path_from_home.clone(), &String::new())?;
                    load_error(path_from_home)
                }
            }
        }
    }
}
