use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum AnomalyType {
    Structuring      = 1, // Splitting large contracts to avoid tender threshold
    IdleFunds        = 2, // Budget allocated but no transfers for 60+ days
    PriceSpike       = 3, // Invoice 3x average for this purpose_code/Gaapalika
    AsareBikas       = 4, // Year-end spending surge (Q4 >> Q1+Q2+Q3)
    GpsContradiction = 5, // Citizen proof contradicts engineer proof location
}

#[account]
#[derive(InitSpace)]
pub struct AnomalyFlag {
    pub project:      Pubkey,
    pub flag_type:    AnomalyType,
    /// Severity 1-10. At 8+, project is auto-frozen.
    pub severity:     u8,
    pub evidence_cid: [u8; 46],
    pub reported_by:  Pubkey,
    pub reported_at:  i64,
    pub auto_frozen:  bool,
    pub bump:         u8,
}
