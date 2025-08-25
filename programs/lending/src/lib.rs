use anchor_lang::prelude::*;
pub mod state;
declare_id!("ESCy6rH5wjrGMdc8sMrc6reERuQXoRytqxpiDUvvZCjH");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
