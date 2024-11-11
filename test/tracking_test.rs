#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

#[test]
fn test_retire_credit_success() {
    let env = Env::default();
    env.mock_all_auths();

    let credit_id = 1;

    // Mock the environment to simulate retiring a credit
    let result = Tracking::retire_credit(env.clone(), credit_id);
    assert!(result.is_ok());

    // Verify that the credit was retired successfully
    // This would typically involve checking the blockchain state to ensure the credit status is updated
}

#[test]
#[should_panic(expected = "CreditNotFound")]
fn test_retire_credit_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let non_existent_credit_id = 999;

    // Attempt to retire a non-existent credit
    let _ = Tracking::retire_credit(env.clone(), non_existent_credit_id);
}

#[test]
#[should_panic(expected = "AlreadyRetired")]
fn test_retire_credit_already_retired() {
    let env = Env::default();
    env.mock_all_auths();

    let credit_id = 2;

    // Mock retiring the credit once
    let _ = Tracking::retire_credit(env.clone(), credit_id);

    // Attempt to retire the same credit again
    let _ = Tracking::retire_credit(env.clone(), credit_id);
}

#[test]
fn test_get_credit_history_success() {
    let env = Env::default();
    env.mock_all_auths();

    let credit_id = 1;

    // Mock the environment to simulate retrieving credit history
    let result = Tracking::get_credit_history(env.clone(), credit_id);
    assert!(result.is_ok());

    // Verify that the credit history is retrieved successfully
    // This would typically involve checking the returned data for accuracy
}
