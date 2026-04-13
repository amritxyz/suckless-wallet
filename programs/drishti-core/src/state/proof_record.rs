use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProofRecord {
    pub project:       Pubkey,
    pub engineer:      Pubkey,
    /// Latitude stored as integer: real_lat * 10_000_000
    pub gps_lat:       i64,
    /// Longitude stored as integer: real_lng * 10_000_000
    pub gps_lng:       i64,
    /// IPFS CIDv1 — must start with [0x12, 0x20]
    pub ipfs_cid:      [u8; 46],
    /// SHA256 of the photo file for tamper detection
    pub photo_sha256:  [u8; 32],
    /// Progress at time of proof in basis points (0-10000)
    pub progress_bps:  u16,
    pub submitted_at:  i64,
    /// EXIF timestamp from the photo — must be within 2h of submitted_at
    pub gps_timestamp: i64,
    pub is_disputed:          bool,
    pub dispute_evidence_cid: Option<[u8; 46]>,
    pub bump: u8,
}
