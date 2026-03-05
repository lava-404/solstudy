use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MinterRole {
    /// Minter wallet
    pub minter: Pubkey,
    #[max_len(32)]
    /// Human-readable label (e.g. "Backend", "Seasonal Event")
    pub label: String,   // max 32 bytes

    /// Max XP allowed per reward_xp call (0 = unlimited)
    pub max_xp_per_call: u64,

    /// Total XP minted by this minter
    pub total_xp_minted: u64,

    /// Whether this minter is active
    pub is_active: bool,

    /// Bump for PDA
    pub bump: u8,
}