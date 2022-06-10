use anchor_lang::prelude::*;

#[cfg(feature = "local-testing")]
declare_id!("6KBz9djJAH3gRHscq9ujMpyZ5bCK9a27o3ybDtJLXowz");

// TODO: replace with real deployment key
#[cfg(not(feature = "local-testing"))]
declare_id!("3zSwHpEF8svwihadvnx7q2EagTyW1tvwn354gzvF5Zh4");

pub mod anchor_len;
pub mod instructions;
pub mod rational;
pub mod state;

use instructions::*;
use state::*;

#[program]
pub mod unstake {
    use super::*;
    pub fn create_pool(ctx: Context<CreatePool>, fee: Fee) -> Result<()> {
        ctx.accounts.run(fee)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
