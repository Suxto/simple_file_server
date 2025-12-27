use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use crate::model::{Path, config::file_configs::ConfigFromFile};

#[derive(Clone)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
    pub permissions_tree: Path,
    pub permissions: Vec<file_configs::UserPermissionFromFile>,
}

pub struct Config {
    pub users: BTreeMap<String, UserConfig>,
    pub paths: BTreeMap<String, Path>,
}

mod file_configs {
    pub use super::*;
    #[derive(Clone, Deserialize, Serialize)]
    pub struct ConfigFromFile {
        pub users: Vec<UserFromFile>,
        pub paths: Vec<PathFromFile>,
        pub misc: Option<MiscFromFile>,
        pub debug: Option<DebugFromFile>,
    }

    impl ConfigFromFile {
        pub fn to_config(&self) -> Config {
            Config {
                users: self
                    .users
                    .clone()
                    .into_iter()
                    .map(|u| (u.username.clone(), u.to_user_config()))
                    .collect(),
                paths: self
                    .paths
                    .clone()
                    .into_iter()
                    .map(|p| (p.name.clone(), p.to_path_config()))
                    .collect(),
            }
        }

        pub async fn from_toml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let config_str = tokio::fs::read_to_string(path).await?;
            let config_from_file: Self = toml::from_str(&config_str)?;
            Ok(config_from_file)
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct MiscFromFile {
        pub port: Option<u16>,
        pub host: Option<String>,
        pub enable_https: Option<bool>,
        pub cert_path: Option<String>,
        pub log_level: Option<String>,
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct DebugSession{
        pub username: String,
        pub token: String,
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct DebugFromFile {
        pub enable: bool,
        pub debug_session: Option<DebugSession>,
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct UserFromFile {
        pub username: String,
        pub password: String,
        pub permissions: Vec<UserPermissionFromFile>,
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct UserPermissionFromFile {
        pub path_name: String,
        pub permission: u8,
    }

    impl UserFromFile {
        fn to_user_config(self) -> UserConfig {
            UserConfig {
                username: self.username,
                password: self.password,
                permissions_tree: Path {
                    path: "/".to_string(),
                    name: "root".to_string(),
                    permission: 0,
                    sub_path: BTreeMap::new(),
                },
                permissions: self.permissions,
            }
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct PathFromFile {
        pub path: String,
        pub name: String,
        pub permission: u8, // rwv - read, write, view
    }

    impl PathFromFile {
        fn to_path_config(self) -> Path {
            Path {
                path: self.path,
                name: self.name,
                permission: self.permission,
                sub_path: BTreeMap::new(),
            }
        }
    }
}

impl Config {
    pub async fn from_config_file(
        config_from_file: &ConfigFromFile,
    ) -> Result<Config, Box<dyn std::error::Error>> {
        let mut config = config_from_file.to_config();

        config
            .paths
            .values_mut()
            .for_each(|p| p.extract_sub_paths());

        config.users.values_mut().for_each(|u| {
            u.permissions.iter().for_each(|p| {
                if let Some(f) = config.paths.get_mut(&p.path_name) {
                    u.permissions_tree.merge_path(f, p.permission);
                }
            });
        });

        // config.build_permission_tree();
        Ok(config)
    }
}
