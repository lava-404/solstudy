use anchor_lang::{context, prelude::*};

use crate::{ MinterRole, Config };
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct UpdateConfig <'info>{
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(
    mut,
    seeds = [b"config"],
    bump = config.bump,
    has_one = authority
  )]
  pub config: Account<'info, Config>,
  pub new_backend_signer: UncheckedAccount<'info>,

  #[account(
    mut,
    seeds = [b"minter", config.backend_signer.as_ref()],
    bump
)]
  pub old_minter_role: Option<Account<'info, MinterRole>>,

  #[account(
    init,
    payer = authority,
    seeds = [b"minter", new_backend_signer.key().as_ref()],
    bump,
    space = 8 + MinterRole::INIT_SPACE
  )]
  pub new_minter_role: Account<'info, MinterRole>,

  pub system_program: Program<'info, System>
}

pub fn update_config(ctx: Context<UpdateConfig>) -> Result<()>{
  
  
  let old_backend_signer =  ctx.accounts.config.backend_signer;
  let config = &mut ctx.accounts.config;
  if let Some(old_minter_role) = &mut ctx.accounts.old_minter_role {
    require! (
      old_minter_role.minter == old_backend_signer,
      ErrorCode::Unauthorized
    );
    old_minter_role.is_active = false;
  }
  config.backend_signer = ctx.accounts.new_backend_signer.key();

  let new_minter = &mut ctx.accounts.new_minter_role;

  new_minter.minter = ctx.accounts.new_backend_signer.key();
  new_minter.label = "Backend".to_string();
  new_minter.max_xp_per_call = 0;
  new_minter.is_active = true;

  Ok(())
}
