use std::{sync::Arc, time::Duration};

use macros::*;
use my_postgres::*;
use sql_where::StaticLineWhereModel;

pub struct DbInfoRepo {
    postgres: MyPostgres,
}

impl DbInfoRepo {
    pub async fn new(postgres_settings: Arc<dyn PostgresSettings + Sync + Send + 'static>) -> Self {
        Self {
            postgres: MyPostgres::from_settings(super::super::app_ctx::APP_NAME, postgres_settings)
                .set_sql_request_timeout(Duration::from_secs(3))
                .build()
                .await,
        }
    }

    pub async fn get_table_sizes(&self) -> Result<Vec<DbInfoEntity>, MyPostgresError> {
        let where_model = StaticLineWhereModel::new("NOT starts_with(table_name, '_')");
        self.postgres
            //.with_retries(3, Duration::from_secs(3))
            .query_rows("information_schema.tables", Some(&where_model))
            .await
    }
}

#[derive(Debug, Clone, PartialEq, SelectDbEntity)]
pub struct DbInfoEntity {
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    #[wrap_column_name("pg_total_relation_size(table_schema || '.' || table_name) AS  ${}")]
    pub total_size: Option<i64>,
}
