use anchor_lang::prelude::*;

mod states;

mod instructions;

use instructions::*;

declare_id!("m1MbXbpDJxJGrysnftRawJYbnd9GvVJ5bLrQVA27wbw");

#[program]
pub mod paper_clip_maximizer {
    use anchor_lang::solana_program::{program::invoke_signed, native_token::LAMPORTS_PER_SOL};
    use solana_program::system_instruction;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let group = ctx.accounts.group.as_mut();
        let group_id = group.to_account_info().key();
        let admin_id = ctx.accounts.admin.key;
        let group_bump = group.init(ctx.accounts.admin.key());

        let ix_update_fees = solana_application_fees_program::instruction::update_fees( 
            1 * LAMPORTS_PER_SOL, 
            group_id, 
            group_id,  
            *admin_id,
        );

        let seeds_group : &[&[u8]] = &[b"pcm_group", &admin_id.to_bytes(), &[group_bump]];
        
        invoke_signed(
            &ix_update_fees, 
            &[group.to_account_info(), ctx.accounts.admin.to_account_info().clone(), ctx.accounts.application_fees_pda.clone(), ctx.accounts.system_program.to_account_info().clone(), ctx.accounts.application_fees_program.to_account_info().clone()], 
            &[seeds_group],
        )?;
        Ok(())
    }
}
