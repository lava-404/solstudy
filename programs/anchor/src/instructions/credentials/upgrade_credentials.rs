use anchor_lang::prelude::*;
use mpl_core::ID as MPL_CORE_ID;
use mpl_core::instructions::UpdateV1CpiBuilder;

use crate::error::ErrorCode;
use crate::state::{Config, Course, Enrollment};

#[derive(Accounts)]
pub struct UpgradeCredential<'info> {

    pub backend_signer: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

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
            enrollment.learner.as_ref()
        ],
        bump = enrollment.bump,
        constraint = enrollment.course == course.key()
    )]
    pub enrollment: Account<'info, Enrollment>,

    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    /// CHECK: This is the Metaplex Core program, validated via address constraint
    #[account(address = MPL_CORE_ID)]
    pub core_program: UncheckedAccount<'info>,
}

pub fn upgrade_credential(
    ctx: Context<UpgradeCredential>,
    credential_name: String,
    metadata_uri: String,
    _courses_completed: u32,
    _total_xp: u64,
) -> Result<()> {

    let config = &ctx.accounts.config;
    let enrollment = &mut ctx.accounts.enrollment;

    // -----------------------------
    // Backend auth
    // -----------------------------
    require!(
        ctx.accounts.backend_signer.key() == config.backend_signer,
        ErrorCode::Unauthorized
    );

    // -----------------------------
    // Must be finalized
    // -----------------------------
    require!(
        enrollment.completed_at.is_some(),
        ErrorCode::CourseNotFinalized
    );

    // -----------------------------
    // Credential must exist
    // -----------------------------
    let stored_asset = enrollment
        .credential_asset
        .ok_or(ErrorCode::CredentialAssetMismatch)?;

    require!(
        stored_asset == ctx.accounts.asset.key(),
        ErrorCode::CredentialAssetMismatch
    );

    // -----------------------------
    // FIXED signer
    // -----------------------------
    let signer: &[&[&[u8]]] = &[&[b"config", &[config.bump]]];

    // -----------------------------
    // ONLY metadata update here
    // -----------------------------
    UpdateV1CpiBuilder::new(&ctx.accounts.core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .authority(Some(&ctx.accounts.config.to_account_info()))
        .new_name(credential_name)
        .new_uri(metadata_uri)
        .invoke_signed(signer)?;

    Ok(())
}