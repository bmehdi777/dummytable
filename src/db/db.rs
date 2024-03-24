use crate::config::Configuration;

pub trait Database {
    async fn fill(config: &Configuration) -> Result<(), Box<dyn std::error::Error>>;
    async fn generate_script(config: &Configuration);
}
