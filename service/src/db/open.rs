//! Methods processing HTTP requests related to setting up Database.

use mobc::Pool;
use mobc_postgres::{
    tokio_postgres::{self, Config},
    PgConnectionManager,
};
use std::str::FromStr;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<PgConnectionManager<tokio_postgres::NoTls>>,
}

use crate::db::sql;

impl Database {
    pub async fn open(start_anew: bool, config: &str) -> Result<Database, tokio_postgres::Error> {
        let config = Config::from_str(config)?;
        let manager = PgConnectionManager::new(config, tokio_postgres::NoTls);
        let pool = Pool::builder()
            .max_open(MAX_OPEN_CONNECTIONS)
            .max_idle(MAX_IDLE_CONNECTIONS)
            .get_timeout(Some(std::time::Duration::from_secs(15)))
            .build(manager);

        let client = pool
            .get()
            .await
            .expect("Irrecoverable error: Failed to set up connection pool.");

        if start_anew {
            client
                .batch_execute(sql::DROP_ALL_TABLES)
                .await
                .expect("Irrecoverable error: Failed to drop all tables in database.");
        }

        client
            .batch_execute(sql::SETUP_DATABASE)
            .await
            .expect("Irrecoverable error: Failed to set up database.");

        Ok(Database { pool })
    }
}

// `MAX_OPEN_CONNECTIONS` represents the maximum number of open connections to the database.
const MAX_OPEN_CONNECTIONS: u64 = 150;

// `MAX_IDLE_CONNECTIONS` denotes the maximum number of idle connections.
const MAX_IDLE_CONNECTIONS: u64 = 25;
