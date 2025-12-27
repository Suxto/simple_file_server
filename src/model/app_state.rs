use crate::model::{Config, ConfigFromFile, Path, UserConfig};
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
    pub async fn new_form_config(config_from_file: &ConfigFromFile) -> Self {
        let config = Config::from_config_file(config_from_file)
            .await
            .unwrap();

        let app_state  = AppState {
            path: Arc::new(config.paths),
            user_config: Arc::new(config.users),
            user_sessions: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        };

        if let Some(debug) = &config_from_file.debug
            && debug.enable
        {
            if let Some(debug_session) = &debug.debug_session
            {
                let username = debug_session.username.clone();
                let session = debug_session.token.clone();
                app_state.add_session(session, username).await;
            }
        }
        app_state
    }

    pub fn get_user_config(&self, username: &str) -> Option<&UserConfig> {
        self.user_config.get(username)
    }

    pub async fn get_user_by_token(&self, session_token: &str) -> Option<&UserConfig> {
        self.get_user_config(self.get_username_by_session(session_token).await?.as_str())
    }

    pub async fn get_username_by_session(&self, session_token: &str) -> Option<String> {
        self.user_sessions.lock().await.get(session_token).cloned()
    }

    pub async fn add_session(& self, session_token: String, username: String) {
        self.user_sessions
            .lock()
            .await
            .insert(session_token, username);
    }
}
