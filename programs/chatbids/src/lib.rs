pub mod constants;
pub mod error;
pub mod pda;
pub mod processor;
pub mod state;
pub mod utils;

use crate::processor::{
    accept_direct_offer::*, close_direct_offer::*,
    create_direct_offer::*, fund_withdraw_pda_wallet::*,update_direct_offer::*
};
use anchor_lang::prelude::*;

declare_id!("ZzS7SCJrNP4qw7yjEM8WeKYNa1HtEPnFWokof6D5nku");

#[program]
pub mod chatbids {
    use super::*;

    pub fn fund_withdraw_pda_wallet(
        ctx: Context<FundWithdrawPdaWalletAccounts>,
        mode: u8,
        amount: u64,
    ) -> Result<()> {
        return process_fund_withdraw_pda_wallet(ctx, mode, amount);
    }

    pub fn create_direct_offer(
        ctx: Context<CreateDirectOfferAccounts>,
        offered_amount: u64,
        endtime: u64,
    ) -> Result<()> {
        return process_create_direct_offer(ctx, offered_amount, endtime);
    }

    pub fn close_direct_offer(ctx: Context<CloseDirectOfferAccounts>) -> Result<()> {
        return process_close_direct_offer(ctx);
    }

    pub fn update_direct_offer(ctx:Context<UpdateDirectOfferAccounts>,offered_amount: u64,endtime: u64) -> Result<()> {
       return process_update_offer(ctx, offered_amount, endtime);
    }

    pub fn accept_direct_offer<'info>(
        ctx: Context<'_, '_, '_, 'info, AcceptDirectOffer<'info>>,
        allowed_royalty: u16,
    ) -> Result<()> {
        return process_accept_direct_offer(ctx, allowed_royalty);
    }

}
