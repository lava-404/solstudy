use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Config {
  pub authority: Pubkey,
  pub backend_signer: Pubkey,
  pub xp_mint: Pubkey,
  pub bump: u8
}