use crate::*;

use crate::states::PaperclipGroup;
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // admin / payer
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account( init,
        seeds = [ 
                    b"pcm_group" as &[u8],
                    &admin.key.to_bytes()
                ],
        bump,
        payer = admin,
        space = 8 + size_of::<PaperclipGroup>(),)]
    pub group: Box<Account<'info, PaperclipGroup>>,

    /// CHECK: application fee pda to apply for group
    #[account (mut)]
    pub application_fees_pda: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: application fee program
    pub application_fees_program: AccountInfo<'info>,
}
