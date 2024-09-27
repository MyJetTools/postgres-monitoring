use std::{collections::BTreeMap, rc::Rc};

use crate::{models::*, DataState, MainState};
use dioxus::prelude::*;

#[component]
pub fn RenderDbSize() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let mut table_sizes_state = use_signal(|| DbSizeState::new());

    let (env, pg_data_size) = {
        let main_state = main_state.read();
        (
            main_state.get_selected_env(),
            main_state.pg_data_size.clone(),
        )
    };

    let data = match pg_data_size {
        DataState::None => {
            main_state.write().pg_data_size = DataState::Loading;

            let env = env.clone();
            spawn(async move {
                let items = get_pg_db_sizes(env.to_string()).await;

                match items {
                    Ok(items) => {
                        main_state.write().pg_data_size = DataState::Loaded(Rc::new(items));
                    }
                    Err(err) => {
                        main_state.write().pg_data_size = DataState::Err(err.to_string());
                    }
                }
            });
            return rsx! {
                div { "Loading..." }
            };
        }

        DataState::Loading => {
            return rsx! {
                div { "Loading..." }
            }
        }

        DataState::Err(e) => {
            return rsx! {
                h2 { style: "color:red", "Error: {e}" }
            }
        }

        DataState::Loaded(data) => data,
    };

    let mut to_render = Vec::new();

    for (db_instance_name, items) in data.iter() {
        to_render.push(rsx! {
            tr {
                td { colspan: 3,
                    h3 { {db_instance_name.as_str()} }
                }
            }
        });
        for itm in items {
            let (mem_size, color) = format_mem_size(itm.db_usage);

            let expand_data = {
                let read_access = table_sizes_state.read();
                read_access
                    .get_data_state(db_instance_name, &itm.datname)
                    .clone()
            };

            let env = env.clone();

            let db_instance_name = Rc::new(db_instance_name.clone());

            let db_name = Rc::new(itm.datname.clone());

            let icon = if expand_data.is_none() {
                "/img/ico-expand.svg"
            } else {
                "/img/ico-collapse.svg"
            };

            to_render.push(rsx! {

                tr { style: "border-bottom: 1px solid lightgray;",
                    td { style: "width: 20px;",
                        div {
                            style: "padding:0",
                            onclick: move |_| {
                                let env = env.clone();
                                let db_instance_name = db_instance_name.clone();
                                let db_name = db_name.clone();
                                table_sizes_state
                                    .write()
                                    .set_data_loading(db_instance_name.as_str(), db_name.as_str());
                                spawn(async move {
                                    match get_tables_size(
                                            env.to_string(),
                                            db_instance_name.to_string(),
                                            db_name.to_string(),
                                        )
                                        .await
                                    {
                                        Ok(data) => {
                                            table_sizes_state
                                                .write()
                                                .set_data_state(
                                                    db_instance_name.as_str(),
                                                    db_name.as_str(),
                                                    data,
                                                );
                                        }
                                        Err(e) => {
                                            table_sizes_state
                                                .write()
                                                .set_err(
                                                    db_instance_name.as_str(),
                                                    db_name.as_str(),
                                                    e.to_string(),
                                                );
                                        }
                                    }
                                });
                            },
                            img {
                                style: "height:16px; cursor:pointer",
                                src: icon
                            }
                        }
                    }
                    td { {itm.datname.as_str()} }
                    td { style: "color:{color}", {mem_size.as_str()} }
                    td { {itm.datcollversion.as_str()} }
                }
            });

            match expand_data {
                DataState::None => {}
                DataState::Loading => {
                    to_render.push(rsx! {
                        tr {
                            td { colspan: 3,
                                div { "Loading..." }
                            }
                        }
                    });
                }
                DataState::Loaded(data) => {
                    for itm in data.values() {
                        let (table_size, color) = format_mem_size(itm.table_size);
                        to_render.push(rsx! {
                            tr {
                                td {
                                }
                                td { "{itm.table_schema}.{itm.table_name.as_str()}" }

                                td { style: "color:{color}", {table_size} }
                                td {}
                            }
                        });
                    }
                }

                DataState::Err(e) => {
                    to_render.push(rsx! {
                        tr {
                            td { colspan: 3,
                                div { "Error: {e}" }
                            }
                        }
                    });
                }
            }
        }
    }

    rsx! {
        table { class: "table table-striped",

            tr { style: "border-bottom: 1px solid lightgray;",
                th {}
                th { "Db" }
                th { "Size" }
                th { "Collation versions" }
            }

            {to_render.into_iter()}
        }
    }
}

