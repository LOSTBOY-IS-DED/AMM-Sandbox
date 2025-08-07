use anchor_lang::prelude::*;

declare_id!("95RfKC6UDmxeMjR9AAhF9xVauYmeFQawVK5W51tHn8wb");

pub mod constant;
pub mod errors;
pub mod instructions;
pub mod states;

pub use instructions::*;
#[program]
pub mod amms {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!(
            "Depositing {} into account {}",
            amount,
            ctx.accounts.user.key()
        );
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        msg!(
            "Withdrawing {} from account {}",
            amount,
            ctx.accounts.user.key()
        );
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        msg!(
            "Swapping {} from account {}",
            amount,
            ctx.accounts.user.key()
        );
        Ok(())
    }
}
