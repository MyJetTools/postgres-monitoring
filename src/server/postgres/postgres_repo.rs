use std::{sync::Arc, time::Duration};

use my_postgres::{macros::*, sql_where::NoneWhereModel, MyPostgres, PostgresSettings};
use my_ssh::SshSessionsPool;
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct PostgresRepo {
    postgres: MyPostgres,
}

impl PostgresRepo {
    pub async fn new(
        postgres_settings: Arc<dyn PostgresSettings + Sync + Send + 'static>,
        ssh_sessions: Arc<SshSessionsPool>,
        private_key: Option<(String, Option<String>)>,
    ) -> Self {
        if let Some((private_key, pass_phrase)) = private_key {
            return Self {
                postgres: MyPostgres::from_settings(
                    super::super::app_ctx::APP_NAME,
                    postgres_settings,
                )
                .with_ssh_sessions(ssh_sessions)
                .with_ssh_private_key(private_key, pass_phrase)
                .build()
                .await,
            };
        }

        Self {
            postgres: MyPostgres::from_settings(super::super::app_ctx::APP_NAME, postgres_settings)
                .with_ssh_sessions(ssh_sessions)
                .build()
                .await,
        }
    }

    pub async fn get_pg_stat_activity(&self) -> Vec<PgActivityEntity> {
        self.postgres
            .with_retries(3, Duration::from_secs(3))
            .query_rows("pg_stat_activity", NoneWhereModel::new())
            .await
            .unwrap()
    }

    pub async fn get_pg_database_sizes(&self) -> Vec<PgDataSizeDbEntity> {
        self.postgres
            .with_retries(3, Duration::from_secs(3))
            .query_rows("pg_database", NoneWhereModel::new())
            .await
            .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, SelectDbEntity)]
pub struct PgActivityEntity {
    pub pid: Option<i32>,
    pub usename: Option<String>,
    pub application_name: Option<String>,
    #[force_cast_db_type]
    pub client_addr: Option<String>,
    #[sql_type("timestamp")]
    pub backend_start: Option<DateTimeAsMicroseconds>,
    #[sql_type("timestamp")]
    pub query_start: Option<DateTimeAsMicroseconds>,
    #[sql_type("timestamp")]
    pub state_change: Option<DateTimeAsMicroseconds>,
    pub state: Option<String>,
    pub query: Option<String>,
}

#[derive(Debug, Clone, PartialEq, SelectDbEntity)]
pub struct PgDataSizeDbEntity {
    pub datname: Option<String>,
    pub datcollversion: Option<String>,

    #[wrap_column_name("pg_database_size(datname) as ${}")]
    pub db_usage: Option<i64>,
}
