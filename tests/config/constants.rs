// "common" submodule defines constants that are used by other submodules.
pub(crate) mod common {
    pub(crate) const TEST_DIR: &str = "/tmp/service/test";
    pub(crate) const OUTPUT_DIR: &str = "../target/debug";
    pub(crate) const HOST_URL: &str = "http://localhost";
}

// Service related constants
pub(crate) mod service {
    pub(crate) const LOGS_PATH: &str = "/logs/service_api";
    pub(crate) const CONFIGURATION_PATH: &str = "/service.toml";
    pub(crate) const BINARY_PATH: &str = "/submission";
    pub(crate) const SENDER_DOES_NOT_EXIST: &str =
        "Sender id does not exist on record. Please provide correct ID";
    pub(crate) const RECEIVER_DOES_NOT_EXIST: &str =
        "Receiver id does not exist on record. Please provide correct ID";
    pub(crate) const NOT_ENOUGH_BALANCE: &str = "Sender does not have enough balance to submit this transacion.Minimum balance needs to be 5.";
    pub(crate) const ACCOUNT_EXISTS: &str = "Account exists on DB";
}

// Postgresql.
pub(crate) mod postgresql {
    pub(crate) const USER_NAME: &str = "postgres";
    pub(crate) const PASSWORD: &str = "postgres";
    pub(crate) const HOST_NAME: &str = "localhost";
}
