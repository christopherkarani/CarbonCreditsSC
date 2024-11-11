#![no_std]

use soroban_sdk::{contracterror, Env};

/// Error types for the Issuance module.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IssuanceError {
    UnauthorizedIssuer,
    InvalidCreditData,
    DuplicateCredit,
}

/// Error types for the Verification module.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationError {
    UnauthorizedVerifier,
    InvalidVerificationData,
    VerificationFailed,
}

/// Error types for the Marketplace module.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MarketplaceError {
    UnauthorizedUser,
    InvalidTransaction,
    InsufficientBalance,
}

/// Error types for the Tracking module.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrackingError {
    CreditNotFound,
    AlreadyRetired,
    InvalidCreditStatus,
}

/// Error types for the Compliance module.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ComplianceError {
    NonCompliance,
    SecurityBreach,
    UnauthorizedAccess,
}
