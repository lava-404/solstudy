use anchor_lang::prelude::*;
use crate::state::{Config, MinterRole};
use crate::error::ErrorCode;
#[derive(Accounts)]
#[instruction(label: String)]
pub struct RegisterMinter<'info> {

    /// Only authority can register
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    /// Wallet being granted minter role
    /// CHECK: Only used as seed + stored
    pub minter: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [b"minter", minter.key().as_ref()],
        bump,
        space = 8 + MinterRole::INIT_SPACE
    )]
    pub minter_role: Account<'info, MinterRole>,

    pub system_program: Program<'info, System>,
}

pub fn register_minter(
  ctx: Context<RegisterMinter>,
  label: String,
  max_xp_per_call: u64,
) -> Result<()> {

  require!(
      label.len() <= 32,
      ErrorCode::LabelTooLong
  );

  let minter_role = &mut ctx.accounts.minter_role;

  minter_role.minter = ctx.accounts.minter.key();
  minter_role.label = label;
  minter_role.max_xp_per_call = max_xp_per_call; // 0 = unlimited
  minter_role.total_xp_minted = 0;
  minter_role.is_active = true;
  minter_role.bump = ctx.bumps.minter_role;

  Ok(())
}