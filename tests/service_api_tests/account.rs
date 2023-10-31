use anyhow::Result;

use crate::config::constants::service;
use crate::config::service::Service;

// Simulate account create success.
#[tokio::test]
async fn test_create_account_success() -> Result<()> {
    // start service binary
    let service = Service::start("test_create_account_success").await;

    // create user 1
    let response = service.create_account(1, 10000).await;
    assert!(response.is_ok());

    // query balance from 1 and check if balance is deducted
    let response = service.query_user(1).await;
    assert!(response.is_ok());
    let user1 = response.unwrap();
    assert_eq!(user1.balance, 10000);

    Ok(())
}

// Simulate non existent account query failure.
#[tokio::test]
async fn test_non_existent_query_failure() -> Result<()> {
    // start service binary
    let service = Service::start("test_non_existent_query_failure").await;

    // query balance from 1 and check if balance is deducted
    let response = service.query_user(1).await;
    assert!(response.is_err());

    let error_response = response.err().as_ref().unwrap().to_string();
    assert_eq!(error_response, service::SENDER_DOES_NOT_EXIST.to_string());

    Ok(())
}

// Simulate non deuplicate account creation failure.
#[tokio::test]
async fn test_duplicate_account_creation_failure() -> Result<()> {
    // start service binary
    let service = Service::start("test_duplicate_account_creation_failure").await;

    // create user 1
    let response = service.create_account(1, 10000).await;
    assert!(response.is_ok());

    // query balance from 1 and check if balance is deducted
    let response = service.query_user(1).await;
    assert!(response.is_ok());
    let user1 = response.unwrap();
    assert_eq!(user1.balance, 10000);

    // create user 1 again
    let response = service.create_account(1, 10000).await;
    assert!(response.is_err());

    let error_response = response.err().as_ref().unwrap().to_string();
    assert_eq!(error_response, service::ACCOUNT_EXISTS.to_string());

    Ok(())
}
