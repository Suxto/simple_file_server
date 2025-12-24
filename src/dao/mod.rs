pub mod app_state;
pub mod config;

pub use config::Config;
pub use app_state::AppState;

pub async fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = tokio::fs::read_to_string("config.toml").await?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
}
