use anchor_lang::prelude::*;

declare_id!("4pgDtkZUeAF7LmVmwJcJyBMmWB3dKFX7yiEGmL2Qjcey");

#[program]
pub mod drishti {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
