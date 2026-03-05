use anchor_lang::prelude::*;

use crate::{Config, Course};

#[derive(Accounts)]
pub struct CreateCourse<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,

  #[account(
    init,
    payer = authority,
    seeds = [b"course", course_id.as_bytes()],
    space = 8 + Course::INIT_SPACE,
    bump
  )]
  pub course: Account<'info, Course>,

  #[account(
    seeds = [b"config"],
    bump = config.bump,
    has_one = authority
  )]
  pub config: Account<'info, Config>,

  pub system_program: Program<'info, System>

}

pub fn update_config(ctx: Context<CreateCourse>, course_id: String, creator_wallet: Pubkey, xp_per_lesson: u64,
lesson_count: u16,    creator_reward_xp: u64,
min_completions_for_reward: u32, total_completions: u32, track: String, prerequisite: Option<Pubkey>, difficulty: u8, 
is_active: bool) -> Result<()> {
  let course = &mut ctx.accounts.course;
  course.course_id = course_id;
  course.creator_wallet = creator_wallet;
  course.xp_per_lesson = xp_per_lesson;
   course.lesson_count = lesson_count;
  course.creator_reward_xp = creator_reward_xp;
  course.min_completions_for_reward = min_completions_for_reward;
  course.total_completions = total_completions;
  course.track = track;
  course.prerequisite = prerequisite;
  course.difficulty = difficulty;
  course.is_active = true;

  Ok(())
}
