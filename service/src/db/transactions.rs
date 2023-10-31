//! Methods processing HTTP requests related to querying transactions.

use serde_derive::{Deserialize, Serialize};

use crate::db::{bigint_to_u64, sql, u64_to_bigint, Database};
use crate::error_codes::Error as ServiceAPIError;

impl Database {
    pub async fn get_tx(&self, limit: usize) -> Result<Vec<Transaction>, ServiceAPIError> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|_| ServiceAPIError::ResourceBusy)?;

        let tx_query_result = client
            .query(sql::SELECT_LATEST_TX, &[&(limit as i64)])
            .await
            .map_err(|_| ServiceAPIError::DatabaseQueryError)?;

        if tx_query_result.is_empty() {
            return Err(ServiceAPIError::DatabaseQueryError);
        }

        let mut txs = Vec::with_capacity(tx_query_result.len());
        for row in tx_query_result {
            txs.push(Transaction {
                from_id: bigint_to_u64(row.get::<_, i64>("from_id")),
                to_id: bigint_to_u64(row.get::<_, i64>("to_id")),
                amount: bigint_to_u64(row.get::<_, i64>("amount")),
            })
        }

        Ok(txs)
    }

    pub async fn post_tx(&self, tx: Transaction) -> Result<String, ServiceAPIError> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|_| ServiceAPIError::ResourceBusy)?;

        // check existence and balance of 1st user
        let from_balance = self
            .check_existence_of_user(tx.from_id)
            .await
            .map_err(|_| ServiceAPIError::SenderDoesNotExist)?;

        // we do not allow the balance to fall down below a certain threshold
        if from_balance < tx.amount + THRESHOLD_BALANCE {
            return Err(ServiceAPIError::NotEnoughBalance);
        }

        // check existence and balance of 2nd user
        if self.check_existence_of_user(tx.to_id).await.is_err() {
            return Err(ServiceAPIError::RecipientDoesNotExist);
        }

        let combined_sql = sql::CREATE_NEW_TX
            .replace("$1", &u64_to_bigint(tx.from_id).to_string())
            .replace("$2", &u64_to_bigint(tx.to_id).to_string())
            .replace("$3", &u64_to_bigint(tx.amount).to_string());

        if client.batch_execute(&combined_sql).await.is_err() {
            return Err(ServiceAPIError::DatabaseQueryError);
        }

        Ok(String::from("Balance transfer completed."))
    }

    pub async fn check_existence_of_user(&self, user_id: u64) -> Result<u64, ServiceAPIError> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|_| ServiceAPIError::ResourceBusy)?;

        // check existence of user and their balance
        let tx_query_result = client
            .query(sql::SELECT_ACCOUNT_INFO_BY_ID, &[&u64_to_bigint(user_id)])
            .await
            .map_err(|_| ServiceAPIError::DatabaseQueryError)?;

        if tx_query_result.is_empty() {
            return Err(ServiceAPIError::DatabaseQueryError);
        }

        Ok(bigint_to_u64(
            tx_query_result[0]
                .get::<_, i64>("balance")
                .try_into()
                .unwrap(),
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub from_id: u64,
    pub to_id: u64,
    pub amount: u64,
}

/// `THRESHOLD_BALANCE` denotes minimum balance of an account
pub const THRESHOLD_BALANCE: u64 = 5;
