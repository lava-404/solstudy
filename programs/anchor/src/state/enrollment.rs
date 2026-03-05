use anchor_lang::prelude::*;
#[account]
pub struct Enrollment {
    /// Course this enrollment belongs to
    pub course: Pubkey,

    /// Learner wallet
    pub learner: Pubkey,

    /// 256-bit lesson completion bitmap (4 × 64 bits)
    pub lesson_flags: [u64; 4],

    /// Timestamp when enrolled
    pub enrolled_at: i64,

    /// Timestamp when course completed (None if incomplete)
    pub completed_at: Option<i64>,

    /// Credential NFT pubkey (None until issued)
    pub credential_asset: Option<Pubkey>,

    /// Bump for PDA
    pub bump: u8,
}