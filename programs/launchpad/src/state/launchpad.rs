use {anchor_lang::prelude::*, anchor_spl::token::Transfer};

#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct Fee {
    numerator: u64,
    denominator: u64,
}

#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct Fees {
    pub new_auction: Fee,
    pub auction_update: Fee,
    pub invalid_bid: Fee,
    pub trade: Fee,
}

#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct CollectedFees {
    pub new_auction_sol: u64,
    pub auction_update_sol: u64,
    pub invalid_bid_usdc: u64,
    pub trade_usdc: u64,
}

#[derive(Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct Permissions {
    pub allow_new_auctions: bool,
    pub allow_auction_updates: bool,
    pub allow_new_bids: bool,
    pub allow_withdrawals: bool,
}

#[account]
#[derive(Default, Debug)]
pub struct Launchpad {
    pub permissions: Permissions,
    pub fees: Fees,
    pub collected_fees: CollectedFees,
    pub transfer_authority_bump: u8,
    pub launchpad_bump: u8,
}

impl anchor_lang::Id for Launchpad {
    fn id() -> Pubkey {
        crate::ID
    }
}

impl Launchpad {
    pub const LEN: usize = 8 + std::mem::size_of::<Launchpad>();

    pub fn validate(&self) -> bool {
        self.fees.new_auction.numerator < self.fees.new_auction.denominator
            && self.fees.auction_update.numerator < self.fees.auction_update.denominator
            && self.fees.invalid_bid.numerator < self.fees.invalid_bid.denominator
            && self.fees.trade.numerator < self.fees.trade.denominator
    }

    pub fn transfer_tokens<'info>(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        let authority_seeds: &[&[&[u8]]] =
            &[&[b"transfer_authority", &[self.transfer_authority_bump]]];

        let context = CpiContext::new(
            token_program,
            Transfer {
                from,
                to,
                authority,
            },
        )
        .with_signer(authority_seeds);

        anchor_spl::token::transfer(context, amount)
    }
}