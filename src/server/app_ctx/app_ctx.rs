use std::{collections::BTreeMap, sync::Arc};

use my_settings_reader::SettingsReader;
use my_ssh::SshSessionsPool;
use tokio::sync::Mutex;

use crate::server::postgres::{DbInfoRepo, PostgresRepo};
use crate::settings_model::SettingsModel;

//pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppContext {
    pub settings_reader: SettingsReader<SettingsModel>,

    pub envs: Mutex<BTreeMap<String, Arc<BTreeMap<String, PostgresRepo>>>>,

    pub ssh_sessions_pool: Arc<SshSessionsPool>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.postgres-monitoring"),
            ssh_sessions_pool: SshSessionsPool::new().into(),
            envs: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn get_postgres_repos(&self, env: &str) -> Arc<BTreeMap<String, PostgresRepo>> {
        let mut envs = self.envs.lock().await;
        if let Some(repo) = envs.get(env) {
            return repo.clone();
        }

        let settings = self.settings_reader.get_settings().await;
        let postgres_settings = settings.get_postgres_settings(env);

        let mut repos = BTreeMap::new();
        for (db_name, postgres_settings) in postgres_settings {
            let repo = PostgresRepo::new(Arc::new(postgres_settings)).await;
            repos.insert(db_name, repo);
        }

        let repos = Arc::new(repos);

        envs.insert(env.to_string(), repos.clone());
        repos
    }

    pub async fn get_db_info(
        &self,
        env: &str,
        db_instance_name: &str,
        db_name: &str,
    ) -> DbInfoRepo {
        let settings = self.settings_reader.get_settings().await;

        let env_conn_string = settings.envs.get(env);

        if env_conn_string.is_none() {
            panic!("Env {} not found", env);
        }

        let conn_strings = env_conn_string.unwrap();

        let conn_string = conn_strings.get(db_instance_name);

        if conn_string.is_none() {
            panic!(
                "Conn string for db_instance {} inside env {} not found",
                env, db_instance_name
            );
        }

        let conn_string = my_postgres::PostgresConnectionString::from_str(conn_string.unwrap());

        let other_env_conn_string = conn_string.to_string_with_new_db_name(APP_NAME, db_name);

        let connection_settings = CustomDbConnection(other_env_conn_string);

        let new_db_repo = DbInfoRepo::new(Arc::new(connection_settings)).await;

        new_db_repo
    }
}

pub struct CustomDbConnection(String);

#[async_trait::async_trait]
impl my_postgres::PostgresSettings for CustomDbConnection {
    async fn get_connection_string(&self) -> String {
        self.0.clone()
    }
}
