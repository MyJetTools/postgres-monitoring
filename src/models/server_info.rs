use serde::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerInfoModel {
    pub version: String,
    pub hours_to_gc: u32,
}
