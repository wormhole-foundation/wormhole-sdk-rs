#![allow(clippy::result_large_err)]

pub mod bpf_loader_upgradeable;

pub mod system_program;

use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

pub fn invoke_data_with_context<'info, A, T>(
    data: T,
    ctx: CpiContext<'_, '_, '_, 'info, A>,
) -> Result<()>
where
    A: ToAccountMetas + ToAccountInfos<'info>,
    T: AnchorSerialize,
{
    invoke_raw_data_with_context(data.try_to_vec()?, ctx)
}

pub fn invoke_raw_data_with_context<'info, A>(
    data: Vec<u8>,
    ctx: CpiContext<'_, '_, '_, 'info, A>,
) -> Result<()>
where
    A: ToAccountMetas + ToAccountInfos<'info>,
{
    invoke_with_context(
        &Instruction {
            program_id: ctx.program.key(),
            data,
            accounts: ctx.to_account_metas(None),
        },
        ctx,
    )
}

pub fn invoke_with_context<'info, A>(
    ix: &Instruction,
    ctx: CpiContext<'_, '_, '_, 'info, A>,
) -> Result<()>
where
    A: ToAccountMetas + ToAccountInfos<'info>,
{
    solana_program::program::invoke_signed(ix, &ctx.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
