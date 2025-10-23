use anchor_lang::prelude::*;

declare_id!("8DYfoJLEghPutbU8fWN3e29D1UYb7pE1xkdXAEUbU4AP");

#[program]
pub mod calculator {
    use super::*;
    // the Initialize function
    pub fn initialize(ctx: Context<Initialize> , init_value: u32) -> Result<()> {
        ctx.accounts.account.num = init_value;
        Ok(())
    }
    // the Double funtion
    pub fn double(ctx: Context<Double>)-> Result<()>{
        ctx.accounts.account.num = ctx.accounts.account.num *2;
        Ok(())
    }

    // the Increment/Add function
    pub fn add(ctx : Context<Add> , num: u32)-> Result<()>{
        ctx.accounts.account.num = ctx.accounts.account.num +num;
        Ok(())
    }

}

pub struct DataShape{
    pub num : u32
}

// Initialization Struct
#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(init , payer = signer, space= 8+4)]
    pub account: Account<'info ,DataShape>,
    pub system_program : Program<'info, System>,
    #[account(mut)]
    signer : Signer<'info>
}

// the Double Struct
#[derive(Accounts)]
pub struct Double<'info>{
    #[account(mut)]
    pub account: Account<'info ,DataShape>,
    #[account(mut)]
    signer : Signer<'info>
}

// the Increment/Add Struct
#[derive(Accounts)]
pub struct Add<'info>{
    #[account(mut)]
    pub account: Account<'info ,DataShape>,
    #[account(mut)]
    signer : Signer<'info>
}
