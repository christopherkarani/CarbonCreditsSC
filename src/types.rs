#![no_std]

use soroban_sdk::{contracttype, Address, BytesN, Env, Symbol, Vec};

/// Struct representing a carbon credit.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credit {
    pub id: u64,
    pub issuer: Address,
    pub amount: u32,
    pub status: CreditStatus,
}

/// Enum representing the status of a carbon credit.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreditStatus {
    Active,
    Retired,
}

/// Struct representing an issuer of carbon credits.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Issuer {
    pub address: Address,
    pub name: Symbol,
}

/// Struct representing a verifier for emissions reductions.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verifier {
    pub address: Address,
    pub name: Symbol,
}

/// Struct representing verification data for a project.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationData {
    pub project_id: u64,
    pub verifier: Address,
    pub verified_amount: u32,
}

/// Struct representing a marketplace order for trading credits.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarketplaceOrder {
    pub credit_id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub price: u32,
}

/// Struct representing a compliance rule.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplianceRule {
    pub rule_id: u64,
    pub description: Symbol,
}

/// Struct representing a security measure.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecurityMeasure {
    pub measure_id: u64,
    pub description: Symbol,
}
