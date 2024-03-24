use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Table {
    pub name_table: String,
    pub lines: LinesTable,
}

pub type LinesTable = HashMap<String, Datatype>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Datatype {
    // Text type
    CHAR,
    VARCHAR(u32),
    BINARY(u32),
    VARBINARY(u32),
    TINYBLOB,
    TINYTEXT,
    TEXT(u32),
    BLOB(u32),
    MEDIUMTEXT,
    MEDIUMBLOB,
    LONGTEXT,
    LONGBLOB,

    // Numeric type
    BIT,
    TINYINT,
    BOOL,
    BOOLEAN,
    SMALLINT,
    MEDIUMINT,
    INT,
    BIGINT,
    FLOAT,
    DOUBLE,
    DECIMAL,
    DEC,

    // Date & time type
    DATE,
    DATETIME(String),
    TIMESTAMP(String),
    TIME(String),
    YEAR,
}

impl Datatype {
    pub fn generate(&self) -> String {
        match &self {
            Datatype::CHAR => todo!(),
            Datatype::VARCHAR(size) => todo!(),
            Datatype::BINARY(size) => todo!(),
            Datatype::VARBINARY(size) => todo!(),
            Datatype::TINYBLOB => todo!(),
            Datatype::TINYTEXT => todo!(),
            Datatype::TEXT(size) => todo!(),
            Datatype::BLOB(size) => todo!(),
            Datatype::MEDIUMTEXT => todo!(),
            Datatype::MEDIUMBLOB => todo!(),
            Datatype::LONGTEXT => todo!(),
            Datatype::LONGBLOB => todo!(),
            Datatype::BIT => todo!(),
            Datatype::TINYINT => todo!(),
            Datatype::BOOL | Datatype::BOOLEAN => todo!(),
            Datatype::SMALLINT => todo!(),
            Datatype::MEDIUMINT => todo!(),
            Datatype::INT => todo!(),
            Datatype::BIGINT => todo!(),
            Datatype::FLOAT => todo!(),
            Datatype::DOUBLE => todo!(),
            Datatype::DECIMAL | Datatype::DEC => todo!(),
            Datatype::DATE => todo!(),
            Datatype::DATETIME(format) => todo!(),
            Datatype::TIMESTAMP(format) => todo!(),
            Datatype::TIME(format) => todo!(),
            Datatype::YEAR => todo!()
         }
    }
}
