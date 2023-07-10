import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Chatbids } from "../target/types/chatbids";
import { buyerKeypair, sellerKeypair } from "./utils/secretKeys";
import {
  Connection,
  Keypair,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

describe("chatbids", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Chatbids as Program<Chatbids>;
  const SOLANA_HOST = clusterApiUrl("devnet");
  const connection = new anchor.web3.Connection(SOLANA_HOST);

  const MINT_ADDRESS = "Axox2MUKqJQxmnrxDb77h1CmQaAUpmLbZRs75jAJTxX8";
  const OWNER = "FKQ1nEazoN9SiEy5xRC4FrskjT3B3usdB74sC9sUYq7";

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  it("Is creating offers!", async () => {
    const buyer = await buyerKeypair();
    const seller = await sellerKeypair();

    console.log("Buyer Public keys", buyer.publicKey.toBase58());
    console.log(
      "Buyer Sol balance:",
      (await connection.getBalance(buyer.publicKey)) / LAMPORTS_PER_SOL
    );

    console.log("Seller Public keys", seller.publicKey.toBase58());
    console.log(
      "Seller Sol balance:",
      (await connection.getBalance(seller.publicKey)) / LAMPORTS_PER_SOL
    );

    // Add your test here.
    // const tx = await program.methods.createDirectOffer().rpc();
    // console.log("Your transaction signature", tx);
  });
});
