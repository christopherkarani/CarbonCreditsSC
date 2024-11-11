#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

#[test]
fn test_buy_credit_success() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let order = MarketplaceOrder {
        credit_id: 1,
        seller: seller.clone(),
        buyer: buyer.clone(),
        price: 100,
    };

    // Mock authorization for the buyer
    env.mock_auths(&[MockAuth {
        address: &buyer,
        invoke: &MockAuthInvoke {
            contract: &env.register_contract(None, Marketplace),
            fn_name: "buy_credit",
            args: (&buyer, &order).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Attempt to buy a credit
    let result = Marketplace::buy_credit(env.clone(), buyer.clone(), order.clone());
    assert!(result.is_ok());

    // Verify that the credit was bought successfully
    // This would typically involve checking the blockchain state to ensure the credit ownership has changed
}

#[test]
#[should_panic(expected = "UnauthorizedUser")]
fn test_buy_credit_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let unauthorized_buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let order = MarketplaceOrder {
        credit_id: 2,
        seller: seller.clone(),
        buyer: unauthorized_buyer.clone(),
        price: 100,
    };

    // Attempt to buy a credit without proper authorization
    let _ = Marketplace::buy_credit(env.clone(), unauthorized_buyer.clone(), order.clone());
}

#[test]
#[should_panic(expected = "InvalidTransaction")]
fn test_sell_credit_invalid_transaction() {
    let env = Env::default();
    env.mock_all_auths();

    let seller = Address::generate(&env);
    let invalid_credit = Credit {
        id: 3,
        issuer: seller.clone(),
        amount: 0, // Invalid amount
        status: CreditStatus::Active,
    };

    // Mock authorization for the seller
    env.mock_auths(&[MockAuth {
        address: &seller,
        invoke: &MockAuthInvoke {
            contract: &env.register_contract(None, Marketplace),
            fn_name: "sell_credit",
            args: (&seller, &invalid_credit, &100).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    // Attempt to sell a credit with invalid data
    let _ = Marketplace::sell_credit(env.clone(), seller.clone(), invalid_credit.clone(), 100);
}
