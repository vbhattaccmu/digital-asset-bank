use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Parser, Debug)]
pub struct CLIArguments {
    /// Path to configuration file.
    #[clap(long, value_parser)]
    pub config_path: String,

    /// If set, the service will drop all tables in config.db_path before proceeding with catchup.
    #[clap(long)]
    pub start_anew: bool,
}

/// [Config] defines configuration for this service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // user name for Service DB.
    pub db_user: String,
    // user password for Service DB.
    pub db_user_pw: String,
    // user hostname for Service DB.
    pub db_host: String,
    // database name for Service DB.
    pub db_name: String,
    // directory for storing Service logs.
    pub logs_dir: String,
    // listening port for the Service service.
    pub port_number: u16,
}

pub(crate) fn load_config(config_path: &str) -> std::result::Result<Config, String> {
    match fs::read_to_string(config_path) {
        Ok(file_str) => {
            let ret: Config = match toml::from_str(&file_str) {
                Ok(r) => r,
                Err(_) => return Err("config.toml is not a proper toml file.".to_string()),
            };

            Ok(ret)
        }
        Err(e) => Err(format!(
            "Error: Config file (config.toml) is not found. 
        Please ensure that the configuration directory for Service: \"{}\" exists. ERROR: {:?}",
            config_path, e
        )),
    }
}
