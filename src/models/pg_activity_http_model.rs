use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgActivityHttpModel {
    pub pid: Option<i32>,
    pub username: String,
    pub application_name: String,
    pub client_addr: String,
    pub backend_start: i64,
    pub query_start: i64,
    pub state_change: i64,
    pub state: String,
    pub query: String,
}

impl PgActivityHttpModel {
    pub fn is_active(&self) -> bool {
        self.state == "active"
    }
    pub fn get_backend_start(&self) -> Option<DateTimeAsMicroseconds> {
        if self.backend_start == 0 {
            return None;
        }

        Some(DateTimeAsMicroseconds::new(self.backend_start))
    }

    pub fn get_query_start(&self) -> Option<DateTimeAsMicroseconds> {
        if self.query_start == 0 {
            return None;
        }

        Some(DateTimeAsMicroseconds::new(self.query_start))
    }

    pub fn get_state_change(&self) -> Option<DateTimeAsMicroseconds> {
        if self.state_change == 0 {
            return None;
        }

        Some(DateTimeAsMicroseconds::new(self.state_change))
    }
}
