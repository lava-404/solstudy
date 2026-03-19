use anchor_lang::prelude::*;

use crate::state::{Config, Course};
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(course_id: String)]
pub struct CreateCourse<'info> {

  #[account(mut)]
  pub authority: Signer<'info>,

  #[account(
    init,
    payer = authority,
    seeds = [b"course", course_id.as_bytes()],
    bump,
    space = 8 + Course::INIT_SPACE
  )]
  pub course: Account<'info, Course>,

  #[account(
    seeds = [b"config"],
    bump = config.bump,
    has_one = authority
  )]
  pub config: Account<'info, Config>,

  pub system_program: Program<'info, System>,
}

pub fn create_course(
  ctx: Context<CreateCourse>,
  course_id: String,
  creator_wallet: Pubkey,
  xp_per_lesson: u64,
  lesson_count: u16,
  creator_reward_xp: u64,
  min_completions_for_reward: u32,
  track: String,
  prerequisite: Option<Pubkey>,
  difficulty: u8,
) -> Result<()> {

  let course = &mut ctx.accounts.course;

  // -----------------------------
  // Validation
  // -----------------------------
  require!(!course_id.is_empty(), ErrorCode::CourseIdEmpty);
  require!(lesson_count > 0, ErrorCode::InvalidLessonCount);
  require!(difficulty >= 1 && difficulty <= 3, ErrorCode::InvalidDifficulty);

  // -----------------------------
  // Set data
  // -----------------------------
  course.course_id = course_id;
  course.creator_wallet = creator_wallet;
  course.xp_per_lesson = xp_per_lesson;
  course.lesson_count = lesson_count;
  course.creator_reward_xp = creator_reward_xp;
  course.min_completions_for_reward = min_completions_for_reward;
  course.total_completions = 0; // always start fresh
  course.track = track;
  course.prerequisite = prerequisite;
  course.difficulty = difficulty;
  course.is_active = true;
  course.bump = ctx.bumps.course;

  Ok(())
}