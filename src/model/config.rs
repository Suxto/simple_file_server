use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::model::Path;

struct UserConfig {
    pub username: String,
    pub password: String,
    pub permissions_tree: Path,
}

#[derive(Clone, Deserialize, Serialize)]
struct ConfigFromFile {
    pub users: Vec<UserFromFile>,
    pub paths: Vec<PathFromFile>,
}

impl ConfigFromFile {
    fn to_config(self) -> Config {
        Config {
            users: self.users.into_iter().map(|u| u.to_user_config()).collect(),
            paths: self.paths.into_iter().map(|p| p.to_path_config()).collect(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct UserFromFile {
    pub username: String,
    pub password: String,
    pub permissions: Vec<UserPermissionFromFile>,
}

#[derive(Clone, Deserialize, Serialize)]
struct UserPermissionFromFile {
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
                sub_path: HashSet::new(),
            },
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
            sub_path: HashSet::new(),
        }
    }
}

struct Config {
    pub users: Vec<UserConfig>,
    pub paths: Vec<Path>,
}

impl Config {
    pub async fn from_toml(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config_str = tokio::fs::read_to_string(path).await?;
        let config_from_file: ConfigFromFile = toml::from_str(&config_str)?;
        let mut config = config_from_file.to_config();
        config.build_permission_tree();
        Ok(config)
    }

    fn build_permission_tree(&mut self) {
        self.users.iter_mut().for_each(|user| {
            user.permissions.iter().fold(
                &Path {
                    path: "/".to_string(),
                    name: "root".to_string(),
                    permission: 0,
                    sub_path: HashSet::new(),
                },
                |mut acc, path| {
                    let mut current_path = acc;
                    path.name.split("/").for_each(|elem| {
                        current_path = &Path {
                    });
                    acc
                },
            );
        });
    }
}