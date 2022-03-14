import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PythTest } from "../target/types/pyth_test";

describe("pyth-test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.PythTest as Program<PythTest>;


  // This is an a account on devnet that contains data for SOL price.
  // THis is the link that contains all the devnet accounts https://pyth.network/developers/accounts/?cluster=devnet#
  let SolPriceAccount = new anchor.web3.PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix")
  let SolSymbolAccount = new anchor.web3.PublicKey("3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E");

  it("Get Price from PYTH!", async () => {
    // Add your test here.
    const tx = await program.rpc.getSolPrice({
      accounts:{
        pythAccount:SolPriceAccount,
        rent:anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram:anchor.web3.SystemProgram.programId,
        userAccount: program.provider.wallet.publicKey
      }
    });
    // Look for this Tx in solscan or explorer.solana.com and check the logs you will get all the data there.
    console.log("Your transaction signature", tx);
  });

  // it("Should be able to get SOL Account Data",async () => {


  //   const tx = await program.rpc.getSolSymbol({
  //     accounts:{
  //       pythAccount:SolPriceAccount,
  //       rent:anchor.web3.SYSVAR_RENT_PUBKEY,
  //       systemProgram:anchor.web3.SystemProgram.programId,
  //       userAccount: program.provider.wallet.publicKey
  //     }
  //   });

  //   console.log("Your transaction signature", tx);
  // })
});
