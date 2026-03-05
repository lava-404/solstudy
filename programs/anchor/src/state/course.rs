use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Course {
    /// Unique course identifier (slug-like)
    #[max_len(32)]
    pub course_id: String,

    /// Creator wallet (receives creator_reward_xp)
    pub creator_wallet: Pubkey,

    /// XP per lesson
    pub xp_per_lesson: u64,

    /// Number of lessons (max 256 enforced elsewhere)
    pub lesson_count: u16,

    /// XP given to creator once threshold met
    pub creator_reward_xp: u64,

    /// Minimum completions required before creator reward activates
    pub min_completions_for_reward: u32,

    /// Total times course completed
    pub total_completions: u32,

    /// Track identifier (e.g. "anchor", q"defi")
    #[max_len(20)]
    pub track: String,

    /// Optional prerequisite course
    pub prerequisite: Option<Pubkey>,

    /// Difficulty: 1 = beginner, 2 = intermediate, 3 = advanced
    pub difficulty: u8,

    /// Whether course is active
    pub is_active: bool,

    /// PDA bump
    pub bump: u8,
}