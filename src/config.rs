use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Stg,
    Prd,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Dev => write!(f, "dev"),
            Environment::Stg => write!(f, "stg"),
            Environment::Prd => write!(f, "prd"),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub name: String,
    pub environment: Environment,
    pub log_level: LogLevel,
    pub app_host: String,
    pub port: u16,
    pub url_prefix: String,
    pub api_url: String,
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_timeout_connection: u64,
    pub db_idle_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::from("gsn_push_processing"),
            environment: Environment::Dev,
            log_level: LogLevel::Info,
            app_host: String::from("127.0.0.1"),
            port: 8080,
            url_prefix: String::from("/api"),
            api_url: String::from("http://localhost:8080"),
            database_url: String::new(),
            db_max_connections: 10,
            db_min_connections: 2,
            db_timeout_connection: 30,
            db_idle_timeout: 600,
        }
    }
}

impl Config {
    /// Load configuration from environment variables, falling back to defaults.
    /// This will attempt to load a `.env` file first if present.
    pub fn from_env() -> Self {
        // Load .env if present, ignore errors
        let _ = dotenvy::dotenv();

        match envy::from_env::<Config>() {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Failed to load config from env ({}). Using defaults.", e);
                Config::default()
            }
        }
    }
}
