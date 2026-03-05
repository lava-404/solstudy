use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct AchievementType {
    /// Unique achievement ID
    #[max_len(32)]
    pub achievement_id: String,   // max 32 bytes

    /// Display name
    #[max_len(64)]
    pub name: String,             // max 64 bytes

    /// Metadata URI for NFT
    #[max_len(200)]
    pub metadata_uri: String,     // max 200 bytes

    /// Metaplex Core collection address
    pub collection: Pubkey,

    /// Max supply (0 = unlimited)
    pub max_supply: u32,

    /// Current number minted
    pub current_supply: u32,

    /// XP reward granted when awarded
    pub xp_reward: u64,

    /// Whether achievement is active
    pub is_active: bool,

    /// Bump
    pub bump: u8,
}