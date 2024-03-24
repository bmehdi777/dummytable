use crate::generator::{Datatype, LinesTable, Table};

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub occurrence: u32,
    pub database: DatabaseConfig,
    pub table_struct: Table,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub protocol: String,
    pub user: String,
    pub password: Option<String>,
    pub host: String,
    pub port: u32,
    pub database_name: String,
}
impl DatabaseConfig {
    pub fn to_url(&self) -> String {
        let password = if let Some(pwd) = self.password.clone() {
            format!(":{pwd}")
        } else {
            String::new()
        };
        format!(
            "{}://{}{}@{}:{}/{}",
            self.protocol, self.user, password, self.host, self.port, self.database_name
        )
    }
}

impl Configuration {
    pub fn read_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let config: Configuration = serde_yaml::from_reader(file)?;
        Ok(config)
    }
    pub fn generate(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;

        let sample_table = Table {
            name_table: String::from("EVENT"),
            lines: LinesTable::from([
                ("id".to_string(), Datatype::VARCHAR(255)),
                ("event_time".to_string(), Datatype::INT),
                ("client_id".to_string(), Datatype::VARCHAR(255)),
            ]),
        };
        let sample_config = Configuration {
            occurrence: 10,
            database: DatabaseConfig {
                protocol: String::from("mysql"),
                user: String::from("user"),
                password: Some(String::from("password")),
                host: String::from("localhost"),
                port: 3306,
                database_name: String::from("default_database"),
            },
            table_struct: sample_table,
        };

        write!(file, "{}", serde_yaml::to_string(&sample_config)?)?;

        Ok(())
    }
}
