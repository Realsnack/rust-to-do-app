use std::io::{Read, Write};

use serde_derive::{Deserialize, Serialize};

use crate::supported_persistence::SupportedPersistence;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub persistence: SupportedPersistence,
    pub task_counter: usize,
}

impl Configuration {
    pub fn new(persistence: SupportedPersistence, task_counter: usize) -> Self {
        Configuration {
            persistence,
            task_counter,
        }
    }

    pub fn load_or_create_configuration(name: &str) -> Self {
        let file = std::fs::File::open(name);
        match file {
            Ok(mut file) => {
                let mut encoded_config = Vec::new();
                file.read_to_end(&mut encoded_config).unwrap();
                let config: Configuration = bincode::deserialize(&encoded_config).unwrap();
                println!("Loaded configuration: {:?}", config);
                return config;
            }
            Err(_) => {
                println!("No configuration file found. Creating a new one.");
                let config = Configuration::new(SupportedPersistence::CSV, 0);
                config.save_configuration(name);
                return config;
            }
        }
    }

    pub fn save_configuration(&self, name: &str) {
        let mut file = std::fs::File::create(name).unwrap();
        let encoded_config = bincode::serialize(&self).unwrap();
        file.write_all(&encoded_config).unwrap();
        file.flush().unwrap();
    }
}
