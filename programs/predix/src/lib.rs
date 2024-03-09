use anchor_lang::prelude::*;

declare_id!("Apkfrh7Bo1UegJLS78fNsgpunbh5bA6LmZSpKVHJXSN6");

#[program]
pub mod predix {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
