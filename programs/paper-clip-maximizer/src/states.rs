use crate::*;

#[account()]
pub struct PaperclipGroup {
    pub admin: Pubkey,

    // source account / account used to make paper clips
    pub source_account: Pubkey,

    // burn account / account representing SOLs used to generate paper clip
    pub burn_account: Pubkey,

    pub number_of_paper_clips_created: u64,
}

impl PaperclipGroup {
    pub fn init(&mut self, admin: Pubkey,) -> u8 {

        let (group_pk, group_bump) =
            Pubkey::find_program_address(&[b"pcm_group" as &[u8], &admin.to_bytes()], &crate::id());

        let (source, _source_bump) =
            Pubkey::find_program_address(&[b"source", &group_pk.to_bytes()], &crate::id());

        let (burn, _burn_bump) =
            Pubkey::find_program_address(&[b"burn", &group_pk.to_bytes()], &crate::id());

        self.admin = admin;
        self.burn_account = burn;
        self.source_account = source;
        self.number_of_paper_clips_created = 0;
        group_bump
    }
}
