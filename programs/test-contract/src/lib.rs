use anchor_lang::prelude::*;

declare_id!("61tgd4VSLsoM53qWgdhGkMrYQeaXB7HiYSJrCPVGvjhd");

#[program]
pub mod test_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
