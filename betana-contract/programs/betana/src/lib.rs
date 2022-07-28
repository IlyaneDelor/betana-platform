use anchor_lang::prelude::*;
use anchor_lang::account;
use anchor_lang::solana_program::entrypoint::ProgramResult; 
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");



pub mod betana {
 

    use anchor_lang::solana_program::{program::invoke,system_instruction::transfer};
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }


 

    pub fn claim_deposit_fund(ctx: Context<SendSol>, transfer_amount: String) -> ProgramResult {

        let base_account = &mut ctx.accounts.base_account;


        let stake_bal: u64 = transfer_amount.parse().unwrap();

        let total_amount = stake_bal as u64;
       
        if total_amount > 0 {
            let ix = &transfer(
                &ctx.accounts.from.key(),
                &ctx.accounts.to.key(),
                total_amount
            );
            invoke(
                &ix,
                &[
                    ctx.accounts.from.to_account_info(),
                    ctx.accounts.to.to_account_info()
                ]
            );
        }
        
        Ok(())
                                                                                                                                                                
    }





    /* 

    [#derive(BTreeMap<K, V>)]
    pub struct MATCHAMOUNT  <K, V>   {
        id_match: K,
        amount: V,

    }

    
        static MATCHAMOUNT : BTreeMap<u64, u64> = MATCHAMOUNT { id_match: 52, amount: 22 };
         
        pub(crate) static mut parieSur = BTreeMap::<u64, Pubkey>::new();
        pub(crate) static mut userAmountMatches = BTreeMap::<BTreeMap<u64, Pubkey>, u64>::new();*/
        

        pub fn create_match(
            ctx: Context<BetMatch>,
            id_match : String,
            id_away_team : String,
            id_home_team : String,
            bets : Vec<BetStruct>
        ) -> ProgramResult {
        let matches = &mut ctx.accounts.matches;
        
        //Build the struct.
        matches.id_match = id_match;
        matches.id_away_team = id_away_team;
        matches.id_home_team = id_home_team;
        matches.bets = bets;

        Ok(())
    }
    

    pub fn place_bet(
        ctx: Context<SendSol>, 
        ctxmatch: Context<BetMatch>,
        id_team : String,
        amount : u64
    ) -> ProgramResult {

        let matches = &mut ctxmatch.accounts.matches;

        let base_account = &mut ctx.accounts.base_account;
    

        //Build the struct.

        let bet_item = BetStruct {
         
            team: id_team,
            amount: amount,
            user_address: *ctx.accounts.from.to_account_info().key,
        };

        matches.bets.push(bet_item);

        

      //  *ctxmatch.bets.push(bet_item);

/* 
        base_account.current_bet = bet_item;
        MATCHAMOUNT.insert(id_match,amount);

       
        parieSur.insert(id_match,user_address);
        userAmountMatches.insert(parieSur,amount);*/

        let transfer_amount = amount as u64;

        if transfer_amount > 0 {
            let ix = &transfer(
                &ctx.accounts.from.key(),
                &ctx.accounts.to.key(),
                transfer_amount
            );
            
            invoke(
                &ix,
                &[
                    ctx.accounts.from.to_account_info(),
                    ctx.accounts.to.to_account_info()
                ],
                
            );
        }

        Ok(())
    }
}




#[derive(Accounts)]
pub struct BetanaPlatform<'info> {
    #[account(init, payer = user, space = 9000 )]
    pub matches: Account<'info, MatchStruct>,
    pub bet: Account<'info, BetStruct>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct BaseAccount {
    pub current_bet: BetStruct,
}



#[derive(Accounts)]
pub struct BetMatch<'info> {
    #[account(mut)]
    pub matches: Account<'info, MatchStruct>
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct BetStruct {
    pub team: String, 
    pub amount: u64,
    pub user_address: Pubkey,
}




#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct MatchStruct {
    
    pub id_match: String,
    
    pub id_away_team: String,
    pub id_home_team: String,
    pub bets : Vec<BetStruct>
}

