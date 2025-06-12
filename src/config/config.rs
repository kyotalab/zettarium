use config::{Config, ConfigError, File};
use etcetera::{BaseStrategy, choose_base_strategy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub paths: PathsConfig,
    pub editor: EditorConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PathsConfig {
    pub db_path: String,
    pub zettel_dir: String,
    pub archive_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditorConfig {
    pub editor: String,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let strategy = choose_base_strategy().expect("Unable to find the config directory!");
    let mut path = strategy.config_dir();
    path.push("zettarium");
    path.push("config.toml");

    if !path.exists() {
        eprintln!("No config file found at: {}", path.display());
    }

    let builder = Config::builder().add_source(File::from(path));

    builder.build()?.try_deserialize()
}
