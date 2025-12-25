use crate::model::{Config, Path, UserConfig};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

#[derive(Clone)]
pub struct AppState {
    pub path: Arc<BTreeMap<String, Path>>,
    pub user_config: Arc<BTreeMap<String, UserConfig>>,
    pub user_sessions: Arc<tokio::sync::Mutex<HashMap<String, String>>>,
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}

impl AppState {
    pub fn new_form_config(config: Arc<Config>) -> Self {
        AppState {
            path: Arc::new(config.paths.clone()),
            user_config: Arc::new(config.users.clone()),
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

    pub async fn add_session(&self, session_token: String, username: String) {
        self.user_sessions
            .lock()
            .await
            .insert(session_token, username);
    }
}
