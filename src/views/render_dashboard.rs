use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use crate::{models::*, DataState, MainState};
use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

#[component]
pub fn RenderDashboard() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();
    let (env, pg_activity_data) = {
        let main_state = main_state.read();
        (
            main_state.get_selected_env(),
            main_state.pg_activity.clone(),
        )
    };

    let mut state = use_signal(|| DashBoardState::new());

    let state_data = { state.read().clone() };

    let data = match pg_activity_data {
        DataState::None => {
            main_state.write().pg_activity = DataState::Loading;

            spawn(async move {
                let items = get_pg_activity(env.to_string()).await;

                match items {
                    Ok(items) => {
                        main_state.write().pg_activity = DataState::Loaded(Rc::new(items));
                    }
                    Err(err) => {
                        main_state.write().pg_activity = DataState::Err(err.to_string());
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

        DataState::Loaded(data) => data,

        DataState::Err(message) => {
            return rsx! {
                h1 { style: "color:red", "Error: {message}" }
            }
        }
    };

    let now = DateTimeAsMicroseconds::new(data.now);

    let mut to_render = Vec::new();
    for (db_name, items) in data.data.iter() {
        to_render.push(rsx! {
            tr {
                td { colspan: 6,
                    h3 { {db_name.as_str()} }
                }
            }
        });
        let mut active_to_render = Vec::new();
        let mut idle_to_render = Vec::new();
        let db_name_expand = db_name.clone();

        let idle_expanded = state_data.is_expanded(db_name);

        if idle_expanded {
            idle_to_render.push(rsx! {
                tr {
                    td { colspan: 7,

                        div {
                            style: "padding: 5px;  background: #f0f0f0",
                            onclick: move |_| {
                                state.write().hide(&db_name_expand);
                            },
                            img {
                                class: "expand-collapse-ico",
                                src: "/img/ico-collapse.svg"
                            }
                        }
                    }
                }
            });
        } else {
            idle_to_render.push(rsx! {
                tr {
                    td { colspan: 7,

                        div { style: "padding: 5px; background: #f0f0f0",
                            img {
                                onclick: move |_| {
                                    state.write().expand(db_name_expand.to_string());
                                },
                                class: "expand-collapse-ico",
                                src: "/img/ico-expand.svg"
                            }
                            "Expand not Active"
                        }
                    }
                }
            });
        }
        for itm in items {
            if !itm.is_active() && !idle_expanded {
                continue;
            }

            let pid = if itm.pid.is_some() {
                itm.pid.unwrap().to_string()
            } else {
                String::new()
            };

            let (be_start, be_since) = if let Some(value) = itm.get_backend_start() {
                let mut result = value.to_rfc3339();
                result.truncate(26);
                (result, now.duration_since(value).to_string())
            } else {
                (String::new(), String::new())
            };

            let (q_start, q_since) = if let Some(value) = itm.get_query_start() {
                let mut result = value.to_rfc3339();
                result.truncate(26);
                (result, now.duration_since(value).to_string())
            } else {
                (String::new(), String::new())
            };

            let (state_change, duration) = if let Some(value) = itm.get_state_change() {
                let mut result = value.to_rfc3339();
                result.truncate(26);

                let duration = if let Some(query_start) = itm.get_query_start() {
                    value.duration_since(query_start).to_string()
                } else {
                    String::new()
                };

                (result, duration)
            } else {
                (String::new(), String::new())
            };

            let color = if itm.is_active() {
                "background:#deffde"
            } else {
                ""
            };

            let item = rsx! {
                tr { style: "border-bottom: 1px solid #ccc; {color}",
                    td { {pid} }
                    td { style: "border-left: 1px solid #ccc",
                        div { style: "padding-bottom: 2px;padding-top: 2px;font-weight: 700;",
                            {itm.application_name.as_str()}
                        }
                        div { style: "padding-bottom: 2px;padding-top: 2px;", {itm.username.as_str()} }
                        div { style: "padding-bottom: 2px;padding-top: 2px;",
                            {itm.client_addr.as_str()}
                        }
                    }

                    td { style: "border-left: 1px solid #ccc",
                        div { style: "font-size: 12px;", {be_start} }
                        div { {be_since} }
                    }
                    td { style: "border-left: 1px solid #ccc",
                        div { style: "font-size: 12px;", {q_start} }
                        div { {q_since} }
                    }
                    td { style: "border-left: 1px solid #ccc",
                        div { style: "font-size: 12px;", {state_change} }
                        div { {duration} }
                    }
                    td { style: "border-left: 1px solid #ccc", {itm.state.as_str()} }
                    td { style: "border-left: 1px solid #ccc;",
                        div { style: "overflow-y: auto;width: 400px;max-height: 100px;",
                            {itm.query.as_str()}
                        }
                    }
                }
            };

            if itm.is_active() {
                active_to_render.push(item);
            } else {
                idle_to_render.push(item);
            }
        }

        to_render.push(rsx! {
            {active_to_render.into_iter()},
            {idle_to_render.into_iter()}
        });
    }

    rsx! {

        table { class: "table table-striped",

            tr {
                th { "PID" }
                th { "Application" }
                th { "Started" }
                th { "Query started" }
                th { "State changed" }
                th { "State" }
                th { "Query" }
            }

            {to_render.into_iter()}
        }
    }
}

#[derive(Debug, Clone)]
pub struct DashBoardState {
    expanded: HashMap<String, ()>,
}

impl DashBoardState {
    pub fn new() -> Self {
        Self {
            expanded: HashMap::new(),
        }
    }

    pub fn is_expanded(&self, db_name: &str) -> bool {
        self.expanded.contains_key(db_name)
    }

    pub fn expand(&mut self, db_name: String) {
        self.expanded.insert(db_name, ());
    }

    pub fn hide(&mut self, db_name: &str) {
        self.expanded.remove(db_name);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgActivityHttpResponse {
    now: i64,
    data: BTreeMap<String, Vec<PgActivityHttpModel>>,
}

#[server]
async fn get_pg_activity(env: String) -> Result<PgActivityHttpResponse, ServerFnError> {
    let postgres_repos = crate::server::APP_CTX
        .get_postgres_repos(env.as_str())
        .await;

    let mut data = BTreeMap::new();

    for (db_name, postgres_repo) in postgres_repos.iter() {
        let pg_response = postgres_repo.get_pg_stat_activity().await;

        for entity in pg_response {
            let http_entity = PgActivityHttpModel {
                pid: entity.pid,
                username: entity.usename.unwrap_or_default(),
                application_name: entity.application_name.unwrap_or_default(),
                client_addr: entity.client_addr.unwrap_or_default(),
                backend_start: if let Some(value) = entity.backend_start {
                    value.unix_microseconds
                } else {
                    0
                },
                query_start: if let Some(value) = entity.query_start {
                    value.unix_microseconds
                } else {
                    0
                },
                state_change: if let Some(value) = entity.state_change {
                    value.unix_microseconds
                } else {
                    0
                },
                state: entity.state.unwrap_or_default(),
                query: entity.query.unwrap_or_default(),
            };

            if !data.contains_key(db_name) {
                data.insert(db_name.to_string(), vec![]);
            }
            data.get_mut(db_name).unwrap().push(http_entity);
        }
    }

    Ok(PgActivityHttpResponse {
        now: DateTimeAsMicroseconds::now().unix_microseconds,
        data,
    })
}
