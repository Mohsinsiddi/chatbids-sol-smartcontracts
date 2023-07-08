pub mod constants;
pub mod error;
pub mod pda;
pub mod processor;
pub mod state;
pub mod utils;

use crate::processor::{
    accept_direct_offer::*, close_direct_offer::*,
    create_direct_offer::*, fund_withdraw_pda_wallet::*,
    initialize::*
};
use anchor_lang::prelude::*;

declare_id!("855UzS1Q9FPctVzgUzL8k4P7dT3aAdF3k7uoXDdWUvEV");

#[program]
pub mod chatbids {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()>{
        return process_initialize(_ctx);
    }

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

    pub fn accept_direct_offer<'info>(
        ctx: Context<'_, '_, '_, 'info, AcceptDirectOffer<'info>>,
        allowed_royalty: u16,
        end_day_timestamp: u64,
    ) -> Result<()> {
        return process_accept_direct_offer(ctx, allowed_royalty, end_day_timestamp);
    }

}
