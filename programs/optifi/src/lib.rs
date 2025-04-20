use anchor_lang::prelude::*;

declare_id!("7HSCEYkgef7SnXk9DoZ4rEh5Qg5yvccpidCxfzcm7wMf");

#[program]
pub mod optifi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
