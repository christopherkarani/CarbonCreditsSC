#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

#[test]
fn test_check_compliance_success() {
    let env = Env::default();
    env.mock_all_auths();

    let compliance_rule = ComplianceRule {
        rule_id: 1,
        description: Symbol::from_str("Rule 1"),
    };

    // Mock the environment to simulate checking compliance
    let result = Compliance::check_compliance(env.clone(), compliance_rule);
    assert!(result.is_ok());

    // Verify that compliance was checked successfully
    // This would typically involve checking the blockchain state or logs
}

#[test]
#[should_panic(expected = "NonCompliance")]
fn test_check_compliance_failure() {
    let env = Env::default();
    env.mock_all_auths();

    let non_compliant_rule = ComplianceRule {
        rule_id: 2,
        description: Symbol::from_str("Non-compliant Rule"),
    };

    // Attempt to check compliance with a non-compliant rule
    let _ = Compliance::check_compliance(env.clone(), non_compliant_rule);
}

#[test]
fn test_enforce_security_success() {
    let env = Env::default();
    env.mock_all_auths();

    let security_measure = SecurityMeasure {
        measure_id: 1,
        description: Symbol::from_str("Security Measure 1"),
    };

    // Mock the environment to simulate enforcing security
    let result = Compliance::enforce_security(env.clone(), security_measure);
    assert!(result.is_ok());

    // Verify that security was enforced successfully
    // This would typically involve checking the blockchain state or logs
}

#[test]
#[should_panic(expected = "SecurityBreach")]
fn test_enforce_security_failure() {
    let env = Env::default();
    env.mock_all_auths();

    let insecure_measure = SecurityMeasure {
        measure_id: 2,
        description: Symbol::from_str("Insecure Measure"),
    };

    // Attempt to enforce an insecure measure
    let _ = Compliance::enforce_security(env.clone(), insecure_measure);
}
