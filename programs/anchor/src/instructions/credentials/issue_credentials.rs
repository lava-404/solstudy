use anchor_lang::prelude::*;
use mpl_core::ID as MPL_CORE_ID;
use mpl_core::instructions::CreateV2CpiBuilder;
use mpl_core::types::{
    Plugin,
    PluginAuthorityPair,
    PermanentFreezeDelegate,
    Attributes,
    Attribute,
};

use crate::error::ErrorCode;
use crate::state::{Config, Course, Enrollment};

#[derive(Accounts)]
pub struct IssueCredential<'info> {

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

    /// CHECK: created via CPI
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    /// CHECK: Metaplex verifies
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,

    /// CHECK: must match enrollment
    pub learner: UncheckedAccount<'info>,

    /// CHECK: This is the Metaplex Core program, validated via address constraint
    #[account(address = MPL_CORE_ID)]
    pub core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn issue_credential(
    ctx: Context<IssueCredential>,
    credential_name: String,
    metadata_uri: String,
    courses_completed: u32,
    total_xp: u64,
) -> Result<()> {

    let config = &ctx.accounts.config;
    let enrollment = &mut ctx.accounts.enrollment;
    let course = &ctx.accounts.course;

    // -----------------------------
    // Backend authorization
    // -----------------------------
    require!(
        ctx.accounts.backend_signer.key() == config.backend_signer,
        ErrorCode::Unauthorized
    );

    // -----------------------------
    // Ensure learner matches enrollment
    // -----------------------------
    require!(
        enrollment.learner == ctx.accounts.learner.key(),
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
    // Validate metadata
    // -----------------------------
    require!(!credential_name.is_empty(), ErrorCode::InvalidAmount);
    require!(!metadata_uri.is_empty(), ErrorCode::InvalidAmount);

    // -----------------------------
    // Create credential if not exists
    // -----------------------------
    if enrollment.credential_asset.is_none() {

      let seeds: &[&[u8]] = &[b"config", &[config.bump]];
      let signer: &[&[&[u8]]] = &[seeds];
        // -----------------------------
        // Build plugins
        // -----------------------------
        let plugins = vec![
            PluginAuthorityPair {
                plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate {
                  frozen: true
                }),
                authority: None,
            },
            PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute {
                            key: "track".to_string(),
                            value: course.track.clone(),
                        },
                        Attribute {
                            key: "courses_completed".to_string(),
                            value: courses_completed.to_string(),
                        },
                        Attribute {
                            key: "total_xp".to_string(),
                            value: total_xp.to_string(),
                        },
                    ],
                }),
                authority: None,
            },
        ];

        // -----------------------------
        // CPI call
        // -----------------------------
        CreateV2CpiBuilder::new(&ctx.accounts.core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .authority(Some(&ctx.accounts.config.to_account_info()))
            .payer(&ctx.accounts.backend_signer.to_account_info())
            .owner(Some(&ctx.accounts.learner.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(credential_name)
            .uri(metadata_uri)
            .plugins(plugins)
            .invoke_signed(signer)?;

        // Store NFT pubkey
        enrollment.credential_asset = Some(ctx.accounts.asset.key());

    } else {
        return err!(ErrorCode::CredentialAlreadyIssued);
    }

    Ok(())
}