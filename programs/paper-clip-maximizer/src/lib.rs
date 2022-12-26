use anchor_lang::prelude::*;

mod states;

mod instructions;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// can be removed when solana_program and anchor::solana_program are same
fn change_pubkey_to_solana_program(
    pubkey: &anchor_lang::prelude::Pubkey,
) -> solana_program::pubkey::Pubkey {
    solana_program::pubkey::Pubkey::new(&pubkey.to_bytes()[..])
}

fn change_pubkey_to_anchor_program(
    pubkey: &solana_program::pubkey::Pubkey,
) -> anchor_lang::prelude::Pubkey {
    anchor_lang::prelude::Pubkey::new(&pubkey.to_bytes()[..])
}

fn convert_ix_to_anchor_ix(ix : solana_program::instruction::Instruction) -> anchor_lang::solana_program::instruction::Instruction {
    anchor_lang::solana_program::instruction::Instruction::new_with_bytes(
        change_pubkey_to_anchor_program(&ix.program_id), 
        &ix.data,
        ix.accounts.iter().map(|x| 
            if x.is_writable { 
                anchor_lang::prelude::AccountMeta::new(change_pubkey_to_anchor_program(&x.pubkey), x.is_signer)
            }
            else { 
                anchor_lang::prelude::AccountMeta::new_readonly(change_pubkey_to_anchor_program(&x.pubkey), x.is_signer)
            }
        ).collect()
     )
}

#[program]
pub mod paper_clip_maximizer {
    use anchor_lang::solana_program::{program::invoke_signed, native_token::LAMPORTS_PER_SOL};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let group = ctx.accounts.group.as_mut();
        let group_id = group.to_account_info().key();
        let admin_id = ctx.accounts.admin.key;
        let authority = ctx.accounts.authority.clone();
        let (group_bump, authority_bump, source_bump, burn_bump) =
            group.init(ctx.accounts.admin.key());

        let seeds_group = &[&b"pcm_group"[..], &admin_id.to_bytes(), &[group_bump]];

        let change_grp_owner_ix = anchor_lang::solana_program::system_instruction::assign(
            &group_id,
            authority.key,
        );
        invoke_signed(
            &change_grp_owner_ix,
            &[group.to_account_info().clone(), authority.clone()],
            &[seeds_group],
        )?;

        let source_seeds = &[&b"source"[..], &group_id.to_bytes(), &[source_bump]];

        let change_source_owner_ix = anchor_lang::solana_program::system_instruction::assign(
            &ctx.accounts.source.key(),
            authority.key,
        );
        invoke_signed(
            &change_source_owner_ix,
            &[ctx.accounts.source.clone(), authority.clone()],
            &[source_seeds],
        )?;


        let burn_seeds = &[&b"burn"[..], &group_id.to_bytes(), &[burn_bump]];

        let change_source_owner_ix = anchor_lang::solana_program::system_instruction::assign(
            &ctx.accounts.burn.key(),
            authority.key,
        );
        invoke_signed(
            &change_source_owner_ix,
            &[ctx.accounts.burn.clone(), authority.clone()],
            &[burn_seeds],
        )?;

        let ix_update_fees = solana_application_fees_program::instruction::update_fees( 
            1 * LAMPORTS_PER_SOL, 
            change_pubkey_to_solana_program(&group_id), 
            change_pubkey_to_solana_program(authority.key), 
            change_pubkey_to_solana_program(admin_id),
        );

        let auth_seeds = &[&b"authority"[..], &group_id.to_bytes(), &[authority_bump]];
        
        invoke_signed(
            &convert_ix_to_anchor_ix(ix_update_fees), 
            &[ctx.accounts.authority.clone(), group.to_account_info(), ctx.accounts.admin.to_account_info().clone(), ctx.accounts.application_fees_pda.clone(), ctx.accounts.system_program.to_account_info().clone()], 
            &[auth_seeds],
        )?;
        Ok(())
    }
}
