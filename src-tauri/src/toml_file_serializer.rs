use std::collections::HashMap;
use std::{fs};
use std::fs::{create_dir, File};
use std::io::{Read, Write};

use dotenv_enum::EnvironmentVariable;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::LocationsEnv;
use crate::serializer::{SaveAndLoad, load_error, save_error};

#[derive(Debug)]
pub struct TomlFileSerializer {
    file: File,
}

impl TomlFileSerializer {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl<T> SaveAndLoad<T> for TomlFileSerializer
    where T: Serialize + DeserializeOwned + std::fmt::Debug
{
    fn save(&self, data: &T, file_name: String) -> Result<(), String> {
        let mut toml_str: String = String::new();
        toml::to_string(&data).ok()
            .map_or_else(|| None, |toml| {
                toml_str = toml;
                self.file.try_clone().ok()
            })
            .map_or_else(|| None, |mut file| {
                //file_name = file.metadata().ok()?.file_name().into_string().ok()?;
                file.write_all(toml_str.as_bytes()).ok()
            })
            .map_or_else(|| Err(format!("Failed to serialize {file_name} into toml")), |_| Ok(()))
    }

    fn load(&self, file_name: String) -> Result<T, String> {
        let mut tomls = String::new();
        self.file.try_clone().ok()
            .map_or_else(|| None, |mut file| {
                file.read_to_string(&mut tomls).ok()
            })
            .map_or_else(|| None, |_| {
                toml::from_str(&tomls).ok()
            })
            .map_or_else(|| Err(format!("Failed to deserialize {file_name} from toml")), Ok)
    }
}

pub fn save<T: Serialize + DeserializeOwned + std::fmt::Debug>(path_from_home: String, data: &T) -> Result<(), String> {
    match dirs::home_dir() {
        None => Err("Cannot find home directory".to_string()),
        Some(home) => {
            let _ = create_dir(home.join(LocationsEnv::Folder.unwrap_value()));
            match File::create(home.join(LocationsEnv::Folder.unwrap_value()).join(&path_from_home)) {
                Ok(file) => TomlFileSerializer::new(file).save(data, path_from_home),
                Err(_) => save_error(path_from_home)
            }
        }
    }
}

pub fn save_with_folders<T: Serialize + DeserializeOwned + std::fmt::Debug>(
    folders: Vec<String>,
    files: Vec<String>,
    data: &T,
) -> Result<(), String> {
    match dirs::home_dir() {
        None => Err("Cannot find home directory".to_string()),
        Some(home) => {
            let paths = folders.iter().fold(home.join(LocationsEnv::Folder.unwrap_value()), |acc, folder| {
                let _ = create_dir(acc.clone());
                let path = acc.join(format!("{folder}"));
                path
            });
            let _ = create_dir(paths.clone());
            for repo_file in files {
                let file_writing_results = match File::create(&paths.join(&repo_file)) {
                    Ok(file) => {
                        TomlFileSerializer::new(file).save(data, repo_file.clone())
                    }
                    Err(_) => save_error(repo_file.clone()).unwrap()
                };
                if file_writing_results.is_err() {
                    return file_writing_results;
                }
            }
            Ok(())
        }
    }
}

pub fn remove_all_toml(folders: Vec<String>) -> Result<(), String> {
    match dirs::home_dir() {
        None => Err("Cannot find home directory".to_string()),
        Some(home) => {
            let paths = folders.iter()
                .fold(home.join(LocationsEnv::Folder.unwrap_value()), |acc, folder| {
                    acc.join(format!("{folder}"))
                });
            let folder_files = fs::read_dir(paths.clone()).expect("TODO: LOL");
            for file_result in folder_files {
                if let Ok(file_entry) = file_result {
                    if let Ok(file_name) = file_entry.file_name().into_string() {
                        if file_name.ends_with(".toml") {
                            fs::remove_file(file_entry.path()).expect("Failed to remove");
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

pub fn load<T: Serialize + DeserializeOwned + std::fmt::Debug + Default>(path_from_home: String) -> Result<T, String> {
    match dirs::home_dir() {
        None => Err("Cannot find home directory".to_string()),
        Some(home) => {
            match File::open(home.join(LocationsEnv::Folder.unwrap_value()).join(&path_from_home)) {
                Ok(file) => TomlFileSerializer::new(file).load(path_from_home),
                Err(_) => {
                    save(path_from_home.clone(), &T::default())?;
                    load_error(path_from_home)
                }
            }
        }
    }
}

pub fn load_all_files<T: Serialize + DeserializeOwned + std::fmt::Debug + Default>(path_from_home: String) -> Result<HashMap<String, T>, String> {
    match dirs::home_dir() {
        None => Err("Cannot find home directory".to_string()),
        Some(home) => {
            let location = home.join(LocationsEnv::Folder.unwrap_value()).join(&path_from_home);
            let folder_files = fs::read_dir(location.clone()).expect("TODO: LOL");
            let mut results: HashMap<String, T> = HashMap::new();
            for file_result in folder_files {
                if let Ok(file_entry) = file_result {
                    match File::open(&file_entry.path()) {
                        Ok(file) => {
                            let name = file_entry.file_name().into_string().unwrap();
                            let toml: Result<T, String> = TomlFileSerializer::new(file).load(name.clone());
                            if toml.is_ok() {
                                results.insert(name.as_str()[0..name.len() - ".toml".len()].to_string().clone(), toml.unwrap());
                            }
                        }
                        Err(_) => {
                            let _ = load_error::<String>(file_entry.file_name().into_string().unwrap());
                        }
                    };
                }
            }
            return Ok(results);
        }
    }
}
