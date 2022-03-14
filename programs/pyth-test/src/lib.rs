use anchor_lang::prelude::*;
// use anchor_lang::solana_program::program::{invoke, invoke_signed};
// use anchor_lang::solana_program::system_instruction;
// use anchor_spl::token::{self, CloseAccount, Mint, MintTo, SetAuthority, TokenAccount, Transfer};
use pyth_client::{
    self, load_mapping, load_price, load_product, CorpAction, Price, PriceStatus, PriceType,
    Product,
};

declare_id!("B8Qq8KxxLAjFzgBLmJcJ8nfQw5WHugw1ZhCPFabqyb8u");

#[program]
pub mod pyth_test {
    use super::*;

    pub fn get_sol_price(ctx: Context<SolPrice>) -> Result<()> {
        let pyth_price_info = &ctx.accounts.pyth_account;
        let pyth_price_data = &pyth_price_info.try_borrow_data()?;
        let price_account: Price = *load_price(pyth_price_data).unwrap();

        msg!("SOL PRICE {}", price_account.agg.price);
        msg!(
            "Get the current status of the aggregate price {:?}",
            price_account.get_current_price_status()
        );
        let sol_price = (price_account.agg.price as u64 / (10u32.pow(8) as u64));
        msg!("SOL/USDC price without decimals {}", sol_price);
        msg!("Some more Logs :) {:?}", price_account.get_current_price());

        Ok(())
    }

    // pub fn get_sol_symbol(ctx: Context<SolSymbol>) -> Result<()> {
    //     let pyth_product_info = &ctx.accounts.pyth_account;
    //     let pyth_product_data = &pyth_product_info.try_borrow_data()?;
    //     let price_account: Product = *load_product(pyth_product_data).unwrap();

    //     msg!("DATA {:?}", price_account.attr);
    //     msg!("DATA {:?}", price_account.size);

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct SolPrice<'info> {
    #[account(mut)]
    pub user_account: Signer<'info>,
    /// CHECK:
    pub pyth_account: AccountInfo<'info>,
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

// #[derive(Accounts)]
// pub struct SolSymbol<'info> {
//     #[account(mut)]
//     pub user_account: Signer<'info>,
//     /// CHECK:
//     pub pyth_account: AccountInfo<'info>,
//     /// CHECK:
//     pub system_program: AccountInfo<'info>,
//     pub rent: Sysvar<'info, Rent>,
// }

#[account]
#[derive(Default)]
pub struct SolPriceAccount {
    pub last_sol_price: u64,
    pub stable_total_supply: u64,
}

#[error_code]
pub enum StableTestError {
    #[msg("User does not have enough SOL")]
    NoEnough,
    #[msg("The SOL/USD price is wrong.")]
    UsdPriceWrong,
}
