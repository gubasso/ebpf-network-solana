use anchor_lang::prelude::*;

declare_id!("CPy4XsWvCfZyjjfx3Uymf2y6BfN7vUHczb9dLYmPvSJc");

#[program]
pub mod ebpf_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
