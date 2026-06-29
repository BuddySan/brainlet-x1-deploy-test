use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod brainlet_test {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("brainlet test program initialized");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
