import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SellNft } from "../target/types/sell_nft";
import { AuthorityType, createMint, createSetAuthorityInstruction, getOrCreateAssociatedTokenAccount, mintTo, TokenError,  } from '@solana/spl-token';
import TransactionFactory from "@project-serum/anchor/dist/cjs/program/namespace/transaction";


describe("sell-nft", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SellNft as Program<SellNft>;
  const authority= anchor.web3.Keypair.fromSecretKey(new Uint8Array([137,54,63,244,72,108,160,19,160,133,133,238,46,97,231,220,175,93,74,208,143,34,178,107,231,254,68,74,93,223,230,88,213,93,170,52,250,19,204,143,208,40,212,81,157,108,252,114,74,95,12,219,242,95,63,231,108,148,2,94,139,52,86,114]));
  const [nftPda, bump] = anchor.web3.PublicKey.findProgramAddressSync([authority.publicKey.toBuffer(), Buffer.from('nft_holder')], program.programId)
  console.log(authority.publicKey.toBase58());

  
  it("Transferring the token!", async () => {
    // Add your test here.
  const mint = await createMint(program.provider.connection, authority, authority.publicKey,null, 0);
  const tokenAccount = await getOrCreateAssociatedTokenAccount(program.provider.connection, authority, mint, authority.publicKey);
  const pdaTokenAccount = await getOrCreateAssociatedTokenAccount(program.provider.connection, authority, mint, nftPda, true);

  //make sure authority and pda have sols in them before running the tests

  await mintTo(program.provider.connection,authority, mint, tokenAccount.address, authority, 1);
  
  let transaction = new anchor.web3.Transaction().add(createSetAuthorityInstruction(mint, authority.publicKey, AuthorityType.MintTokens, null ));
  await anchor.web3.sendAndConfirmTransaction(program.provider.connection, transaction, [authority]);

    const tx = await program.methods.initialize(bump).accounts({
      nftMint: mint,
      nftTokenAccount: pdaTokenAccount.address,
      nftPda: nftPda,
      authority: authority.publicKey,
      tokenAccount: tokenAccount.address
    }).signers([authority]).rpc();
    console.log("Your transaction signature", tx);

    const tx2 = await program.methods.getBack().accounts({
      nftMint: mint,
      nftTokenAccount: pdaTokenAccount.address,
      nftPda: nftPda,
      authority: authority.publicKey,
      tokenAccount: tokenAccount.address
    }).signers([authority]).rpc()
    console.log("Your transaction signature", tx2);
  });
});
