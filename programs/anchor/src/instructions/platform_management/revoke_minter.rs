use anchor_lang::prelude::*;
use crate::state::{Config, MinterRole};

#[derive(Accounts)]
pub struct RevokeMinter<'info> {

    /// Only authority can revoke
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"minter", minter_role.minter.as_ref()],
        bump = minter_role.bump,
        close = authority
    )]
    pub minter_role: Account<'info, MinterRole>,
}

pub fn revoke_minter(ctx: Context<RevokeMinter>) -> Result<()> {
  // No extra logic needed.
  // Anchor will:
  // - close the account
  // - send rent to authority
  Ok(())
}