use std::collections::{BTreeMap, HashMap};

use my_ssh::SshCredentialsSettingsModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, BTreeMap<String, String>>,
    pub ssh_credentials: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

impl SettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().cloned().collect()
    }

    pub fn get_postgres_settings(&self, env: &str) -> BTreeMap<String, AppPostgresSettings> {
        let result = self
            .envs
            .get(env)
            .expect(format!("Can not find env {}", env).as_str());

        result
            .iter()
            .map(|(db_name, con_string)| (db_name.clone(), AppPostgresSettings(con_string.clone())))
            .collect()
    }

    pub async fn get_ssh_private_key(&self, env: &str) -> Option<(String, Option<String>)> {
        if self.ssh_credentials.is_none() {
            println!("Ssh private keys are not set");
        }
        let ssh_credentials = self.ssh_credentials.as_ref()?;

        if let Some(itm) = ssh_credentials.get(env) {
            let file_path = rust_extensions::file_utils::format_path(itm.cert_path.as_str());

            let cert_content = tokio::fs::read_to_string(file_path.as_str()).await;

            if let Err(err) = &cert_content {
                panic!(
                    "Can not read cert file: {}. Err: {:?}",
                    file_path.as_str(),
                    err
                );
            }

            return Some((cert_content.unwrap(), Some(itm.cert_pass_prase.clone())));
        }

        let itm = ssh_credentials.get("*")?;

        let file_path = rust_extensions::file_utils::format_path(itm.cert_path.as_str());

        let cert_content = tokio::fs::read_to_string(file_path.as_str()).await;

        if let Err(err) = &cert_content {
            panic!(
                "Can not read cert file: {}. Err: {:?}",
                file_path.as_str(),
                err
            );
        }

        println!("Loaded default ssh credentials for env: {}", env);

        Some((cert_content.unwrap(), Some(itm.cert_pass_prase.clone())))
    }
}

pub struct AppPostgresSettings(String);

#[async_trait::async_trait]
impl my_postgres::PostgresSettings for AppPostgresSettings {
    async fn get_connection_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::SettingsModel;

    #[test]
    fn test() {
        let mut envs = BTreeMap::new();

        let mut postgres = BTreeMap::new();
        postgres.insert("db1".to_string(), "postgres://localhost:5432".to_string());
        envs.insert("env1".to_string(), postgres);

        let set = SettingsModel {
            envs,
            ssh_credentials: None,
        };

        let result = serde_yaml::to_string(&set).unwrap();

        println!("{}", result);
    }
}
