import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";
import { PaperClipMaximizer } from "../target/types/paper_clip_maximizer";
import * as logger from "mocha-logger"
import struct from "buffer-layout"
import { assert } from "chai";

const applicationFeesProgram = new web3.PublicKey('App1icationFees1111111111111111111111111111')
type PaperClipMaximizerGroup = anchor.IdlAccounts<PaperClipMaximizer>["paperclipGroup"];

describe("paper-clip-maximizer", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.PaperClipMaximizer as Program<PaperClipMaximizer>;

  const payer = web3.Keypair.generate();

  // const log_id = connection.onLogs( 'all',(x) => {
  //   if (x.err != null) {
  //     logger.log("error : " + x.err.toString())
  //   }
  //   else {
  //       for (const l of x.logs) {
  //         logger.log("log : " + l)
  //       }
  //   }
  // })

  it("Airdrop payer 10 SOLs", async () => {
    
    // airdrop payer 10 SOLs
    const airdrop_sig = await connection.requestAirdrop(payer.publicKey, 10 * web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdrop_sig, "finalized");
  });

  let group : web3.PublicKey = null;

  it("Initialize group", async () => {
    const [_group, _group_bump] = web3.PublicKey.findProgramAddressSync([Buffer.from("pcm_group"), payer.publicKey.toBuffer()], program.programId);
    group = _group;
    const [source, _s_bump] = web3.PublicKey.findProgramAddressSync([Buffer.from("source"), group.toBuffer()], program.programId);
    const [burn, _b_bump] = web3.PublicKey.findProgramAddressSync([Buffer.from("burn"), group.toBuffer()], program.programId);
    const [applicationFeesPda, _app_fee_bump] = web3.PublicKey.findProgramAddressSync([group.toBuffer()], applicationFeesProgram);
    
    let signature = await program.methods.initialize().accounts(
      {
        admin: payer.publicKey,
        group,
        source,
        burn,
        applicationFeesPda,
        applicationFeesProgram,
        systemProgram: web3.SystemProgram.programId,
      }
    ).signers([payer]).rpc();

    await connection.confirmTransaction(signature, "finalized");

    let application_fee_account_info = await connection.getAccountInfo(applicationFeesPda);
    let fee = application_fee_account_info.data.readBigUInt64LE(0);
    assert(fee.toString() === web3.LAMPORTS_PER_SOL.toString(), "Fee is not 1 SOL");
  });

  it("Application fee is charged", async () => {
    const balance_before = await connection.getBalance(payer.publicKey);
    const group_balance_before = await connection.getBalance(group);
    const group_info: PaperClipMaximizerGroup =
      await program.account.paperclipGroup.fetch(group);
    const signature = await program.methods.makePaperClips(new anchor.BN(1000)).accounts(
      {
        group : group, 
        applicationFeesProgram: applicationFeesProgram,
        source: group_info.sourceAccount,
        burn: group_info.burnAccount,
        systemProgram: web3.SystemProgram.programId,
        payer : payer.publicKey,
      }).signers([payer]).rpc();
    
    await connection.confirmTransaction(signature, "finalized");
    const balance_after = await connection.getBalance(payer.publicKey);
    const group_balance_after = await connection.getBalance(group);
    logger.log("before : " + balance_before);
    logger.log("after : " + balance_after);
    logger.log("group before : " + group_balance_before);
    logger.log("group after : " + group_balance_after);
    assert(balance_before - balance_after > web3.LAMPORTS_PER_SOL);
    assert(group_balance_after - group_balance_before > web3.LAMPORTS_PER_SOL);
  })

  it("Application fee is rebated sucessfully", async () => {
    const balance_before = await connection.getBalance(payer.publicKey);
    const group_balance_before = await connection.getBalance(group);
    const group_info: PaperClipMaximizerGroup =
      await program.account.paperclipGroup.fetch(group);
    const transfer_ix = web3.SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: group_info.sourceAccount,
      lamports: 10000,
    });
    let transaction = new web3.Transaction().add(transfer_ix);
    transaction.recentBlockhash = await (await connection.getLatestBlockhash()).blockhash;
    await web3.sendAndConfirmTransaction(connection, transaction, [payer]);
    const burn_balance_before = await connection.getBalance(group_info.burnAccount);

    const signature = await program.methods.makePaperClips(new anchor.BN(1000)).accounts(
      {
        group : group, 
        applicationFeesProgram: applicationFeesProgram,
        source: group_info.sourceAccount,
        burn: group_info.burnAccount,
        systemProgram: web3.SystemProgram.programId,
        payer : payer.publicKey,
      }).signers([payer]).rpc();
    
    await connection.confirmTransaction(signature, "finalized");
    const balance_after = await connection.getBalance(payer.publicKey);
    const group_balance_after = await connection.getBalance(group);
    const burn_balance_after = await connection.getBalance(group_info.burnAccount);

    logger.log("before : " + balance_before);
    logger.log("after : " + balance_after);
    logger.log("group before : " + group_balance_before);
    logger.log("group after : " + group_balance_after);
    logger.log("burn before : " + burn_balance_before);
    logger.log("burn after : " + burn_balance_after);

    assert(balance_before - balance_after < web3.LAMPORTS_PER_SOL);
    assert(group_balance_after == group_balance_before);
    assert(burn_balance_after == burn_balance_before + 1000);
  })

  // it("remove log listner", async () => {
  //   await connection.removeOnLogsListener(log_id);
  // });
});
