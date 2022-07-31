use diesel::{Connection, PgConnection};
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    // wrapped in Secrete to hide from logging
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_str(&self) -> String{
        format!("postgres://{}:{}@{}:{}/{}", self.username, self.password.expose_secret(), self.host, self.port, self.database_name)
    }

    pub fn get_connection(&self) -> PgConnection {
        let database_url = self.connection_str();
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed tp determine the current directory.");
    let configuration_directory = base_path.join("configuration");

    // Detect the running env
    // default to `local` if unspecified
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yml")))
        .add_source(config::File::from(configuration_directory.join(&environment_filename)))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{} is not a supported env. Use Either `local` or `production`.", other))
        }
    }
}