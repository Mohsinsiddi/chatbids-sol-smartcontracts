use crate::state::DirectOfferState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseDirectOfferAccounts<'info> {
    #[account(mut,seeds=["DirectOfferState".as_bytes(),buyer.key().as_ref(),nft_metadata.key().as_ref()],bump,close = buyer)]
    direct_offer_state: Account<'info, DirectOfferState>,
    /// CHECK:checked in the constraint
    nft_mint: AccountInfo<'info>,
    #[account(mut)]
    buyer: Signer<'info>,
    /// CHECK: checked in the constraint
    #[account(seeds=["metadata".as_bytes(),mpl_token_metadata::id().key().as_ref(),nft_mint.key().as_ref()],bump,seeds::program=mpl_token_metadata::id())]
    nft_metadata: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

pub fn process_close_direct_offer(_ctx: Context<CloseDirectOfferAccounts>) -> Result<()> {
    Ok(())
}
