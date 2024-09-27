use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbInfoHttpModel {
    pub table_name: String,
    pub table_schema: String,
    pub table_size: i64,
}