fn format_mem_size(value: i64) -> (String, &'static str) {
    let value = value as f64;
    if value < 1024.0 {
        return (format!("{}", value), "black");
    }

    let value = value / 1024.0;
    if value < 1024.0 {
        return (format!("{:.3} KB", value), "darkgreen");
    }

    let value = value / 1024.0;

    if value < 1024.0 {
        return (format!("{:.3} Mb", value), "green");
    }

    let value = value / 1024.0;

    let color = if value < 1024.0 {
        "darkorange"
    } else {
        "darkred"
    };
    return (format!("{:.3} Gb", value), color);
}

pub struct DbSizeState {
    pub loaded: BTreeMap<String, DataState<Rc<BTreeMap<i64, DbInfoHttpModel>>>>,
}

impl DbSizeState {
    pub fn new() -> Self {
        Self {
            loaded: BTreeMap::new(),
        }
    }

    pub fn set_data_loading(&mut self, db_instance_name: &str, db_name: &str) {
        let key = format!("{db_instance_name}.{db_name}");
        self.loaded.insert(key, DataState::Loading);
    }

    pub fn set_data_state(
        &mut self,
        db_instance_name: &str,
        db_name: &str,
        data: Vec<DbInfoHttpModel>,
    ) {
        let key = format!("{db_instance_name}.{db_name}");
        let mut items = BTreeMap::new();

        for itm in data {
            items.insert(-itm.table_size, itm);
        }
        self.loaded.insert(key, DataState::Loaded(Rc::new(items)));
    }

    pub fn set_err(&mut self, db_instance_name: &str, db_name: &str, err: String) {
        let key = format!("{db_instance_name}.{db_name}");
        self.loaded.insert(key, DataState::Err(err));
    }

    pub fn get_data_state(
        &self,
        db_instance_name: &str,
        db_name: &str,
    ) -> DataState<Rc<BTreeMap<i64, DbInfoHttpModel>>> {
        let key = format!("{db_instance_name}.{db_name}");
        if let Some(data) = self.loaded.get(&key) {
            return data.clone();
        }

        DataState::None
    }
}

#[server]
async fn get_pg_db_sizes(
    env: String,
) -> Result<BTreeMap<String, Vec<PgDbSizeHttpModel>>, ServerFnError> {
    let postgres_repos = crate::server::APP_CTX
        .get_postgres_repos(env.as_str())
        .await;

    let mut data = BTreeMap::new();

    for (db_name, postgres_repo) in postgres_repos.iter() {
        let pg_response = postgres_repo.get_pg_database_sizes().await;

        for entity in pg_response {
            if entity.datname.is_none() {
                continue;
            }

            let data_base_name = entity.datname.unwrap();

            if data_base_name == "template0"
                || data_base_name == "template1"
                || data_base_name == "postgres"
                || data_base_name == "admin"
            {
                continue;
            }

            let http_entity = PgDbSizeHttpModel {
                datname: data_base_name,
                datcollversion: entity.datcollversion.unwrap_or_default(),
                db_usage: entity.db_usage.unwrap_or_default(),
            };

            if !data.contains_key(db_name) {
                data.insert(db_name.to_string(), vec![]);
            }
            data.get_mut(db_name).unwrap().push(http_entity);
        }
    }

    Ok(data)
}

#[server]
async fn get_tables_size(
    env: String,
    db_instance_name: String,
    db_name: String,
) -> Result<Vec<DbInfoHttpModel>, ServerFnError> {
    let postgres_repos = crate::server::APP_CTX
        .get_db_info(env.as_str(), db_instance_name.as_str(), db_name.as_str())
        .await;

    let data = postgres_repos.get_table_sizes().await;

    if let Err(err) = data.as_ref() {
        return Err(ServerFnError::new(format!("{:?}", err)));
    }

    let data = data.unwrap();

    let result: Vec<_> = data
        .into_iter()
        .filter(|itm| {
            if let Some(table_schema) = itm.table_schema.as_ref() {
                table_schema != "pg_catalog" && table_schema != "information_schema"
            } else {
                false
            }
        })
        .map(|itm| DbInfoHttpModel {
            table_name: itm.table_name.unwrap_or_default(),
            table_schema: itm.table_schema.unwrap_or_default(),
            table_size: itm.total_size.unwrap_or_default(),
        })
        .collect();

    Ok(result)
}
