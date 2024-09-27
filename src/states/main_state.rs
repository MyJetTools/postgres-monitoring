use std::collections::BTreeMap;
use std::rc::Rc;

use dioxus_utils::js::WebLocalStorage;

use crate::storage_settings::selected_time_zone::SelectedTimeZone;

use crate::{models::*, PgActivityHttpResponse};

pub const ENV_LOCAL_STORAGE_KEY: &str = "env";
use super::DataState;

pub struct MainState {
    pub storage: WebLocalStorage,
    pub envs: Option<Vec<Rc<String>>>,
    pub selected_time_zone: SelectedTimeZone,
    pub server_settings: DataState<Rc<ServerInfoModel>>,
    pub pg_activity: DataState<Rc<PgActivityHttpResponse>>,
    pub pg_data_size: DataState<Rc<BTreeMap<String, Vec<PgDbSizeHttpModel>>>>,
    time_zone: TimeZone,
}

impl MainState {
    pub fn new() -> Self {
        let storage = dioxus_utils::js::GlobalAppSettings::get_local_storage();

        let selected_time_zone = crate::storage_settings::selected_time_zone::get();
        Self {
            envs: None,
            server_settings: DataState::None,
            storage,
            time_zone: TimeZone::default(),
            selected_time_zone,
            pg_activity: DataState::None,
            pg_data_size: DataState::None,
        }
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_environments(&mut self, envs: Vec<String>, time_zone: TimeZone) {
        let envs: Vec<Rc<String>> = envs.into_iter().map(Rc::new).collect();

        self.envs = Some(envs);
        self.time_zone = time_zone;
    }

    pub fn get_selected_timezone(&self) -> TimeZone {
        match self.selected_time_zone {
            SelectedTimeZone::UtcZero => TimeZone::create_utc_zero(),
            SelectedTimeZone::LocalTime => self.time_zone,
        }
    }

    pub fn get_local_timezone(&self) -> TimeZone {
        self.time_zone
    }

    pub fn active_env_changed(&mut self, value: &str) {
        dioxus_utils::js::GlobalAppSettings::get_local_storage().set(ENV_LOCAL_STORAGE_KEY, value);
        self.reset_data();
        self.server_settings = DataState::None;
    }

    pub fn get_selected_env(&self) -> Rc<String> {
        let env = self.storage.get(ENV_LOCAL_STORAGE_KEY).unwrap_or_default();

        for itm in self.envs.as_ref().unwrap() {
            if itm.as_str() == env {
                return itm.clone();
            }
        }

        self.envs.as_ref().unwrap().first().unwrap().clone()
    }

    pub fn reset_data(&mut self) {
        self.pg_activity = DataState::None;
        self.pg_data_size = DataState::None;
    }
}
