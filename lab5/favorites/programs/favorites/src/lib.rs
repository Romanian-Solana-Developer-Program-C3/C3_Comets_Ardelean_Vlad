use anchor_lang::prelude::*;

declare_id!("8u4S6a4jRam2kAJjQmdGn2fLVxUZfKdm7VqkxHgV6emf");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
