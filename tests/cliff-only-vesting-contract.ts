import * as anchor from "@coral-xyz/anchor";
import { Program, Idl } from "@coral-xyz/anchor";
import idl from "../target/idl/cliff_only_vesting_contract.json";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { assert } from "chai";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

describe("cliff_only_vesting_contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const programId = new PublicKey("J3Qo8zJpx3cj6PW8Zru1xTbR2WFPLmy8rJMyoctKNiVf");
  const program = new Program(idl as Idl, provider);
  const admin = Keypair.generate();
  const beneficiary = Keypair.generate();

  let config_vesting: PublicKey;
  let vesting_vault: PublicKey;
  let authority: PublicKey;
  let admin_token_account: PublicKey;
  let token_mint: PublicKey;
  let beneficiary_data: PublicKey;
  let beneficiary_wallet: PublicKey;
  
  const decimals = 2;
  const amount = 5;
  const total_tokens = 500;
  const startTime = Math.floor(Date.now() / 1000);
  const cliffDuration = 1; // 1 Seconds obviously for tests to pass!

  before(async () => {
    const airdropSignature = await provider.connection.requestAirdrop(admin.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockHash = await provider.connection.getLatestBlockhash()

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    })

    token_mint = await createMint(provider.connection, admin, admin.publicKey, null, 9);

    admin_token_account = await createAccount(provider.connection, admin, token_mint, admin.publicKey);
    beneficiary_wallet = await createAccount(provider.connection, admin, token_mint, beneficiary.publicKey);

    const [configPda] = findProgramAddressSync([Buffer.from("config_vesting"), token_mint.toBuffer()],programId)
    config_vesting = configPda;
    
    const [vestingVaultPda] = findProgramAddressSync([Buffer.from("vesting_vault"), token_mint.toBuffer()],programId)
    vesting_vault = vestingVaultPda;

    const [authorityPda] = findProgramAddressSync([Buffer.from("authority"), token_mint.toBuffer()],programId)
    authority = authorityPda;

    const [beneficiaryDataPda] = findProgramAddressSync(
      [
      Buffer.from("beneficiary_data"), 
      beneficiary_wallet.toBuffer()
      ],
      programId
    )

    beneficiary_data = beneficiaryDataPda;

    await mintTo(provider.connection, admin, token_mint, admin_token_account, admin, 15000000);
    await mintTo(provider.connection, admin, token_mint, beneficiary_wallet, admin, 15000000);

    await program.methods
      .initializeAccounts()
      .accounts({
          configVesting: config_vesting,
          vestingVault: vesting_vault,
          authority: authority,
          admin: admin.publicKey,
          adminTokenAccount: admin_token_account,
          tokenMint: token_mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();
  });

  it("Initialize a beneficiary account", async () => {
    
    await program.methods
      .initializeBeneficiaryAccount()
      .accounts({
        beneficiaryData: beneficiary_data,
        beneficiaryWallet: beneficiary_wallet,
        admin: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();
  });

  it("Add a beneficiary", async () => {
    
    await program.methods
      .addBeneficiary(
      new anchor.BN(total_tokens),
      )
      .accounts({
          beneficiaryData: beneficiary_data,
          beneficiaryWallet: beneficiary_wallet,
          vestingVault: vesting_vault,
          configVesting: config_vesting,
          beneficiary: beneficiary.publicKey,
          tokenMint: token_mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([beneficiary])
        .rpc();
  });

  it("Initialize vesting", async () => {

    const adminTokenAccountBefore = await getAccount(provider.connection, admin_token_account);
    const revocable = true;

    await program.methods
      .initializeVesting(
      new anchor.BN(decimals),
      new anchor.BN(startTime),
      new anchor.BN(cliffDuration),
      revocable
      )
      .accounts({
          configVesting: config_vesting,
          vestingVault: vesting_vault,
          authority: authority,
          admin: admin.publicKey,
          adminTokenAccount: admin_token_account,
          tokenMint: token_mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

    const adminTokenAccountAfter = await getAccount(provider.connection, admin_token_account);
    const vestingVaultAfter = await getAccount(provider.connection, vesting_vault);
    
//    assert.equal(
//      BigInt(Number(adminTokenAccountBefore.amount)) - (BigInt(amount) * BigInt(10 ** decimals)),
//      BigInt(Number(adminTokenAccountAfter.amount)),
//      "Admin token account should decrease"
//    );
//
//    assert.equal(
//      Number(vestingVaultAfter.amount),
//      amount * 10 ** decimals,
//      "Vesting vault should increase"
//    );

  });

  it("Claim vesting", async () => {

    const beneficiaryWalletBefore = await getAccount(provider.connection, beneficiary_wallet);

    await program.methods
      .claim()
      .accounts({
          configVesting: config_vesting,
          beneficiaryData: beneficiary_data,
          beneficiaryWallet: beneficiary_wallet,
          beneficiary: beneficiary.publicKey,
          vestingVault: vesting_vault,
          authority: authority,
          tokenMint: token_mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([beneficiary])
        .rpc();

    const beneficiaryWalletAfter = await getAccount(provider.connection, beneficiary_wallet);

    assert.equal(
        Number(beneficiaryWalletBefore.amount) + total_tokens,
        Number(beneficiaryWalletAfter.amount),
        "Beneficiary wallet should increase by percent of total_tokens"
    )

    });

it("Revoke vesting", async () => {
      // Fetch and log config_vesting state
    const configVesting = await program.account.cliffVestingAccount.fetch(config_vesting);
    console.log("Config Vesting State:", {
        revocable: configVesting.revocable,
        decimals: configVesting.decimals.toString(),
        startTime: configVesting.startTime.toString(),
        cliffDuration: configVesting.cliffDuration.toString(),
    });

    await program.methods
      .revoke()
      .accounts({
          configVesting: config_vesting,
          vestingVault: vesting_vault,
          authority: authority,
          admin: admin.publicKey,
          adminTokenAccount: admin_token_account,
          tokenMint: token_mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

    });

});