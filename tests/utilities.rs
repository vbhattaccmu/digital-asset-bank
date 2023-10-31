use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub(crate) fn get_test_binary_path(
    tests_output_dir_prefix: &str,
    tests_binary_path: &str,
) -> Result<String> {
    let test_binary: PathBuf = [
        PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        PathBuf::from(format!("{}{}", tests_output_dir_prefix, tests_binary_path)),
    ]
    .iter()
    .collect();

    if test_binary.exists() {
        Ok(test_binary.display().to_string())
    } else {
        Err(anyhow!("{} does not exist.", test_binary.display()))
    }
}

pub(crate) fn get_test_config_path(test_dir: &str, test_name: &str) -> String {
    let config_dir_per_test: PathBuf = [PathBuf::from(format!("{}/{}", test_dir, test_name))]
        .iter()
        .collect();

    if !config_dir_per_test.exists() {
        std::fs::create_dir_all(&config_dir_per_test).unwrap();
    }

    config_dir_per_test.display().to_string()
}

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub from_id: u64,
    pub to_id: u64,
    pub amount: u64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub balance: u64,
}
