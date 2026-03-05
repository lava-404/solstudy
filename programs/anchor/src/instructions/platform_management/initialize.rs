use anchor_lang::prelude::*;
use crate::state::{Config, MinterRole};
use anchor_spl::token_interface::{
  Mint, TokenInterface,
};

#[derive(Accounts)]
pub struct Initialize <'info>{
  #[account(mut)]
  pub authority: Signer<'info>,

  #[account(
    init,
    payer = authority,
    space = 8 + Config::INIT_SPACE,
    seeds= [b"config"],
    bump,
  )]
  pub config: Account<'info, Config>,

  #[account(
    init,
    payer = authority,
    mint::decimals = 9,
    mint::authority = config.key(),
    mint::freeze_authority = config.key(),
  )]
  pub xp_mint: InterfaceAccount<'info, Mint>,

  pub backend_signer: UncheckedAccount<'info>,

  
  #[account(
    init,
    payer = authority,
    space = 8 + MinterRole::INIT_SPACE,
    seeds= [b"minter_role", backend_signer.key().as_ref()],
    bump,
  )]
  pub minter_role: Account<'info, MinterRole>,
  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
  let config = &mut ctx.accounts.config;
  let minter_role = &mut ctx.accounts.minter_role;

  // -----------------------
  // CONFIG SETUP
  // -----------------------
  config.authority = ctx.accounts.authority.key();
  config.backend_signer = ctx.accounts.backend_signer.key();
  config.xp_mint = ctx.accounts.xp_mint.key();
  config.bump = ctx.bumps.config;

  // -----------------------
  // AUTO REGISTER BACKEND AS MINTER
  // -----------------------
  minter_role.minter = ctx.accounts.backend_signer.key();
  minter_role.label = "Backend".to_string();
  minter_role.max_xp_per_call = 0; // unlimited
  minter_role.total_xp_minted = 0;
  minter_role.is_active = true;
  minter_role.bump = ctx.bumps.minter_role;

  Ok(())
}