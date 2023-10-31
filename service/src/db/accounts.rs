//! Helpers processing HTTP requests related to accounts.

use serde_derive::{Deserialize, Serialize};

use crate::db::{bigint_to_u64, sql, u64_to_bigint, Database};
use crate::error_codes::Error as ServiceAPIError;

impl Database {
    pub async fn get_account_info(&self, id: Option<u64>) -> Result<Vec<User>, ServiceAPIError> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|_| ServiceAPIError::ResourceBusy)?;

        let mut users = Vec::new();

        let account_query_result = if id.is_some() {
            client
                .query(
                    sql::SELECT_ACCOUNT_INFO_BY_ID,
                    &[&u64_to_bigint(id.unwrap())],
                )
                .await
                .map_err(|_| ServiceAPIError::DatabaseQueryError)?
        } else {
            client
                .query(sql::SELECT_ACCOUNT_INFO_BY_ID, &[&Option::<i64>::None])
                .await
                .map_err(|_| ServiceAPIError::DatabaseQueryError)?
        };

        if account_query_result.is_empty() {
            return Err(ServiceAPIError::SenderDoesNotExist);
        }

        for row in account_query_result {
            users.push(User {
                id: bigint_to_u64(row.get::<_, i64>("id")),
                balance: bigint_to_u64(row.get::<_, i64>("balance")),
            })
        }

        Ok(users)
    }

    pub async fn create_account(&self, user: User) -> Result<String, ServiceAPIError> {
        let mut client = self
            .pool
            .get()
            .await
            .map_err(|_| ServiceAPIError::ResourceBusy)?;
        let db_transaction = client
            .transaction()
            .await
            .map_err(|_| ServiceAPIError::DatabaseQueryError)?;

        db_transaction
            .execute(
                sql::CREATE_NEW_USER,
                &[&u64_to_bigint(user.id), &u64_to_bigint(user.balance)],
            )
            .await
            .map_err(|_| ServiceAPIError::AccountExists)?;
        db_transaction
            .commit()
            .await
            .map_err(|_| ServiceAPIError::ResourceBusy)?;

        Ok(String::from("Account created successfully"))
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub balance: u64,
}
