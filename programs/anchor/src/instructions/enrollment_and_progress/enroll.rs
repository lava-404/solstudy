use anchor_lang::prelude::*;

use crate::{Course, Enrollment};
use crate::error::ErrorCode;
#[derive(Accounts)]

pub struct Enroll<'info> {
  #[account(mut)]
  pub learner: Signer<'info>,

  #[account(
    mut,
    seeds = [b"course", course.course_id.as_bytes()],
    bump = course.bump
  )]
  pub course: Account<'info, Course>,
   
  #[account(
    init,
    payer = learner,
    seeds = [b"enrollment", course.course_id.as_bytes(), learner.key().as_ref()],
    space = 8 + Enrollment::INIT_SPACE,
    bump
  )]
  pub enrollment: Account<'info, Enrollment>,

      /// CHECK: Verified manually if needed
      pub prerequisite_enrollment: Option<Account<'info, Enrollment>>,


  
  pub system_program: Program<'info, System>
}
pub fn enroll(ctx: Context<Enroll>) -> Result<()> {
  let course = &ctx.accounts.course;
  let enrollment = &mut ctx.accounts.enrollment;

  // ---------------------------
  // Check course is active
  // ---------------------------
  require!(course.is_active, ErrorCode::CourseNotActive);

  // ---------------------------
  // Check prerequisite (if any)
  // ---------------------------
  if let Some(prereq_course) = course.prerequisite {

      let prereq_enrollment = ctx.accounts
          .prerequisite_enrollment
          .as_ref()
          .ok_or(ErrorCode::PrerequisiteNotMet)?;

      // Ensure prerequisite enrollment belongs to learner
      require!(
          prereq_enrollment.learner == ctx.accounts.learner.key(),
          ErrorCode::PrerequisiteNotMet
      );

      // Ensure prerequisite enrollment is for correct course
      require!(
          prereq_enrollment.course == prereq_course,
          ErrorCode::PrerequisiteNotMet
      );

      // Ensure prerequisite completed
      require!(
          prereq_enrollment.completed_at.is_some(),
          ErrorCode::PrerequisiteNotMet
      );
  }

  // ---------------------------
  // Initialize enrollment
  // ---------------------------
  enrollment.course = course.key();
  enrollment.learner = ctx.accounts.learner.key();
  enrollment.lesson_flags = [0; 4];
  enrollment.enrolled_at = Clock::get()?.unix_timestamp;
  enrollment.completed_at = None;
  enrollment.credential_asset = None;
  enrollment.bump = ctx.bumps.enrollment;

  Ok(())
}