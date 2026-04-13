//! Drishti Protocol Constants
//! Source: Butwal Metropolitan Policy Document 2082/083

/// Basis points denominator. Use this for all percentage math.
/// NEVER use f64. NEVER use raw percentages.
/// Example: 30% = 3000 basis points. max = (budget * 3000) / 10_000
pub const BPS_DENOMINATOR: u64 = 10_000;

/// Procurement thresholds in NPR (1 token = 1 NPR, 0 decimals)
pub const DIRECT_PURCHASE_LIMIT:  u64 = 100_000;   // 1 Lakh
pub const QUOTATION_LIMIT:        u64 = 500_000;   // 5 Lakh
pub const SEALED_QUOTATION_LIMIT: u64 = 2_000_000; // 20 Lakh

/// Ward-zone disbursement ceilings in basis points
/// Source: Section (th) point 21, Butwal Metropolitan Policy 2082/083
pub const BPS_WARD_TINAU_EAST:      u16 = 3_000; // Wards 14-19: 30%
pub const BPS_WARD_12_13:           u16 = 2_500; // Wards 12-13: 25%
pub const BPS_WARD_WEST_OUTER:      u16 = 2_000; // Wards 1,2,11w,14-19: 20%
pub const BPS_GRAVEL_MAINTENANCE:   u16 = 1_500; // Road gravel/maintenance: 15%
pub const BPS_PUBLIC_SPACES:        u16 = 1_000; // Religious/educational/sports: 10%
pub const BPS_MARGINAL_FLOOR:       u16 =   500; // Marginal settlements: 5% (floor)

/// Billing advance and retention
pub const FIRST_BILL_ADVANCE_BPS: u16 =   2_000; // 20% advance on first bill
pub const RETENTION_BPS:          u16 =     500; // 5% withheld for defect liability

/// 14th Amendment of Public Procurement Rules: max variation order
pub const VARIATION_ORDER_MAX_BPS: u16 = 12_500; // 125% of original contract

/// Time constants in seconds
pub const PROOF_MAX_AGE_SECS:     i64 = 30 * 24 * 3_600; // 30 days
pub const GPS_PHOTO_MAX_LAG_SECS: i64 =  2 * 3_600;       // 2 hours
pub const IDLE_FUNDS_SECS:        i64 = 60 * 24 * 3_600;  // 60 days flag threshold

/// GPS: store coordinates as integer (real_coord * GPS_PRECISION = stored)
pub const GPS_PRECISION: i64 = 10_000_000; // 7 decimal places
pub const GPS_SITE_RADIUS_M: i64 = 200;    // 200m site boundary tolerance

/// Anomaly severity at which project auto-freezes
pub const AUTO_FREEZE_SEVERITY: u8 = 8;

/// Max signers in a multisig disbursement
pub const MAX_MULTISIG_SIGNERS: usize = 4;
