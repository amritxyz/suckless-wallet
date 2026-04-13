use anchor_lang::prelude::*;

#[error_code]
pub enum RegistryError {
    #[msg("Wallet is already registered in the Drishti registry")]
    DuplicateWallet,
    #[msg("Only the FCGO authority can register or modify wallets")]
    UnauthorizedRegistrar,
    #[msg("Wallet type value is not a valid WalletType variant (0-7)")]
    InvalidWalletType,
    #[msg("IRD number is required for Vendor wallet type")]
    MissingIrdNumber,
    #[msg("Name field exceeds maximum length of 64 bytes")]
    NameTooLong,
    #[msg("Tax clearance expiry timestamp is in the past")]
    ExpiredTaxClearance,
    #[msg("Cannot deactivate a wallet that is already inactive")]
    AlreadyInactive,
}
