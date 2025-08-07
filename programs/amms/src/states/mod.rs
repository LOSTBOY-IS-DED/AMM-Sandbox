use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    // indentifier to deferentiate between multiple pools
    // helps in deriving PDA for each AMMS instance
    pub seed: u64,

    // Optional maintainer of the pool
    // can be used to control parameter like fees or to pause the AMMs
    // if set to None, the pool is completey decentralized
    pub authority: Option<Pubkey>,

    // SPL token mint address for token_x -> first asset in the pair
    pub mint_x: Pubkey,

    // SPL token mint address for token_y -> second asset in the pair
    pub mint_y: Pubkey,

    // Swap fee token on each trader in basis points like (30 = 0.30%)
    // this fees goes to the liquidity providers or protocol treasury
    pub fee: u16,

    // Boolean to lock the amm
    // if set to true , the swap and deposit functions are disabled
    pub locked: bool,

    // Bump to derive PDA for this config account
    // Ensure the correct address is derived on chain
    pub config_bump: u8,

    //Bump to derive PDA for the LP token mint account
    //Lp token represnt a user share of the liquidity pool
    pub lp_bump: u8,
}
