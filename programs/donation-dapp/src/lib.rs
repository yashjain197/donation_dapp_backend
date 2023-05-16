use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("BNLeVGT6zY4QBQfJe1NV1PFUQHik3rNyBm8d9A2fNnCY");

#[program]
pub mod donation_dapp {
    use super::*;

    pub fn create(ctx: Context<Create>, name:String, description:String) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        let count = &mut ctx.accounts.count;
        count.count +=1;
        campaign.name = name;
        campaign.description= description;
        campaign.amount_received = 0;
        campaign.admin = *ctx.accounts.user.key;
    
        Ok(())
    }

    pub fn initialize_counter(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Initialized new count. Current value: {}!", counter.count);
        Ok(())
    }

    pub fn end_campaign(ctx: Context<EndCampaign>, amount:u64) -> ProgramResult{

        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;
        if campaign.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        } 

        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        
        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount; 
        **user.to_account_info().try_borrow_mut_lamports()? += amount; 
        Ok(())

    }

     pub fn pay(ctx: Context<Pay>, amount: u64) -> ProgramResult{

        let (pda, bump_seed) = Pubkey::find_program_address(&[b"DONATION_DAPP".as_ref(), &ctx.accounts.campaign.key().to_bytes()],ctx.program_id);

        if pda != ctx.accounts.pda.key(){
            return Err(ProgramError::AccountAlreadyInitialized)
        }
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &pda,
            amount  
        );
       let invoke =  anchor_lang::solana_program::program::invoke(
            &ix,
            &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.pda.to_account_info()
            ],
            
        );

        if invoke.err().is_some() {
            return Err(ProgramError::InvalidArgument);
        }

        (&mut ctx.accounts.campaign).amount_received += amount;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Create<'info>{
    #[account(mut)]
    pub count: Account<'info, Counter>,
    #[account{
        init,
        payer=user,
        space=500,
        seeds=[b"DONATION_DAPP".as_ref(),&[count.count],user.key().as_ref(),],
        bump}]
     pub campaign: Account<'info, Campaign>,
     #[account(mut)]
     pub user: Signer<'info>,
     pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndCampaign<'info>{
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>
} 

#[derive(Accounts)]
pub struct Pay<'info>{
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pda: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>
} 

#[account]
pub struct Campaign{
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_received: u64,
    // pub count: Count,
}

#[account]
pub struct Counter {
    count: u8
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}