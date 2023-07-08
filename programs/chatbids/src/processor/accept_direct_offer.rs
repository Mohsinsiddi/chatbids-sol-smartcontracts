use std::str::FromStr;

use crate::constants::CREDIT_DENOMINATOR_FACTOR;
use crate::constants::HISTORY_MANAGER;
use crate::constants::OCCS_MANAGER;
use crate::constants::STAKING_VAULT;
use crate::constants::TEAM_MULTISIG_VAULT;
use crate::constants::TRADERS_VAULT;
use crate::error;
use crate::pda::get_user_pda_wallet;
use crate::state::DirectOfferState;
use crate::state::{UserTradeStateMetadata, USER_TRADE_STATE_METADATA_SIZE};
use crate::utils;
use crate::utils::{assert_pubkey_equal_from_array, AssertPubkey};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::token::{Token, TokenAccount};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

#[derive(Accounts)]
pub struct AcceptDirectOffer<'info> {
    #[account(mut,seeds=["DirectOfferState".as_bytes(),buyer.key().as_ref(),nft_metadata.key().as_ref()],bump)]
    direct_offer_state: Box<Account<'info, DirectOfferState>>,
    #[account(init_if_needed,payer=seller,seeds=[b"UserTradeStateMetadata",seller_every_day_trade_state.key().as_ref()],bump,space =USER_TRADE_STATE_METADATA_SIZE)]
    seller_trade_state_metadata: Box<Account<'info, UserTradeStateMetadata>>,
    #[account(init_if_needed,payer=seller,seeds=[b"UserTradeStateMetadata",buyer_every_day_trade_state.key().as_ref()],bump,space =USER_TRADE_STATE_METADATA_SIZE)]
    buyer_trade_state_metadata: Box<Account<'info, UserTradeStateMetadata>>,
    #[account(mut)]
    seller: Signer<'info>,
    /// CHECK:checked in the constaint
    #[account(mut)]
    buyer: AccountInfo<'info>,
    /// CHECK: checked in the implementation
    nft_metadata: AccountInfo<'info>,
    /// CHECK: checked in the constraint
    #[account(mut,seeds=["UserPdaWallet".as_bytes(),buyer.key().as_ref()],bump)]
    buyer_pda_wallet: AccountInfo<'info>,
    #[account(mut)]
    seller_nft_ata: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    buyer_nft_ata: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK:checked in implementation
    team_multisig_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:checked in implementation
    trader_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:checked in implementation
    staking_vault: AccountInfo<'info>,
    system_program: Program<'info, System>,
    token_program_id: Program<'info, Token>,

    /// CHECK:checked in implementation
    program_access_state: AccountInfo<'info>,

    /// CHECK:checked in implementation
    instructions: AccountInfo<'info>,

    // -------------history_accounts--------------
    /// CHECK:checked in implementation
    history_manager: AccountInfo<'info>,
    /// CHECK:checked in implementation
    #[account(mut)]
    seller_every_day_trade_state: AccountInfo<'info>,
    /// CHECK:checked in implementation
    #[account(mut)]
    buyer_every_day_trade_state: AccountInfo<'info>,
    /// CHECK:checked in implementation
    #[account(mut)]
    market_every_day_trade_state: AccountInfo<'info>,

    // -------------occs_accounts--------------
    /// CHECK:checked in implementation
    occs_manager: AccountInfo<'info>,

    /// CHECK:checked in implementation
    #[account(mut)]
    seller_occs_state: AccountInfo<'info>,
    /// CHECK:checked in implementation
    #[account(mut)]
    buyer_occs_state: AccountInfo<'info>,
}

