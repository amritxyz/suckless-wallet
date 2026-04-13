use anchor_lang::prelude::*;
use crate::constants::MAX_MULTISIG_SIGNERS;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum ProcurementMethod {
    DirectPurchase   = 0,
    Quotation        = 1,
    SealedQuotation  = 2,
    EBidding         = 3,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum BillingStage {
    NotStarted  = 0,
    FirstBill   = 1,
    SecondBill  = 2,
    FinalBill   = 3,
    Closed      = 4,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum PurposeCode {
    Road    = 0,
    School  = 1,
    Health  = 2,
    Water   = 3,
    Other   = 4,
}

#[account]
#[derive(InitSpace)]
pub struct ProjectState {
    // Identity
    pub project_id:       [u8; 8],
    pub gaapalika_wallet: Pubkey,
    pub purpose_code:     PurposeCode,
    pub ward_zone:        u8,

    // Budget — all values in NPR tokens (1 token = 1 NPR, 0 decimals)
    pub total_budget:      u64,
    pub variation_ceiling: u64,
    pub total_disbursed:   u64,
    pub retention_held:    u64,

    // Thresholds (from ward-zone lookup and procurement rules)
    pub max_disbursement_bps:          u16,
    pub procurement_method:            ProcurementMethod,
    pub citizen_contribution_required: bool,
    pub citizen_contribution_verified: bool,

    // Physical progress — stored in basis points (0-10000)
    pub physical_progress_bps:   u16,
    pub engineer_proof_hash:     [u8; 32],
    pub last_proof_submitted_at: i64,

    // Billing lifecycle
    pub billing_stage:         BillingStage,
    pub first_bill_released:   bool,
    pub second_bill_released:  bool,
    pub final_bill_released:   bool,

    // Variation orders (14th Amendment)
    pub variation_approved:  bool,
    pub variation_approver:  Option<Pubkey>,

    // Multi-sig disbursement approval
    pub multisig_required:  u8,
    pub multisig_collected: u8,
    #[max_len(MAX_MULTISIG_SIGNERS)]
    pub multisig_signers:   Vec<Pubkey>,

    // Time constraints
    pub chamasik:              u8,
    pub chamasik_deadline:     i64,
    pub defect_liability_end:  i64,

    // Anomaly and CIAA state
    pub anomaly_flag_count: u8,
    pub last_anomaly_at:    i64,
    pub is_frozen:          bool,

    pub bump: u8,
}
