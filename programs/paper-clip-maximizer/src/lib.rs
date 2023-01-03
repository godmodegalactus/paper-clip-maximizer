use anchor_lang::prelude::*;

mod states;

mod instructions;

use instructions::*;

declare_id!("2m9g7TYbAcZwcT3TSxHh4czMS6s7jdNqGpPeNi378XeL");

#[error_code]
pub enum PaperclipMaximizerErrors {
    #[msg("Cannot process such a huge request")]
    MakeClipLimit,
}

#[program]
pub mod paper_clip_maximizer {
    use anchor_lang::solana_program::{native_token::LAMPORTS_PER_SOL, program::invoke_signed};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let group = ctx.accounts.group.as_mut();
        let group_id = group.to_account_info().key();
        let admin_id = ctx.accounts.admin.key;
        let group_bump = group.init(ctx.accounts.admin.key());

        // allocate 1 SOL to source
        let transfer_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.admin.to_account_info().clone(),
            to: ctx.accounts.source.clone(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        anchor_lang::system_program::transfer(cpi_context, LAMPORTS_PER_SOL)?;

        // allocate 1 SOL to burn
        let transfer_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.admin.to_account_info().clone(),
            to: ctx.accounts.burn.clone(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );
        anchor_lang::system_program::transfer(cpi_context, LAMPORTS_PER_SOL)?;

        let ix_update_fees = solana_application_fees_program::instruction::update_fees(
            1 * LAMPORTS_PER_SOL,
            group_id,
            group_id,
            *admin_id,
        );

        let seeds_group: &[&[u8]] = &[b"pcm_group", &admin_id.to_bytes(), &[group_bump]];

        invoke_signed(
            &ix_update_fees,
            &[
                group.to_account_info().clone(),
                ctx.accounts.admin.to_account_info().clone(),
                ctx.accounts.application_fees_pda.clone(),
                ctx.accounts.system_program.to_account_info().clone(),
                ctx.accounts
                    .application_fees_program
                    .to_account_info()
                    .clone(),
            ],
            &[seeds_group],
        )?;
        Ok(())
    }

    pub fn make_paper_clips(
        ctx: Context<MakePaperClips>,
        number_of_paper_clips_wanted: u64,
    ) -> Result<()> {
        if number_of_paper_clips_wanted > LAMPORTS_PER_SOL {
            // too many paper clips wanted / limit reached
            return Err(PaperclipMaximizerErrors::MakeClipLimit.into());
        }

        let group = &mut ctx.accounts.group;
        assert!(ctx.accounts.burn.key.eq(&group.burn_account));
        assert!(ctx.accounts.source.key.eq(&group.source_account));
        group.number_of_paper_clips_created += number_of_paper_clips_wanted;

        let admin = group.admin;
        let (group_pk, group_bump) =
            Pubkey::find_program_address(&[b"pcm_group" as &[u8], &admin.to_bytes()], &crate::id());
        assert!(group_pk.eq(&group.key()));

        if ctx.accounts.source.lamports() > number_of_paper_clips_wanted + LAMPORTS_PER_SOL {
            let seeds_group: &[&[u8]] = &[b"pcm_group", &admin.to_bytes(), &[group_bump]];
            let (_source, source_bump) =
                Pubkey::find_program_address(&[b"source", &group.key().to_bytes()], &crate::id());
            let seeds_source: &[&[&[u8]]] =
                &[&[b"source", &group.key().to_bytes(), &[source_bump]]];
            let transfer_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.source.clone(),
                to: ctx.accounts.burn.clone(),
            };
            let cpi_context = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                transfer_accounts,
                seeds_source,
            );
            anchor_lang::system_program::transfer(cpi_context, number_of_paper_clips_wanted)?;

            // rebate if there were enough sols
            let ix_rebate = solana_application_fees_program::instruction::rebate(
                group.key(),
                group.key(),
                ctx.accounts.payer.key(),
            );
            invoke_signed(
                &ix_rebate,
                &[
                    ctx.accounts.group.to_account_info().clone(),
                    ctx.accounts.application_fees_program.clone(),
                    ctx.accounts.payer.to_account_info().clone(),
                ],
                &[seeds_group],
            )?;
        }
        Ok(())
    }
}
