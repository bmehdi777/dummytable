use crate::config::Configuration;
use crate::db::db::Database;

use mysql_async::prelude::*;

pub struct MariaDB {}

impl Database for MariaDB {
    async fn fill(config: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
        let database_url = config.database.to_url();
        let pool = mysql_async::Pool::new(database_url.as_str());

        let mut connection = pool.get_conn().await?;

        let table_name = &config.table_struct.name_table;
        let mut table_field = String::new();
        let mut table_field_value = String::new();


        let mut lines_iter = config.table_struct.lines.iter().peekable();

        if let Some(line) = lines_iter.next() {
            let key = line.0.to_string();
            table_field.push_str(&key);
            table_field_value.push_str(&format!(":{}", &key));
        } else {
            return Err("No lines provided".into());
        }

        while let Some(line) = lines_iter.next() {
            let key = line.0.to_string();
            table_field.push_str(&format!(",{}", key));
            table_field_value.push_str(&format!(",:{}", key));

        }

        let test = params! { "customer_id" => "test"};

        connection
            .exec_batch(
                format!(
                    r"INSERT INTO {table_name} ({}) VALUES ({})",
                    table_field, table_field_value
                ),
                params,
            )
            .await?;

        Ok(())
    }
    async fn generate_script(config: &Configuration) {}
}
