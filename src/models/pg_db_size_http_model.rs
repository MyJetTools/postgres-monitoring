use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgDbSizeHttpModel {
    pub datname: String,
    pub datcollversion: String,
    pub db_usage: i64,
}
