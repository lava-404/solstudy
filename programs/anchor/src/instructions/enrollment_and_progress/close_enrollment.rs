use anchor_lang::prelude::*;
use crate::state::{Course, Enrollment};
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct CloseEnrollment<'info> {

    /// Learner must sign
    #[account(mut)]
    pub learner: Signer<'info>,

    #[account(
        seeds = [b"course", course.course_id.as_bytes()],
        bump = course.bump
    )]
    pub course: Account<'info, Course>,

    #[account(
        mut,
        seeds = [
            b"enrollment",
            course.course_id.as_bytes(),
            learner.key().as_ref()
        ],
        bump = enrollment.bump,
        close = learner
    )]
    pub enrollment: Account<'info, Enrollment>,
}

pub fn close_enrollment(ctx: Context<CloseEnrollment>) -> Result<()> {

  let enrollment = &ctx.accounts.enrollment;

  // -------------------------
  // If completed → allow immediately
  // -------------------------
  if enrollment.completed_at.is_some() {
      return Ok(());
  }

  // -------------------------
  // If incomplete → enforce 24h cooldown
  // -------------------------
  let now = Clock::get()?.unix_timestamp;

  require!(
      now - enrollment.enrolled_at >= 86400,
      ErrorCode::UnenrollCooldown
  );

  Ok(())
}