pub fn process_accept_direct_offer<'info>(
    ctx: Context<'_, '_, '_, 'info, AcceptDirectOffer<'info>>,
    allowed_royalty: u16,
    end_day_timestamp: u64,
) -> Result<()> {
    let direct_offer_state: &mut Box<Account<'_, DirectOfferState>> = &mut ctx.accounts.direct_offer_state;
    let seller_nft_ata = &mut ctx.accounts.seller_nft_ata;
    let buyer_nft_ata = &mut ctx.accounts.buyer_nft_ata;
    let seller = &mut ctx.accounts.seller;
    let buyer = &mut ctx.accounts.buyer;
    let token_program_id = &mut ctx.accounts.token_program_id;
    let buyer_pda_wallet = &mut ctx.accounts.buyer_pda_wallet   ;
    let nft_metadata = &mut ctx.accounts.nft_metadata;

    let team_multisig_vault = &mut ctx.accounts.team_multisig_vault;
    let trader_vault = &mut ctx.accounts.trader_vault;
    let staking_vault = &mut ctx.accounts.staking_vault;
    let remaining_accounts = ctx.remaining_accounts;

    let program_access_state = &mut ctx.accounts.program_access_state;
    let instructions = &mut ctx.accounts.instructions;
    let history_manager = &mut ctx.accounts.history_manager;
    let seller_every_day_trade_state = &mut ctx.accounts.seller_every_day_trade_state;
    let buyer_every_day_trade_state = &mut ctx.accounts.buyer_every_day_trade_state;
    let market_every_day_trade_state = &mut ctx.accounts.market_every_day_trade_state;
    let occs_manager = &mut ctx.accounts.occs_manager;
    let seller_occs_state = &mut ctx.accounts.seller_occs_state;
    let buyer_occs_state = &mut ctx.accounts.buyer_occs_state;
    let system_program = &mut ctx.accounts.system_program;

    let (_buyer_pda_wallet_pubkey, pda_bump) = get_user_pda_wallet(buyer.key());

    let data = Metadata::from_account_info(&nft_metadata)?;

    assert_pubkey_equal_from_array(vec![
        AssertPubkey {
            pubkey_one: seller_nft_ata.owner.key(),
            pubkey_two: seller.key(),
        },
        AssertPubkey {
            pubkey_one: buyer_nft_ata.owner.key(),
            pubkey_two: buyer.key(),
        },
        AssertPubkey {
            pubkey_one: buyer_nft_ata.mint.key(),
            pubkey_two: data.mint.key(),
        },
        AssertPubkey {
            pubkey_one: seller_nft_ata.mint.key(),
            pubkey_two: data.mint.key(),
        },
        utils::AssertPubkey {
            pubkey_one: Pubkey::from_str(HISTORY_MANAGER).unwrap(),
            pubkey_two: history_manager.key(),
        },
        utils::AssertPubkey {
            pubkey_one: Pubkey::from_str(OCCS_MANAGER).unwrap(),
            pubkey_two: occs_manager.key(),
        },
        utils::AssertPubkey {
            pubkey_one: Pubkey::from_str(TEAM_MULTISIG_VAULT).unwrap(),
            pubkey_two: team_multisig_vault.key(),
        },
        utils::AssertPubkey {
            pubkey_one: Pubkey::from_str(TRADERS_VAULT).unwrap(),
            pubkey_two: trader_vault.key(),
        },
        utils::AssertPubkey {
            pubkey_one: Pubkey::from_str(STAKING_VAULT).unwrap(),
            pubkey_two: staking_vault.key(),
        },
    ])?;

    if allowed_royalty > data.data.seller_fee_basis_points {
        return Err(error::ProgramError::InvalidOptionRoyaltyValue.into());
    }

    let clock = Clock::get()?;

    if clock.unix_timestamp as u64 > direct_offer_state.endtime {
        return Err(error::ProgramError::OfferExpired.into());
    }


    utils::transfer_tokens(
        &buyer_nft_ata.to_account_info(),
        &token_program_id.to_account_info(),
        &seller_nft_ata.to_account_info(),
        1,
        &seller.to_account_info(),
        &[],
    )?;

    utils::distribute_amount(
        direct_offer_state.offered_amount,
        allowed_royalty,
        &seller.to_account_info(),
        &buyer_pda_wallet.clone(),
        &team_multisig_vault.clone(),
        &trader_vault.clone(),
        &staking_vault.clone(),
        &nft_metadata.clone(),
        &remaining_accounts.clone(),
        &[
            "UserPdaWallet".as_bytes(),
            buyer.key().as_ref(),
            &[pda_bump],
        ],
    )?;


    direct_offer_state.is_expired = true;
    Ok(())
}
