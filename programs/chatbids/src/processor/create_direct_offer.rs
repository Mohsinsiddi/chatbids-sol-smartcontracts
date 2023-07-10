use crate::error::ProgramError;
use crate::utils::{assert_pubkey_equal_from_array, AssertPubkey};
use crate::{
    constants::MINIMUM_LISTING_AMOUNT,
    state::{DirectOfferState, DIRECT_OFFER_STATE_SIZE},
};
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct CreateDirectOfferAccounts<'info> { 
    #[account(init_if_needed, payer=buyer,seeds=["DirectOfferState".as_bytes(),buyer.key().as_ref(),nft_metadata.key().as_ref()],bump,space = DIRECT_OFFER_STATE_SIZE)]
    direct_offer_state: Account<'info, DirectOfferState>,
    /// CHECK:checked in the constraint
    nft_mint: AccountInfo<'info>,
    /// CHECK:checked in the constraint
    seller: AccountInfo<'info>,
    #[account(mut)]
    buyer: Signer<'info>,
    /// CHECK: checked in the constraint
    #[account(seeds=["metadata".as_bytes(),mpl_token_metadata::id().key().as_ref(),nft_mint.key().as_ref()],bump,seeds::program=mpl_token_metadata::id())]
    nft_metadata: AccountInfo<'info>,
    /// CHECK: checked in the constraint
    #[account(seeds=["UserPdaWallet".as_bytes(),buyer.key().as_ref()],bump)]
    buyer_pda_wallet: AccountInfo<'info>,
    seller_nft_ata: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
}

pub fn process_create_direct_offer(
    ctx: Context<CreateDirectOfferAccounts>,
    offered_amount: u64,
    endtime: u64,
) -> Result<()> {
    let direct_offer_state: &mut Account<'_, DirectOfferState> = &mut ctx.accounts.direct_offer_state;
    let buyer = &mut ctx.accounts.buyer;
    let nft_metadata = &mut ctx.accounts.nft_metadata;
    let buyer_pda_wallet = &mut ctx.accounts.buyer_pda_wallet;
    let seller = &mut ctx.accounts.seller;
    let seller_nft_ata = &mut ctx.accounts.seller_nft_ata;
    let nft_mint = &mut ctx.accounts.nft_mint;

    assert_pubkey_equal_from_array(vec![
        AssertPubkey {
            pubkey_one: seller_nft_ata.owner.key(),
            pubkey_two: seller.key(),
        },
        AssertPubkey {
            pubkey_one: seller_nft_ata.mint.key(),
            pubkey_two: nft_mint.key(),
        },
    ])?;

    if seller_nft_ata.amount != 1 {
        return Err(ProgramError::InsufficientFunds.into());
    }

    let clock: Clock = Clock::get()?;
    if endtime < clock.unix_timestamp as u64 {
        return Err(ProgramError::InvalidTimestamp.into());
    }

    if offered_amount < MINIMUM_LISTING_AMOUNT as u64 {
        return Err(ProgramError::InvalidOfferAmount.into());
    }

    if direct_offer_state.is_expired == true {
        return Err(ProgramError::InvalidOffer.into());
    }

    if buyer_pda_wallet.lamports() <= offered_amount {
        return Err(ProgramError::InsufficientFunds.into());
    }

    direct_offer_state.offered_amount = offered_amount;
    direct_offer_state.endtime = endtime;
    direct_offer_state.buyer = buyer.key();
    direct_offer_state.nft_metadata = nft_metadata.key();
    direct_offer_state.seller = seller.key();
    direct_offer_state.is_expired = false;
    Ok(())
}
