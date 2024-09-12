use config::Environment;
use serde::Deserialize;

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let configuration_directory = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("config.toml"),
        ))
        .add_source(
            Environment::with_prefix("FILECRAB")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: Application,
    pub localization: Localization,
    pub database: Database,
}

#[derive(Deserialize, Clone)]
pub struct Application {
    pub host: String,
    pub port: u16,
    pub date_format: String,
}

#[derive(Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct Localization {
    pub default: String,
    pub method: Vec<LocalizationSortMethod>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LocalizationSortMethod {
    Header,
}
