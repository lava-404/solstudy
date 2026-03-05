use std::task::Context;

use anchor_lang::prelude::{self, CpiContext, InterfaceAccount};
use anchor_spl::{associated_token::spl_associated_token_account::solana_program::stake::config::Config, token::TokenAccount};

use crate::Course;

#[derive(Accounts)]
pub struct CompleteLesson<'info> {
  #[account(mut)]
  pub backend_signer: Signer<'info>,

  #[account(
    seeds = [b"config"],
    bump = config.bump
  )]
  pub config: Account<'info, Config>,

  #[account(
    mut,
    seeds = [b"course", course.course_id.as_bytes]
  )]
  pub course: Account<'info, Course>,

  #[account(
    mut,
    seeds = [b"enrollment", course.course_id.as_le_bytes(), enrollment.learner.key()],
    bump = enrollment.bump,
    constraint = enrollment.course == course.key()
  )]
  pub enrollment: Account<'info, Enrollment>,

  #[account(
    mut,
    address = config.total_xp_minted)]
  pub xp_mint: InterfaceAccount<'info, Mint>,

  #[account(mut)]
  pub learner_ata: InterfaceAccount<'info, TokenAccount>,

  pub token_program: Interface<'info, TokenInterface>,

}


pub fn complete_lesson(
  ctx: Context<CompleteLesson>,
  lesson_index: u16,
) -> Result<()> {

  let config = &ctx.accounts.config;
  let course = &ctx.accounts.course;
  let enrollment = &mut ctx.accounts.enrollment;

  // -----------------------------
  // Backend authorization
  // -----------------------------
  require!(
      ctx.accounts.backend_signer.key() == config.backend_signer,
      ErrorCode::Unauthorized
  );

  // -----------------------------
  // Ensure not finalized
  // -----------------------------
  require!(
      enrollment.completed_at.is_none(),
      ErrorCode::CourseAlreadyFinalized
  );

  // -----------------------------
  // Lesson bounds check
  // -----------------------------
  require!(
      lesson_index < course.lesson_count,
      ErrorCode::LessonOutOfBounds
  );

  // -----------------------------
  // Bitmap logic
  // -----------------------------
  let word_index = (lesson_index / 64) as usize;
  let bit_index = lesson_index % 64;

  let mask = 1u64 << bit_index;

  // Check already completed
  require!(
      enrollment.lesson_flags[word_index] & mask == 0,
      ErrorCode::LessonAlreadyCompleted
  );

  // Set bit
  enrollment.lesson_flags[word_index] |= mask;

  // -----------------------------
  // Mint XP
  // -----------------------------
  let seeds = &[b"config".as_ref(), &[config.bump]];
let signer_seeds = &[&seeds[..]];

  let cpi_accounts = MintTo {
      mint: ctx.accounts.xp_mint.to_account_info(),
      to: ctx.accounts.learner_xp_ata.to_account_info(),
      authority: ctx.accounts.config.to_account_info(),
  };

  let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer_seed);

  mint_to(cpi_ctx, course.xp_per_lesson)?;

  Ok(())
}