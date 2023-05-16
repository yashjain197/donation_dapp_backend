import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DonationDapp } from "../target/types/donation_dapp";

describe("donation-dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DonationDapp as Program<DonationDapp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
