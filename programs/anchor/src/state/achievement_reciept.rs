use anchor_lang::prelude::*;

#[account]
pub struct AchievementReceipt {
    /// Achievement ID (or derive via seed)
    pub achievement_id: String,  // max 32 bytes

    /// Recipient wallet
    pub recipient: Pubkey,

    /// Bump
    pub bump: u8,
}