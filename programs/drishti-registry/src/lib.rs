use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;

declare_id!("2uAdCz8GAxynicss9aWpvH7wDRHvVPPUF5hpqECpEuEN");

#[program]
pub mod drishti_registry {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
