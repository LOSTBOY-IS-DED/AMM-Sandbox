use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::states::Config;

// Accounts required for initializing a new AMM pool
// This structs defines all the accounts needed to create a new liquidity pool
// with x and y tokens to setup the init pool configuration

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    // the admin who is creating the pool must sign the transaction
    // mutable bacause the will pay for the account creation cost
    #[account(mut)]
    pub admin: Signer<'info>,

    // The mint account for the first token x in the trading pair
    // This is immutable as we only need to read the mint information for validation
    pub mint_x: Account<'info, Mint>,

    // The mint account for the first token x in the trading pair
    // This is immutable as we only need to read the mint information for validation
    pub mint_y: Account<'info, Mint>,

    // The main configuration accout for the AMM pool
    /// This PDA stores all pool settings, token references, and metadata
    /// Uses a custom seed provided by the admin to allow multiple pools
    #[account(
        init,   // create a new account
        payer = admin,  // admin pays for creation of the account
        seeds = [b"config", seed.to_le_bytes().as_ref()],  // seeds for the config account
        bump , // achor finds conical bump
        space = Config::INIT_SPACE  // require space for the config account
    )]
    pub config: Account<'info, Config>,

    // The liquidity providor token mint for this pool
    // Users recieve this token as reciepents when providing liquidity
    // The config PDA is set as mint authority for security
    #[account(
        init,   // create a new account
        payer = admin,  // admin pays for creation of the account
        mint::decimals = 6,
        mint::authority = config.key(),
        seeds = [b"lp", config.key().as_ref()],  // seeds for the config account
        bump , // achor finds conical bump
    )]
    pub mint_lp: Account<'info, Mint>,

    // The vault that will hold all deposited token X
    // This is an associated token account owned by the config PDA
    // All token X deposits from users go into this vault
    #[account(
        init,                                           // Create new token account
        payer = admin,                                 // Admin pays for creation
        associated_token::mint = mint_x,               // Associated with mint_x
        associated_token::authority = config,          // Owned by config PDA
        associated_token::token_program = token_program, // Uses SPL Token program
    )]
    pub vault_x: Account<'info, TokenAccount>,

    /// The vault that will hold all deposited token Y
    /// This is an associated token account owned by the config PDA
    /// All token Y deposits from users go into this vault
    #[account(
        init,                                           // Create new token account
        payer = admin,                                 // Admin pays for creation
        associated_token::mint = mint_y,               // Associated with mint_y
        associated_token::authority = config,          // Owned by config PDA
        associated_token::token_program = token_program, // Uses SPL Token program
    )]
    pub vault_y: Account<'info, TokenAccount>,

    /// SPL Token program for token operations
    pub token_program: Program<'info, Token>,

    /// Associated Token program for ATA operations
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    // Initializes a new AMM pool with the specified parameters
    //
    // This function creates a new constant product AMM pool that allows users to:
    // - Swap between token X and token Y
    // - Provide liquidity and earn fees
    // - Remove liquidity by burning LP tokens
    //
    // The pool uses the constant product formula (x * y = k) to determine
    // exchange rates and maintain liquidity depth.
    //
    // # Arguments
    // * `seed` - Unique identifier for this pool, allows multiple pools with same token pairs
    // * `fee` - Trading fee in basis points (e.g., 30 = 0.30%)
    // * `authority` - Optional authority that can update pool settings (None = immutable)
    // * `bumps` - Canonical bump values for PDAs (provided by Anchor)
    pub fn initialize(
        &mut self,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        // Initialize the config account with all pool parameters
        self.config.set_inner(Config {
            seed,                      // Unique pool identifier
            authority,                 // Optional update authority
            mint_x: self.mint_x.key(), // First token in the pair
            mint_y: self.mint_y.key(), // Second token in the pair
            fee,                       // Trading fee in basis points
            locked: false,             // Pool starts unlocked (active)
            config_bump: bumps.config, // PDA bump for config account
            lp_bump: bumps.mint_lp,    // PDA bump for LP mint
        });

        // Pool is now ready for liquidity deposits and trading
        Ok(())
    }
}
