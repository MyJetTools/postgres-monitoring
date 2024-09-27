use std::sync::Arc;

use my_postgres::{
    macros::SelectDbEntity, sql_where::NoneWhereModel, MyPostgres, PostgresSettings,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct PostgresRepo {
    postgres: MyPostgres,
}

impl PostgresRepo {
    pub async fn new(postgres_settings: Arc<dyn PostgresSettings + Sync + Send + 'static>) -> Self {
        Self {
            postgres: MyPostgres::from_settings(super::super::app_ctx::APP_NAME, postgres_settings)
                .build()
                .await,
        }
    }

    pub async fn get_pg_stat_activity(&self) -> Vec<PgActivityEntity> {
        self.postgres
            .query_rows("pg_stat_activity", NoneWhereModel::new())
            .await
            .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, SelectDbEntity)]
pub struct PgActivityEntity {
    pub pid: Option<i32>,
    pub usename: Option<String>,
    pub application_name: Option<String>,
    #[force_cast_to_db_type]
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
