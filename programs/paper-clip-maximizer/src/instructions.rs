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

    /// CHECK: authority pda for the group
    #[account (
        seeds = [
            b"authority" as &[u8],
            &group.to_account_info().key.to_bytes(),
        ],
        bump,
    )]
    pub authority: AccountInfo<'info>,

    /// CHECK: authority pda for the group
    #[account (
        seeds = [
            b"source" as &[u8],
            &group.to_account_info().key.to_bytes(),
        ],
        bump,
    )]
    pub source: AccountInfo<'info>,

    /// CHECK: authority pda for the group
    #[account (
        seeds = [
            b"burn" as &[u8],
            &group.to_account_info().key.to_bytes(),
        ],
        bump,
    )]
    pub burn: AccountInfo<'info>,

    /// CHECK: application fee pda to apply for group
    pub application_fees_pda: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: application fee program
    pub application_fees_program: AccountInfo<'info>,
}
