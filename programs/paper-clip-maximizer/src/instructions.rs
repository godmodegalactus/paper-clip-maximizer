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


// Make paper clips will make paper clips from SOL.
// It will use 1 lamport per paper clip produced. This lamports will be transfered from source account to burn account
// if source account has enough lamports then we will rebate the called its application fees.
// if source account does not have enough SOLs 1 SOL application fee will be charged to extract all the resources required to create the paper clip
#[derive(Accounts)]
pub struct MakePaperClips<'info> {
    #[account(mut)]
    pub group: Box<Account<'info, PaperclipGroup>>,

    /// CHECK: source is a derivable address
    #[account( mut,
        seeds = [b"source", &group.key().to_bytes()],
        bump,)]
    pub source: AccountInfo<'info>,

    /// CHECK: burn is a derivable address
    #[account( mut,
        seeds = [b"burn", &group.key().to_bytes()],
        bump,)]
    pub burn: AccountInfo<'info>,
}
