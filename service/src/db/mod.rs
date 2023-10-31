/// Initializes the DB and creates a connection pool with configurable parameters.
pub(crate) mod open;
pub use open::*;

/// Defines methods for querying attributes related to accounts from DB.
pub(crate) mod accounts;
pub use accounts::*;

/// Defines methods for querying attributes related to transactions from DB.
pub(crate) mod transactions;
pub use transactions::*;

/// Defines all SQL queries used to query information from DB.
pub(crate) mod sql;

/// Helpers
pub fn bigint_to_u64(val: i64) -> u64 {
    val as u64
}

pub fn u64_to_bigint(val: u64) -> i64 {
    val as i64
}
