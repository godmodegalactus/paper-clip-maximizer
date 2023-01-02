import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";
import { PaperClipMaximizer } from "../target/types/paper_clip_maximizer";
import * as logger from "mocha-logger"
import struct from "buffer-layout"
import { assert } from "chai";

const applicationFeesProgram = new web3.PublicKey('App1icationFees1111111111111111111111111111')

describe("paper-clip-maximizer", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.PaperClipMaximizer as Program<PaperClipMaximizer>;

  const payer = web3.Keypair.generate();

  const log_id = connection.onLogs( 'all',(x) => {
    if (x.err != null) {
      logger.log("error : " + x.err.toString())
    }
    else {
        for (const l of x.logs) {
          logger.log("log : " + l)
        }
    }
  })
  it("Airdrop payer 10 SOLs", async () => {
    
    // airdrop payer 10 SOLs
    const airdrop_sig = await connection.requestAirdrop(payer.publicKey, 10 * web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdrop_sig, "finalized");
  });

  it("Initialize group", async () => {
    const [group, _group_bump] = web3.PublicKey.findProgramAddressSync([Buffer.from("pcm_group"), payer.publicKey.toBuffer()], program.programId);
    const [applicationFeesPda, _app_fee_bump] = web3.PublicKey.findProgramAddressSync([group.toBuffer()], applicationFeesProgram);
    
    let signature = await program.methods.initialize().accounts(
      {
        admin: payer.publicKey,
        group,
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

  it("remove log listner", async () => {
    await connection.removeOnLogsListener(log_id);
  });
});
