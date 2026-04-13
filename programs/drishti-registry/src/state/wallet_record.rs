use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum WalletType {
    Ministry      = 0,
    Province      = 1,
    District      = 2,
    Gaapalika     = 3,
    Vendor        = 4,
    Engineer      = 5,
    CitizenSigner = 6,
    CiaaAuditor   = 7,
}

#[account]
#[derive(InitSpace)]
pub struct WalletRecord {
    /// The registered Solana public key
    pub wallet: Pubkey,
    /// Governance role of this wallet in the system
    pub wallet_type: WalletType,
    /// UTF-8 display name, zero-padded to 64 bytes
    pub name: [u8; 64],
    /// IRD registration number — required for Vendor type, None otherwise
    pub ird_number: Option<[u8; 12]>,
    /// Which Gaapalika this wallet operates under
    pub gaapalika_id: [u8; 8],
    /// Whether this wallet is currently authorized to transact
    pub is_active: bool,
    /// Unix timestamp of registration
    pub registered_at: i64,
    /// The authority wallet that performed the registration
    pub registered_by: Pubkey,
    /// Tax clearance certificate expiry — required for Vendor, None for others
    pub tax_clearance_expiry: Option<i64>,
    /// PDA bump seed
    pub bump: u8,
}
