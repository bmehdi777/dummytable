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
    CHAR(u32),
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
