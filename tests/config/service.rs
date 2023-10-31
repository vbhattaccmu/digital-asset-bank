use anyhow::{anyhow, Result};
use portpicker::pick_unused_port;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use tokio_postgres::NoTls;

use crate::config::constants::common;
use crate::utilities::*;
use crate::{config, utilities};

pub(crate) struct Service {
    pub test_name: String,
    process: std::process::Child,
    port_number: u16,
}

// Config denotes the configuration settings for the integration tests related to the Service API.
#[derive(Deserialize, Serialize)]
struct Config {
    pub db_user: String,
    pub db_user_pw: String,
    pub db_host: String,
    pub db_name: String,
    pub logs_dir: String,
    pub port_number: u16,
}

impl Config {
    fn new_with_unconfigured_ports(
        config_dir_per_test: &str,
        test_name: &str,
        logs_path: &str,
        db_user: &str,
        db_passwd: &str,
        db_host_name: &str,
    ) -> Self {
        Config {
            port_number: 0,
            db_host: db_host_name.to_string(),
            db_user: db_user.to_string(),
            db_user_pw: db_passwd.to_string(),
            db_name: test_name.to_string(),
            logs_dir: format!("{}/{}{}", config_dir_per_test, test_name, logs_path),
        }
    }
}

impl Service {
    pub(crate) async fn start(test_name: &str) -> Service {
        let service_binary = utilities::get_test_binary_path(
            config::common_constants::OUTPUT_DIR,
            config::service_constants::BINARY_PATH,
        )
        .unwrap();

        let config_dir_per_test =
            utilities::get_test_config_path(config::common_constants::TEST_DIR, test_name);

        let client = Self::connect().await.unwrap();
        client
            .execute(&format!("{} {}", "DROP DATABASE IF EXISTS", test_name), &[])
            .await
            .unwrap();
        client
            .execute(&format!("{} {}", "CREATE DATABASE", test_name), &[])
            .await
            .unwrap();

        let mut configuration = Config::new_with_unconfigured_ports(
            config_dir_per_test.as_str(),
            test_name,
            config::service_constants::LOGS_PATH,
            config::postgresql_constants::USER_NAME,
            config::postgresql_constants::PASSWORD,
            config::postgresql_constants::HOST_NAME,
        );

        loop {
            let port_number: u16 = pick_unused_port().expect("No ports free");
            configuration.port_number = port_number;

            std::fs::write(
                format!(
                    "{}{}",
                    &config_dir_per_test,
                    config::service_constants::CONFIGURATION_PATH
                ),
                toml::to_string(&configuration).unwrap(),
            )
            .expect("Unable to generate config json for testing.");

            let process = Command::new(&service_binary)
                .arg(&format!(
                    "--config-path={}{}",
                    config_dir_per_test,
                    config::service_constants::CONFIGURATION_PATH
                ))
                .stdout(Stdio::null())
                .spawn();

            std::thread::sleep(std::time::Duration::from_millis(2000));

            if process.is_ok() {
                return Service {
                    test_name: test_name.to_string(),
                    process: process.unwrap(),
                    port_number,
                };
            }
        }
    }

    pub(crate) async fn drop_database(test_name: &str) -> anyhow::Result<()> {
        let client = Self::connect().await.unwrap();

        match client
            .execute(&format!("{} {}", "DROP DATABASE IF EXISTS", test_name), &[])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    async fn connect() -> anyhow::Result<tokio_postgres::Client> {
        let (client, conn) = tokio_postgres::connect(
            &format!(
                "postgresql://{}:{}@{}",
                config::postgresql_constants::USER_NAME,
                config::postgresql_constants::PASSWORD,
                config::postgresql_constants::HOST_NAME
            ),
            NoTls,
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(client)
    }

    pub(crate) async fn submit_transaction(
        &self,
        from_id: u64,
        to_id: u64,
        amount: u64,
    ) -> Result<String> {
        let api_url = format!("{}:{}", common::HOST_URL, self.port_number);
        let mut map = HashMap::new();
        map.insert("from_id", from_id);
        map.insert("to_id", to_id);
        map.insert("amount", amount);

        let request_path = format!("{}/transactions", &api_url);

        let response = reqwest::Client::new()
            .post(request_path)
            .json(&map)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                return Ok("Success".to_string());
            }
            _ => {
                let body = response.text().await.unwrap();
                return Err(anyhow!(body));
            }
        }
    }

    pub(crate) async fn create_account(&self, id: u64, balance: u64) -> Result<String> {
        let api_url = format!("{}:{}", common::HOST_URL, self.port_number);
        let mut map = HashMap::new();
        map.insert("id", id);
        map.insert("balance", balance);

        let request_path = format!("{}/users", &api_url);

        let response = reqwest::Client::new()
            .post(request_path)
            .json(&map)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                return Ok("Success".to_string());
            }
            _ => {
                let body = response.text().await.unwrap();
                return Err(anyhow!(body));
            }
        }
    }

    // Query user by id.
    pub(crate) async fn query_user(&self, id: u64) -> Result<User> {
        let api_url = format!("{}:{}", common::HOST_URL, self.port_number);
        let request_path = format!("{}/users/{}", &api_url, &id.to_string());

        let response = reqwest::Client::new()
            .get(request_path)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let data = response.text().await.unwrap();

                let result: Vec<User> = serde_json::from_str(&data).unwrap();

                return Ok(result[0].clone());
            }
            _ => {
                let body = response.text().await.unwrap();
                return Err(anyhow!(body));
            }
        }
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        match self.process.kill() {
            Err(e) => println!("Could not kill child process: {}", e),
            Ok(_) => {}
        }

        let test_name = self.test_name.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async {
                Service::drop_database(&test_name).await.unwrap();
            });
        })
        .join()
        .expect("Thread panicked")
    }
}
