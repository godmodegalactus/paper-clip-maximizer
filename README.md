# Paper clip Maximizer

This is an example to test application fees (writable account fees). These fees are applied when a owner of the account sets application fees using solana-application-fees-program to a particular account. They are applied when a transaction locks the relevant account in write mode. Check the SIMD 0016 for more information.

## Purpose

A paper clip maximizer group is created by an admin. This group can be used to create paper clips from lamports. \
Application fee to make paper clips is 1 SOLs if not enough sols are provided in Source account of the group. \
If users funds the source account before hand and then calls instruction to make the paper clips the application fees will be rebated. The lamports from source account will be transfered to burn account. \
The minimum balance of source and burn accounts are 1 SOL.
