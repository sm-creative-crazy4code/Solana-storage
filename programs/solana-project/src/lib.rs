use anchor_lang::prelude::*;

declare_id!("FqoqX9JwoazhFRZPVXciW7NKkYLEkUJhy48kVFmPEUz7");

#[program]
pub mod solana_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let initial_account= &ctx.account.initial_account;
        initial_account.value =10;
        Ok(())
    }

    pub fn update_value(ctx: Context<UpdateValue>,value:u64) -> Result<()> {
        let storage_account = &ctx.account.storage_account;
        storage_account.value = value;
        Ok(())
}

#[derive(Accounts)]
pub struct Initialize<`info> {
#[account(init,player=user,space=9000)]
pub initial_account:Account<`info,Init>,
#[account(mut)]
pub user : Signer<`info>,
pub system_program : Program<`info,System>,
 



}



#[account]
pub struct Init{
    pub value: u64
}
