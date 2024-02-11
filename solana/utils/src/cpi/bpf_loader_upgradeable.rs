use anchor_lang::prelude::*;

pub use solana_program::bpf_loader_upgradeable::{self, id, ID};

pub fn set_upgrade_authority<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, SetUpgradeAuthority<'info>>,
    program_id: &Pubkey,
    new_authority: Option<&Pubkey>,
) -> Result<()> {
    solana_program::program::invoke_signed(
        &bpf_loader_upgradeable::set_upgrade_authority(
            program_id,
            &ctx.accounts.current_authority.key(),
            new_authority,
        ),
        &ctx.to_account_infos(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn set_upgrade_authority_checked<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, SetUpgradeAuthorityChecked<'info>>,
    program_id: &Pubkey,
) -> Result<()> {
    solana_program::program::invoke_signed(
        &bpf_loader_upgradeable::set_upgrade_authority_checked(
            program_id,
            &ctx.accounts.current_authority.key(),
            &ctx.accounts.new_authority.key(),
        ),
        &ctx.to_account_infos(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn upgrade<'info>(ctx: CpiContext<'_, '_, '_, 'info, Upgrade<'info>>) -> Result<()> {
    solana_program::program::invoke_signed(
        &bpf_loader_upgradeable::upgrade(
            &ctx.accounts.program.key(),
            &ctx.accounts.buffer.key(),
            &ctx.accounts.authority.key(),
            &ctx.accounts.spill.key(),
        ),
        &ctx.to_account_infos(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct SetUpgradeAuthority<'info> {
    #[account(mut)]
    pub program_data: AccountInfo<'info>,

    #[account(signer)]
    pub current_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetUpgradeAuthorityChecked<'info> {
    #[account(mut)]
    pub program_data: AccountInfo<'info>,

    #[account(signer)]
    pub current_authority: AccountInfo<'info>,

    #[account(signer)]
    pub new_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Upgrade<'info> {
    #[account(mut)]
    pub program: AccountInfo<'info>,

    #[account(mut)]
    pub program_data: AccountInfo<'info>,

    #[account(mut)]
    pub buffer: AccountInfo<'info>,

    #[account(signer)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub spill: AccountInfo<'info>,

    pub rent: AccountInfo<'info>,

    pub clock: AccountInfo<'info>,
}

#[derive(Debug, Clone)]
pub struct BpfLoaderUpgradeable;

impl anchor_lang::Id for BpfLoaderUpgradeable {
    fn id() -> Pubkey {
        ID
    }
}
