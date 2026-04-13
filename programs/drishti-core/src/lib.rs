use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;

use state::*;

declare_id!("F1V5vkLAv3e4XxER9ULW83FLnfcQEWsm5PHujy6S9pm2");

#[program]
pub mod drishti_core {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
