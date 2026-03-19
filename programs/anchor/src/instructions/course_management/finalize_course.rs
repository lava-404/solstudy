use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, MintTo, mint_to};

use crate::error::ErrorCode;
use crate::state::{Config, Course, Enrollment};

#[derive(Accounts)]
pub struct FinalizeCourse<'info> {

    /// Backend must sign
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
    pub learner_xp_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub creator_xp_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn finalize_course(ctx: Context<FinalizeCourse>) -> Result<()> {

    let config = &ctx.accounts.config;
    let course = &mut ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;

    // -------------------------
    // Backend authorization
    // -------------------------
    require!(
        ctx.accounts.backend_signer.key() == config.backend_signer,
        ErrorCode::Unauthorized
    );

    // -------------------------
    // Not already finalized
    // -------------------------
    require!(
        enrollment.completed_at.is_none(),
        ErrorCode::CourseAlreadyFinalized
    );

    // -------------------------
    // Verify full bitmap
    // -------------------------
    let mut completed_count: u32 = 0;

    for word in enrollment.lesson_flags.iter() {
        completed_count += word.count_ones();
    }

    require!(
        completed_count == course.lesson_count as u32,
        ErrorCode::CourseNotCompleted
    );

    // -------------------------
    // Calculate completion bonus
    // -------------------------
    let total_lesson_xp = course
        .xp_per_lesson
        .checked_mul(course.lesson_count as u64)
        .ok_or(ErrorCode::Overflow)?;

    let completion_bonus = total_lesson_xp / 2;

    // -------------------------
    // FIXED signer seeds
    // -------------------------
    let signer: &[&[&[u8]]] = &[&[b"config", &[config.bump]]];

    // -------------------------
    // Mint completion bonus to learner
    // -------------------------
    let learner_cpi_accounts = MintTo {
        mint: ctx.accounts.xp_mint.to_account_info(),
        to: ctx.accounts.learner_xp_ata.to_account_info(),
        authority: ctx.accounts.config.to_account_info(),
    };

    let learner_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        learner_cpi_accounts,
        signer,
    );

    mint_to(learner_cpi_ctx, completion_bonus)?;

    // -------------------------
    // Increment total completions
    // -------------------------
    course.total_completions = course
        .total_completions
        .checked_add(1)
        .ok_or(ErrorCode::Overflow)?;

    // -------------------------
    // Creator reward (threshold gated)
    // -------------------------
    if course.total_completions >= course.min_completions_for_reward {
        if course.creator_reward_xp > 0 {

            let creator_cpi_accounts = MintTo {
                mint: ctx.accounts.xp_mint.to_account_info(),
                to: ctx.accounts.creator_xp_ata.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            };

            let creator_cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                creator_cpi_accounts,
                signer,
            );

            mint_to(creator_cpi_ctx, course.creator_reward_xp)?;
        }
    }

    // -------------------------
    // Mark completed
    // -------------------------
    enrollment.completed_at = Some(Clock::get()?.unix_timestamp);

    Ok(())
}