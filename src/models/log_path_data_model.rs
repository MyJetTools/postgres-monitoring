use rust_extensions::base64::{FromBase64, IntoBase64};

use crate::storage_settings::log_level::SelectedLevel;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPathDataModel {
    #[prost(uint32, tag = "1")]
    pub level: u32,
    #[prost(bool, tag = "2")]
    pub is_ctx_search: bool,
    #[prost(string, tag = "3")]
    pub time_range: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub search_string: ::prost::alloc::string::String,
}

impl LogPathDataModel {
    pub fn get_log_level(&self) -> SelectedLevel {
        match self.level {
            0 => SelectedLevel::Error,
            1 => SelectedLevel::FatalError,
            2 => SelectedLevel::Warning,
            3 => SelectedLevel::Info,
            4 => SelectedLevel::Debug,
            _ => SelectedLevel::All,
        }
    }

    pub fn from_base_64(src: &str) -> Option<Self> {
        let data = src.from_base64().ok()?;
        let model: Self = prost::Message::decode(data.as_slice()).ok()?;
        Some(model)
    }

    pub fn to_base_64(&self) -> String {
        let mut data = Vec::new();
        prost::Message::encode(self, &mut data).unwrap();
        data.into_base64()
    }
}

impl Into<u32> for SelectedLevel {
    fn into(self) -> u32 {
        match self {
            SelectedLevel::Error => 0,
            SelectedLevel::FatalError => 1,
            SelectedLevel::Warning => 2,
            SelectedLevel::Info => 3,
            SelectedLevel::Debug => 4,
            SelectedLevel::All => 100,
        }
    }
}
