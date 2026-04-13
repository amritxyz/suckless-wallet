use anchor_lang::prelude::*;

#[error_code]
pub enum CoreError {
    // Registry checks
    #[msg("Destination wallet is not registered in the Drishti government registry")]
    UnregisteredDestination,
    #[msg("Registry PDA address does not match expected derivation — substitution attack blocked")]
    RegistryPdaMismatch,
    #[msg("Wallet address in registry does not match destination authority")]
    WalletAddressMismatch,

    // Tax compliance
    #[msg("Vendor tax clearance certificate has expired or is missing")]
    VendorTaxClearanceExpired,

    // Progress gate
    #[msg("Transfer amount exceeds the physical-progress-gated disbursement ceiling")]
    ExceedsProgressGate,
    #[msg("Transfer amount exceeds the ward-zone disbursement ceiling for this project")]
    ExceedsWardZoneCeiling,

    // Proof requirements
    #[msg("No engineer proof has been submitted for this project")]
    NoProofSubmitted,
    #[msg("Engineer proof is older than the 30-day validity window")]
    ProofTooOld,
    #[msg("Engineer proof hash is empty — submit a valid ProofRecord first")]
    InvalidProofHash,
    #[msg("IPFS CID must start with bytes [0x12, 0x20] (SHA256 multihash prefix)")]
    InvalidIpfsCid,
    #[msg("GPS timestamp in photo is more than 2 hours before submission time")]
    GpsTimestampTooOld,

    // Chamasik (quarterly deadline)
    #[msg("Chamasik quarterly spending deadline has been exceeded — funds are locked")]
    ChamasikDeadlineExceeded,

    // Procurement compliance
    #[msg("Amount exceeds the Direct Purchase limit of NPR 1 Lakh")]
    ExceedsDirectPurchaseLimit,
    #[msg("Amount exceeds the Quotation limit of NPR 5 Lakh")]
    ExceedsQuotationLimit,
    #[msg("Sealed Quotation requires at least 3-of-4 multi-sig approval")]
    InsufficientMultiSig,
    #[msg("E-Bidding projects require a verified tender ID from the oracle")]
    TenderNotVerified,

    // Project state
    #[msg("This project has been frozen by CIAA — all transfers are blocked")]
    ProjectFrozenByCiaa,
    #[msg("Ward zone value must be between 0 and 5")]
    InvalidWardZone,
    #[msg("Purpose code value must be between 0 and 4")]
    InvalidPurposeCode,

    // Governance
    #[msg("This wallet type is not authorized to sign this disbursement tier")]
    UnauthorizedSigner,
    #[msg("This wallet has already signed — duplicate signatures are not accepted")]
    AlreadySigned,
    #[msg("All multisig signer slots are full (maximum 4)")]
    SignerSlotsFull,
    #[msg("Only a Ministry-level wallet can approve a variation order")]
    OnlyMinistryCanApproveVariation,
    #[msg("Variation order ceiling exceeds the 14th Amendment limit of 125%")]
    ExceedsVariationLimit,
    #[msg("Only a CIAA Auditor wallet can freeze a project")]
    OnlyCiaaCanFreeze,

    // Arithmetic safety
    #[msg("Arithmetic overflow — computation aborted for safety")]
    ArithmeticOverflow,
}
