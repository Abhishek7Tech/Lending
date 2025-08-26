use anchor_lang::prelude::*;
mod instructions;
use instructions::*;
mod error;
mod constants;

pub mod state;
declare_id!("ESCy6rH5wjrGMdc8sMrc6reERuQXoRytqxpiDUvvZCjH");

#[program]
pub mod lending {

    use super::*;

    pub fn initialize_bank(
        ctx: Context<InitBank>,
        max_ltv: u64,
        liquidation_threshold: u64,
    ) -> Result<()> {
        process_init_bank(ctx, max_ltv, liquidation_threshold)
    }

    pub fn initialize_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        process_init_user(ctx, usdc_address)
    }

    pub fn deposite(ctx: Context<Deposite>, amount: u64) -> Result<()> {
        process_depoiste(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        process_withdraw(ctx, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        process_borrow(ctx, amount)
    }
    
}
