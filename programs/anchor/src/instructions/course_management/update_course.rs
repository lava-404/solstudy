use anchor_lang::prelude::*;
use crate::state::{Config, Course};
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct UpdateCourse<'info> {
    /// Only authority can update course
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"course", course.course_id.as_bytes()],
        bump = course.bump
    )]
    pub course: Account<'info, Course>,
}
pub fn update_course(
  ctx: Context<UpdateCourse>,
  new_xp_per_lesson: Option<u64>,
  new_creator_reward_xp: Option<u64>,
  new_min_completions: Option<u32>,
  new_difficulty: Option<u8>,
  new_prerequisite: Option<Option<Pubkey>>, 
  new_is_active: Option<bool>,
) -> Result<()> {

  let course = &mut ctx.accounts.course;

  // Update XP per lesson
  if let Some(xp) = new_xp_per_lesson {
      course.xp_per_lesson = xp;
  }

  // Update creator reward XP
  if let Some(reward) = new_creator_reward_xp {
      course.creator_reward_xp = reward;
  }

  // Update threshold
  if let Some(min) = new_min_completions {
      course.min_completions_for_reward = min;
  }

  // Update difficulty
  if let Some(diff) = new_difficulty {
      require!(diff >= 1 && diff <= 3, ErrorCode::InvalidDifficulty);
      course.difficulty = diff;
  }

  // Update prerequisite
  // Option<Option<Pubkey>> allows:
  // None -> do not change
  // Some(None) -> remove prerequisite
  // Some(Some(pubkey)) -> set prerequisite
  if let Some(prereq) = new_prerequisite {
      course.prerequisite = prereq;
  }

  // Activate / deactivate course
  if let Some(active) = new_is_active {
      course.is_active = active;
  }

  Ok(())
}