use anchor_lang::prelude::*;

pub use solana_program::bpf_loader_upgradeable::{self, id, ID};

#[derive(Debug, Clone)]
pub struct BpfLoaderUpgradeable;

impl anchor_lang::Id for BpfLoaderUpgradeable {
    fn id() -> Pubkey {
        ID
    }
}

pub use __private::{
    set_buffer_authority, set_buffer_authority_checked, set_upgrade_authority,
    set_upgrade_authority_checked, upgrade, SetBufferAuthority, SetBufferAuthorityChecked,
    SetUpgradeAuthority, SetUpgradeAuthorityChecked, Upgrade,
};

mod __private {
    use super::*;

    pub fn set_upgrade_authority<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, SetUpgradeAuthority<'info>>,
        program_id: &Pubkey,
    ) -> Result<()> {
        solana_program::program::invoke_signed(
            &bpf_loader_upgradeable::set_upgrade_authority(
                program_id,
                ctx.accounts.current_authority.key,
                ctx.accounts.new_authority.as_ref().map(|a| a.key),
            ),
            &ctx.to_account_infos(),
            ctx.signer_seeds,
        )
        .map_err(Into::into)
    }

    pub struct SetUpgradeAuthority<'info> {
        pub program_data: AccountInfo<'info>,
        pub current_authority: AccountInfo<'info>,
        pub new_authority: Option<AccountInfo<'info>>,
    }

    impl ToAccountMetas for SetUpgradeAuthority<'_> {
        fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
            vec![]
            // match &self.new_authority {
            //     Some(new_authority) => vec![
            //         AccountMeta::new(*self.program_data.key, false),
            //         AccountMeta::new(*self.current_authority.key, true),
            //         AccountMeta::new(*new_authority.key, true),
            //     ],
            //     None => vec![
            //         AccountMeta::new(*self.program_data.key, false),
            //         AccountMeta::new(*self.current_authority.key, true),
            //     ],
            // }
        }
    }

    impl<'info> ToAccountInfos<'info> for SetUpgradeAuthority<'info> {
        fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
            match &self.new_authority {
                Some(new_authority) => vec![
                    self.program_data.clone(),
                    self.current_authority.clone(),
                    new_authority.clone(),
                ],
                None => vec![self.program_data.clone(), self.current_authority.clone()],
            }
        }
    }

    pub fn set_upgrade_authority_checked<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, SetUpgradeAuthorityChecked<'info>>,
        program_id: &Pubkey,
    ) -> Result<()> {
        solana_program::program::invoke_signed(
            &bpf_loader_upgradeable::set_upgrade_authority_checked(
                program_id,
                ctx.accounts.current_authority.key,
                ctx.accounts.new_authority.key,
            ),
            &ctx.to_account_infos(),
            ctx.signer_seeds,
        )
        .map_err(Into::into)
    }

    #[derive(Accounts)]
    pub struct SetUpgradeAuthorityChecked<'info> {
        pub program_data: AccountInfo<'info>,
        pub current_authority: AccountInfo<'info>,
        pub new_authority: AccountInfo<'info>,
    }

    pub fn set_buffer_authority<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, SetBufferAuthority<'info>>,
    ) -> Result<()> {
        solana_program::program::invoke_signed(
            &bpf_loader_upgradeable::set_buffer_authority(
                ctx.accounts.buffer.key,
                ctx.accounts.current_authority.key,
                ctx.accounts.new_authority.key,
            ),
            &ctx.to_account_infos(),
            ctx.signer_seeds,
        )
        .map_err(Into::into)
    }

    #[derive(Accounts)]
    pub struct SetBufferAuthority<'info> {
        pub buffer: AccountInfo<'info>,
        pub current_authority: AccountInfo<'info>,
        pub new_authority: AccountInfo<'info>,
    }

    pub fn set_buffer_authority_checked<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, SetBufferAuthorityChecked<'info>>,
    ) -> Result<()> {
        solana_program::program::invoke_signed(
            &bpf_loader_upgradeable::set_buffer_authority_checked(
                ctx.accounts.buffer.key,
                ctx.accounts.current_authority.key,
                ctx.accounts.new_authority.key,
            ),
            &ctx.to_account_infos(),
            ctx.signer_seeds,
        )
        .map_err(Into::into)
    }

    #[derive(Accounts)]
    pub struct SetBufferAuthorityChecked<'info> {
        pub buffer: AccountInfo<'info>,
        pub current_authority: AccountInfo<'info>,
        pub new_authority: AccountInfo<'info>,
    }

    pub fn upgrade<'info>(ctx: CpiContext<'_, '_, '_, 'info, Upgrade<'info>>) -> Result<()> {
        solana_program::program::invoke_signed(
            &bpf_loader_upgradeable::upgrade(
                ctx.accounts.program.key,
                ctx.accounts.buffer.key,
                ctx.accounts.authority.key,
                ctx.accounts.spill.key,
            ),
            &ctx.to_account_infos(),
            ctx.signer_seeds,
        )
        .map_err(Into::into)
    }

    #[derive(Accounts)]
    pub struct Upgrade<'info> {
        pub program: AccountInfo<'info>,
        pub program_data: AccountInfo<'info>,
        pub buffer: AccountInfo<'info>,
        pub authority: AccountInfo<'info>,
        pub spill: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
    }
}
