use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id!("7HSCEYkgef7SnXk9DoZ4rEh5Qg5yvccpidCxfzcm7wMf");

#[program]
pub mod optifi {
    use super::*;

    #[account]
    pub struct YieldPool {
        pub pool_id: String,
        pub total_deposits: u64,
        pub allocations: Vec<Allocation>,
        pub last_rebalance: i64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
    pub struct Allocation {
        pub protocol: String,
        pub percentage: u64,
        pub apy: f64,
    }

    pub fn create_pool(ctx: Context<CreatePool>, pool_id: String) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.pool_id = pool_id;
        pool.total_deposits = 0;
        pool.allocations = vec![];
        pool.last_rebalance = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 200 + 8)]
    pub pool: Account<'info, YieldPool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
