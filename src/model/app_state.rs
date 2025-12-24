use crate::model::{Config, Path, UserConfig};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub path: Arc<Vec<Path>>,
    pub user_config: Arc<HashMap<String, UserConfig>>,
    pub user_sessions: Arc<tokio::sync::Mutex<HashMap<String, String>>>,
}

impl AppState {
    pub fn new_form_config(config: Arc<Config>) -> Self {
        AppState {
            path: Arc::new(
                config
                    .paths
                    .iter()
                    .map(|pc| Path::from(pc.clone()))
                    .collect(),
            ),
            user_config: Arc::new(
                config
                    .users
                    .iter()
                    .map(|u| (u.username.clone(), u.clone()))
                    .collect(),
            ),
            user_sessions: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    pub fn get_user_config(&self, username: &str) -> Option<&UserConfig> {
        self.user_config.get(username)
    }

    pub async fn get_user_by_session(&self, session_token: &str) -> Option<&UserConfig> {
        self.user_sessions
            .lock()
            .await
            .get(session_token)
            .and_then(|username| self.get_user_config(username))
    }
}
