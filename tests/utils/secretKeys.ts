import bs58 from "bs58";
import { Connection, Keypair, clusterApiUrl } from "@solana/web3.js";
import "dotenv/config";

export const sellerKeypair = async () => {
  const sk = process.env.SELLER_PRIVATE_KEY!;
  const keypair = Keypair.fromSecretKey(bs58.decode(sk));
  return keypair;
};

export const buyerKeypair = async () => {
  const sk = process.env.BUYER_PRIVATE_KEY!;
  const keypair = Keypair.fromSecretKey(bs58.decode(sk));
  return keypair;
};
