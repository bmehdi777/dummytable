use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub occurrence: u32,
    pub path_generation_file: Option<String>,
}

impl Configuration {
    pub fn read_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let config: Configuration = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}
