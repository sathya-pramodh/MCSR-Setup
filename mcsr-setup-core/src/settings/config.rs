use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

use super::package_manager::PackageManager;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub package_manager: PackageManager,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Field {
    PackageManager,
}

impl Field {
    pub fn get_desc(&self) -> String {
        String::from(match self {
            Field::PackageManager => "Package Manager",
        })
    }
}

impl Config {
    pub fn new(package_manager: PackageManager) -> Self {
        Self { package_manager }
    }

    pub fn get_descriptors(&self) -> HashMap<Field, String> {
        let mut desc = HashMap::new();
        let fields = vec![Field::PackageManager];
        for field in fields {
            match field {
                Field::PackageManager => desc.insert(field, self.package_manager.to_string()),
            };
        }
        desc
    }

    pub fn write(&self, file: String) {
        let contents = match serde_json::to_string(self) {
            Ok(contents) => contents,
            Err(_) => String::new(),
        };
        match fs::write(file, contents) {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = match env::var_os("HOME") {
            Some(dir) => match dir.into_string() {
                Ok(str) => str,
                Err(_) => String::new(),
            },
            None => String::new(),
        };
        let mut conf_dir = match env::var_os("XDG_CONFIG_HOME") {
            Some(dir) => match dir.into_string() {
                Ok(str) => str,
                Err(_) => format!("{}/.config", home),
            },
            None => format!("{}/.config", home),
        };
        if conf_dir == String::new() {
            conf_dir = format!("{}/.config", home);
        }

        let dir_path = format!("{}/mcsr-setup", conf_dir);
        let dir = Path::new(dir_path.as_str());
        let file = dir.join(Path::new("config.json"));
        let file_path = format!("{}/config.json", dir_path);
        let mut conf: Config = Config::new(PackageManager::default());
        if dir.exists() {
            let file = dir.join(Path::new("config.json"));
            if file.exists() {
                let contents = match fs::read_to_string(file) {
                    Ok(contents) => contents,
                    Err(_) => String::new(),
                };
                conf = match serde_json::from_str(contents.as_str()) {
                    Ok(conf) => conf,
                    Err(_) => Config::new(PackageManager::default()),
                };
            }
        } else {
            match fs::create_dir(dir_path.clone()) {
                Ok(_) => (),
                Err(_) => (),
            }

            match File::create_new(file.clone()) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err),
            };
        }

        conf.write(file_path);
        return conf;
    }
}
