#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

#[test]
fn test_verify_project_success() {
    let env = Env::default();
    env.mock_all_auths();

    let verifier = Address::generate(&env);
    let verification_data = VerificationData {
        project_id: 1,
        verifier: verifier.clone(),
        verified_amount: 100,
    };

    // Mock authorization for the verifier
    env.mock_auths(&[MockAuth {
        address: &verifier,
        invoke: &MockAuthInvoke {
            contract: &env.register_contract(None, Verification),
            fn_name: "verify_project",
            args: (&verifier, &verification_data).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Attempt to verify a project
    let result = Verification::verify_project(env.clone(), verifier.clone(), verification_data.clone());
    assert!(result.is_ok());

    // Verify that the project was verified successfully
    // This would typically involve checking the blockchain state to ensure the verification data is recorded
}

#[test]
#[should_panic(expected = "UnauthorizedVerifier")]
fn test_verify_project_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let unauthorized_verifier = Address::generate(&env);
    let verification_data = VerificationData {
        project_id: 2,
        verifier: unauthorized_verifier.clone(),
        verified_amount: 100,
    };

    // Attempt to verify a project without proper authorization
    let _ = Verification::verify_project(env.clone(), unauthorized_verifier.clone(), verification_data.clone());
}

#[test]
#[should_panic(expected = "InvalidVerificationData")]
fn test_verify_project_invalid_data() {
    let env = Env::default();
    env.mock_all_auths();

    let verifier = Address::generate(&env);
    let invalid_verification_data = VerificationData {
        project_id: 3,
        verifier: verifier.clone(),
        verified_amount: 0, // Invalid amount
    };

    // Mock authorization for the verifier
    env.mock_auths(&[MockAuth {
        address: &verifier,
        invoke: &MockAuthInvoke {
            contract: &env.register_contract(None, Verification),
            fn_name: "verify_project",
            args: (&verifier, &invalid_verification_data).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Attempt to verify a project with invalid data
    let _ = Verification::verify_project(env.clone(), verifier.clone(), invalid_verification_data.clone());
}
