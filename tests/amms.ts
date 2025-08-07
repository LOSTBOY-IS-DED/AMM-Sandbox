import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amms } from "../target/types/amms";

import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

describe("amm creation", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.amms as Program<Amms>;
  const connection = provider.connection;
  const admin = provider.wallet;

  let mintX: anchor.web3.PublicKey;
  let mintY: anchor.web3.PublicKey;
  let lpMint: anchor.web3.PublicKey;

  let configPda: anchor.web3.PublicKey;
  let vaultX: anchor.web3.PublicKey;
  let vaultY: anchor.web3.PublicKey;

  let userAtaX: anchor.web3.PublicKey;
  let userAtaY: anchor.web3.PublicKey;
  let userLpAta: anchor.web3.PublicKey;

  const seed = new anchor.BN(42);
  const fee = 30;


  it("Initializing the AMM pool", async () => {

    mintX = await createMint(connection, admin.payer, admin.publicKey, null, 6);
    mintY = await createMint(connection, admin.payer, admin.publicKey, null, 6);

    [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    [lpMint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), configPda.toBuffer()],
      program.programId
    );

    vaultX = await getAssociatedTokenAddress(mintX, configPda, true);
    vaultY = await getAssociatedTokenAddress(mintY, configPda, true);

    const tx = await program.methods
      .initialize(seed, fee, null)
      .accounts({
        admin: admin.publicKey,
        mintX,
        mintY,
        config: configPda,
        mintLp: lpMint,
        vaultX,
        vaultY,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);

    console.log("✅ AMM initialized successfully");;

    // Create user token accounts and mint initial balances
    const ataX = await getOrCreateAssociatedTokenAccount(connection, admin.payer, mintX, admin.publicKey);
    const ataY = await getOrCreateAssociatedTokenAccount(connection, admin.payer, mintY, admin.publicKey);
    const ataLp = await getOrCreateAssociatedTokenAccount(connection, admin.payer, lpMint, admin.publicKey);

    await mintTo(connection, admin.payer, mintX, ataX.address, admin.payer, 1_000_000);
    await mintTo(connection, admin.payer, mintY, ataY.address, admin.payer, 1_000_000);

    userAtaX = ataX.address;
    userAtaY = ataY.address;
    userLpAta = ataLp.address;
  });

  it("Deposits liquidity into the pool", async () => {
    const depositAmount = new anchor.BN(1_000_000);
    const maxX = new anchor.BN(500_000);
    const maxY = new anchor.BN(500_000);

    const tx = await program.methods
      .deposit(depositAmount, maxX, maxY)
      .accounts({
        user: admin.publicKey,
        userTokenX: userAtaX,
        userTokenY: userAtaY,
        userLp: userLpAta,
        config: configPda,
        vaultX,
        vaultY,
        lpMint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    console.log("✅ Deposited liquidity");
  });

  
});
