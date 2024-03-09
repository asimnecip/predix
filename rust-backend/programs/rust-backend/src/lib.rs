use anchor_lang::prelude::*;

declare_id!("4HGr5zRPPn6SHLX5E3oBsu5WU77htwvzU4Bqs1PYTS9q");

#[program]
pub mod rust_backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
