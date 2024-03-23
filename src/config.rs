use crate::parser::{LinesTable, Table, Datatype};

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub occurrence: u32,
    pub db_url: String,
    pub table_struct: Table,
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
                ("client_id".to_string(), Datatype::VARCHAR(255))
            ]),
        };
        let sample_config = Configuration {
            occurrence: 10,
            db_url: String::from("mysql://user:password@localhost:3306/default_database"),
            table_struct: sample_table,
        };

        write!(file, "{}", serde_yaml::to_string(&sample_config)?)?;

        Ok(())
    }
}
