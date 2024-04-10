use anchor_lang::prelude::*;

declare_id!("CPy4XsWvCfZyjjfx3Uymf2y6BfN7vUHczb9dLYmPvSJc");

#[program]
pub mod ebpf_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, date: String) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        require!(date.len() == 10, ErrorCode::InvalidDate);
        data_account.date = date.clone();
        msg!("Date stored: {}", date);
        Ok(())
    }
}

#[account]
pub struct DataAccount {
    pub date: String, // YYYY-MM-DD
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)] // 8 bytes for the discriminator + 10 bytes for the date + 22 bytes for the string overhead
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided date is not valid.")]
    InvalidDate,
}
