import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import assert from "assert";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.calculator as Program<Calculator>;
  const newAccount = anchor.web3.Keypair.generate();
  console.log("key of the newAccount :", newAccount.publicKey);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(10)
      .accounts({
        signer: anchor.getProvider().wallet?.publicKey,
        account: newAccount.publicKey
      })
      .signers([newAccount])
      .rpc();
    console.log("Your transaction signature", tx);

    let account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.ok(account.num == 10);
  });

  it("Is double!", async () => {
    const tx = await program.methods.double()
      .accounts({
        account: newAccount.publicKey,
        signer: anchor.getProvider().wallet?.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    let account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.ok(account.num == 20);
  });

  it("is add! now ", async () => {
    const tx = await program.methods.add(10)
      .accounts({
        account: newAccount.publicKey,
        signer: anchor.getProvider().wallet?.publicKey,
      })
      .rpc();
    let account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.ok(account.num == 30);
  });
})
/// this is the test cases  for the anchor contract