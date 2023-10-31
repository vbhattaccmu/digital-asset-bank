use anyhow::Result;

use crate::config::constants::service;
use crate::config::service::Service;

// Simulate transaction success.
#[tokio::test]
async fn test_submit_tx_success() -> Result<()> {
    // start service binary
    let service = Service::start("test_submit_tx_success").await;

    // create user 1
    let response = service.create_account(1, 10000).await;
    assert!(response.is_ok());

    // create user 2
    let response = service.create_account(2, 10000).await;
    assert!(response.is_ok());

    // submit tx from 1 to 2
    let response = service.submit_transaction(1, 2, 100).await;
    assert!(response.is_ok());

    // query balance from 1 and check if balance is deducted
    let response = service.query_user(1).await;
    assert!(response.is_ok());
    let user1 = response.unwrap();
    assert_eq!(user1.balance, 10000 - 100);

    // query balance from 2 and check if balance is added
    let response = service.query_user(2).await;
    assert!(response.is_ok());
    let user2 = response.unwrap();
    assert_eq!(user2.balance, 10000 + 100);

    Ok(())
}

// Simulate transaction failure due to insufficien balance.
#[tokio::test]
async fn test_submit_tx_failure_insufficient_balance() -> Result<()> {
    // start service binary
    let service = Service::start("test_submit_tx_failure_insufficient_balance").await;

    // create user 1
    let response = service.create_account(1, 10).await;
    assert!(response.is_ok());

    // create user 2
    let response = service.create_account(2, 10000).await;
    assert!(response.is_ok());

    // submit tx from 1 to 2
    let response = service.submit_transaction(1, 2, 100).await;
    assert!(response.is_err());

    let error_response = response.err().as_ref().unwrap().to_string();
    assert_eq!(error_response, service::NOT_ENOUGH_BALANCE.to_string());

    Ok(())
}

// Simulate transaction failure non existent sender.
#[tokio::test]
async fn test_submit_tx_failure_non_existent_sender() -> Result<()> {
    // start service binary
    let service = Service::start("test_submit_tx_failure_non_existent_sender").await;

    // create recipient
    let response = service.create_account(2, 10000).await;
    assert!(response.is_ok());

    // submit tx from 1 to 2
    let response: std::result::Result<String, anyhow::Error> =
        service.submit_transaction(1, 2, 100).await;
    assert!(response.is_err());
    let error_response = response.err().as_ref().unwrap().to_string();
    assert_eq!(error_response, service::SENDER_DOES_NOT_EXIST.to_string());

    Ok(())
}

// Simulate transaction failure non existent recipient.
#[tokio::test]
async fn test_submit_tx_failure_non_existent_recipient() -> Result<()> {
    // start service binary
    let service = Service::start("test_submit_tx_failure_non_existent_recipient").await;

    // create sender
    let response = service.create_account(1, 10000).await;
    assert!(response.is_ok());

    // submit tx from 1 to 2
    let response = service.submit_transaction(1, 2, 100).await;
    assert!(response.is_err());

    // submit tx from 1 to 2
    let response: std::result::Result<String, anyhow::Error> =
        service.submit_transaction(1, 2, 100).await;
    assert!(response.is_err());
    let error_response = response.err().as_ref().unwrap().to_string();
    assert_eq!(error_response, service::RECEIVER_DOES_NOT_EXIST.to_string());

    Ok(())
}
