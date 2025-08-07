use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::states::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {}

impl<'info> Initialize<'info> {}
