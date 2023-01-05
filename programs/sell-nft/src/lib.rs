use anchor_lang::{prelude::*, system_program};
use anchor_spl::{token};

declare_id!("HgdZU356qBiVDeNrEsuSbyQ4wGvYdUdhVJpirAFzaPtf");

#[program]
pub mod sell_nft {
    use anchor_lang::solana_program::{
        native_token::LAMPORTS_PER_SOL
    };

    use super::*;

    pub fn initialize(ctx: Context<SellNFT>, bump: u8) -> Result<()> {
        assert!(
            ctx.accounts.nft_mint.supply == 1 && ctx.accounts.nft_mint.mint_authority.is_none(),
            "The NFT should have 1 token supply and no mint authority"
        );
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_account.to_account_info(),
                    to: ctx.accounts.nft_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            1,
        )?;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.nft_pda.to_account_info(),
                    to: ctx.accounts.authority.to_account_info(),
                },
            )
            .with_signer(&[&[
                ctx.accounts.authority.key.as_ref(),
                b"nft_holder".as_ref(),
                &[bump],
            ][..]]),
            LAMPORTS_PER_SOL / 100000000,
        )?;
        Ok(())
    }

    pub fn get_back(ctx: Context<GetBack>) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer { from: ctx.accounts.authority.to_account_info(), to: ctx.accounts.nft_pda.to_account_info() },
            ),
            LAMPORTS_PER_SOL/100000000,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SellNFT<'info> {
    ///CHECK: We are goind to create this account
    #[account(mut, seeds = [authority.key.as_ref(), b"nft_holder".as_ref()], bump)]
    nft_pda: AccountInfo<'info>,
    ///CHECK:Using this account to extract the nft
    #[account(mut)]
    token_account: AccountInfo<'info>,
    #[account(mut)]
    nft_mint: Account<'info, token::Mint>,
    ///CHECK: Account that holds the NFT
    #[account(mut)]
    nft_token_account: AccountInfo<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct GetBack<'info> {
    ///CHECK: We are goind to create this account
    #[account(mut, seeds = [authority.key.as_ref(), b"nft_holder".as_ref()], bump)]
    nft_pda: AccountInfo<'info>,
    ///CHECK:Using this account to extract the nft
    #[account(mut)]
    token_account: AccountInfo<'info>,
    #[account(mut)]
    nft_mint: Account<'info, token::Mint>,
    ///CHECK: Account that holds the NFT
    #[account(mut)]
    nft_token_account: AccountInfo<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}
