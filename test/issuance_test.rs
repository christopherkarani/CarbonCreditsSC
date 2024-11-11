#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

#[test]
fn test_create_credit_success() {
    let env = Env::default();
    env.mock_all_auths();

    let issuer = Address::generate(&env);
    let credit = Credit {
        id: 1,
        issuer: issuer.clone(),
        amount: 100,
        status: CreditStatus::Active,
    };

    // Mock authorization for the issuer
    env.mock_auths(&[MockAuth {
        address: &issuer,
        invoke: &MockAuthInvoke {
            contract: &env.register_contract(None, Issuance),
            fn_name: "create_credit",
            args: (&issuer, &credit).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Attempt to create a credit
    let result = Issuance::create_credit(env.clone(), issuer.clone(), credit.clone());
    assert!(result.is_ok());

    // Verify that the credit was created successfully
    // This would typically involve checking the blockchain state to ensure the credit exists
    // For example, you might check a storage map or event log
}

#[test]
#[should_panic(expected = "UnauthorizedIssuer")]
fn test_create_credit_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let unauthorized_issuer = Address::generate(&env);
    let credit = Credit {
        id: 2,
        issuer: unauthorized_issuer.clone(),
        amount: 100,
        status: CreditStatus::Active,
    };

    // Attempt to create a credit without proper authorization
    let _ = Issuance::create_credit(env.clone(), unauthorized_issuer.clone(), credit.clone());
}

#[test]
#[should_panic(expected = "InvalidCreditData")]
fn test_create_credit_invalid_data() {
    let env = Env::default();
    env.mock_all_auths();

    let issuer = Address::generate(&env);
    let invalid_credit = Credit {
        id: 3,
        issuer: issuer.clone(),
        amount: 0, // Invalid amount
        status: CreditStatus::Active,
    };

    // Mock authorization for the issuer
    env.mock_auths(&[MockAuth {
        address: &issuer,
        invoke: &MockAuthInvoke {
            contract: &env.register_contract(None, Issuance),
            fn_name: "create_credit",
            args: (&issuer, &invalid_credit).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Attempt to create a credit with invalid data
    let _ = Issuance::create_credit(env.clone(), issuer.clone(), invalid_credit.clone());
}
