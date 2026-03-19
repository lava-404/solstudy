use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, MintTo, mint_to};

use crate::state::{Course, Enrollment, Config};
use crate::error::ErrorCode;

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
    seeds = [b"course", course.course_id.as_bytes()],
    bump = course.bump
  )]
  pub course: Account<'info, Course>,

  #[account(
    mut,
    seeds = [
      b"enrollment",
      course.course_id.as_bytes(),
      enrollment.learner.as_ref()
    ],
    bump = enrollment.bump,
    constraint = enrollment.course == course.key()
  )]
  pub enrollment: Account<'info, Enrollment>,

  #[account(
    mut,
    address = config.xp_mint
  )]
  pub xp_mint: Account<'info, Mint>,

  #[account(mut)]
  pub learner_ata: Account<'info, TokenAccount>,

  pub token_program: Program<'info, Token>,
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

  require!(
      enrollment.lesson_flags[word_index] & mask == 0,
      ErrorCode::LessonAlreadyCompleted
  );

  enrollment.lesson_flags[word_index] |= mask;

  // -----------------------------
  // FIXED signer seeds
  // -----------------------------
  let signer: &[&[&[u8]]] = &[&[b"config", &[config.bump]]];

  // -----------------------------
  // Mint XP
  // -----------------------------
  let cpi_accounts = MintTo {
      mint: ctx.accounts.xp_mint.to_account_info(),
      to: ctx.accounts.learner_ata.to_account_info(),
      authority: ctx.accounts.config.to_account_info(),
  };

  let cpi_ctx = CpiContext::new_with_signer(
      ctx.accounts.token_program.to_account_info(),
      cpi_accounts,
      signer,
  );

  mint_to(cpi_ctx, course.xp_per_lesson)?;

  Ok(())
}