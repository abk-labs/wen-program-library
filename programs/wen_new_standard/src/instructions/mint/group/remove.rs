use anchor_lang::prelude::*;

use anchor_spl::token_interface::{
    group_member_pointer_update, GroupMemberPointerUpdate, Mint, Token2022,
};

use crate::{
    get_bump_in_seed_form, Manager, MintErrors, TokenGroup, TokenGroupMember, GROUP_ACCOUNT_SEED,
    MANAGER_SEED, MEMBER_ACCOUNT_SEED, TOKEN22,
};

#[derive(Accounts)]
#[instruction()]
pub struct RemoveGroup<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = group.update_authority == authority.key(),
        seeds = [GROUP_ACCOUNT_SEED, group.mint.as_ref()],
        bump,
    )]
    pub group: Account<'info, TokenGroup>,
    #[account(
        mut,
        has_one = mint @ MintErrors::InvalidTokenGroupMemberMint,
        seeds = [MEMBER_ACCOUNT_SEED, mint.key().as_ref()],
        bump,
    )]
    pub member: Account<'info, TokenGroupMember>,
    #[account(
        mut,
        mint::token_program = TOKEN22
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [MANAGER_SEED],
        bump
    )]
    pub manager: Box<Account<'info, Manager>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

impl RemoveGroup<'_> {
    fn update_group_member_pointer_member_address(&self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_accounts = GroupMemberPointerUpdate {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        group_member_pointer_update(cpi_ctx, None)?;
        Ok(())
    }
}

pub fn handler(ctx: Context<RemoveGroup>) -> Result<()> {
    let group = &mut ctx.accounts.group;
    group.decrement_size()?;

    let member = &mut ctx.accounts.member;
    member.close(ctx.accounts.payer.to_account_info())?;

    let signer_seeds = &[MANAGER_SEED, &get_bump_in_seed_form(&ctx.bumps.manager)];

    ctx.accounts
        .update_group_member_pointer_member_address(&[&signer_seeds[..]])?;

    Ok(())
}
