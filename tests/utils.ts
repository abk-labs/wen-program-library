import { AnchorProvider } from "@coral-xyz/anchor";
import {
  Keypair,
  Connection,
  PublicKey,
  VersionedTransaction,
  TransactionMessage,
  TransactionInstruction,
  Signer,
  Commitment,
  SystemProgram,
} from "@solana/web3.js";
import {
  TYPE_SIZE,
  LENGTH_SIZE,
  getMintLen,
  ExtensionType,
  createInitializeMetadataPointerInstruction,
  TOKEN_2022_PROGRAM_ID,
  createInitializeMint2Instruction,
  createMintToCheckedInstruction,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import {
  TokenMetadata,
  pack,
  createInitializeInstruction,
} from "@solana/spl-token-metadata";
import { min } from "bn.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

export const MANAGER_SEED = Buffer.from("manager");
export const GROUP_ACCOUNT_SEED = Buffer.from("group");
export const MEMBER_ACCOUNT_SEED = Buffer.from("member");
export const TEST_SALE = Buffer.from("test_sale");
export const SALE = Buffer.from("sale");
export const LISTING = Buffer.from("listing");

export const getExtraMetasAccountPda = (mint: PublicKey, programId: PublicKey) => {
  const [extraMetasAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("extra-account-metas"), mint.toBuffer()],
    programId
  );
  return extraMetasAccount;
};

export const getApproveAccountPda = (mint: PublicKey, programId: PublicKey) => {
  const [approveAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("approve-account"), mint.toBuffer()],
    programId
  );

  return approveAccount;
};

export const getManagerAccountPda = (programId: PublicKey) => {
  const [managerAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("manager")],
    programId
  );
  return managerAccount;
};

export const getGroupAccountPda = (mint: PublicKey, programId: PublicKey) => {
  const [groupAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("group"), mint.toBuffer()],
    programId
  );
  return groupAccount;
};

export const getMemberAccountPda = (mint: PublicKey, programId: PublicKey) => {
  const [memberAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("member"), mint.toBuffer()],
    programId
  );
  return memberAccount;
};

export const getDistributionAccountPda = (
  group: PublicKey,
  paymentMint: PublicKey,
  programId: PublicKey
) => {
  const [distributionAccount] = PublicKey.findProgramAddressSync(
    [group.toBuffer(), paymentMint.toBuffer()],
    programId
  );
  return distributionAccount;
};

export const getSaleAccountPda = (
  group: PublicKey,
  distribution: PublicKey,
  programId: PublicKey
) => {
  const [saleAccount] = PublicKey.findProgramAddressSync(
    [TEST_SALE, SALE, group.toBuffer(), distribution.toBuffer()],
    programId
  );
  return saleAccount;
};

export const getListingAccountPda = (
  seller: PublicKey,
  mint: PublicKey,
  programId: PublicKey
) => {
  const [listingAccount] = PublicKey.findProgramAddressSync(
    [TEST_SALE, LISTING, seller.toBuffer(), mint.toBuffer()],
    programId
  );
  return listingAccount;
};

export async function airdrop(
  connection: Connection,
  address: PublicKey,
  airdropLamports: number,
  commitment: Commitment = "confirmed"
) {
  const signature = await connection.requestAirdrop(address, airdropLamports);
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash(
    commitment
  );

  await connection.confirmTransaction(
    {
      blockhash,
      lastValidBlockHeight,
      signature,
    },
    commitment
  );
}

export async function getMinRentForWNSMint(
  connection: Connection,
  metaData: TokenMetadata,
  type: string
) {
  // Size of MetadataExtension 2 bytes for type, 2 bytes for length
  const metadataExtension = TYPE_SIZE + LENGTH_SIZE;
  // Size of metadata
  const metadataLen = pack(metaData).length;

  // Size of Mint Account with extensions
  const mintLen = getMintLen(
    [
      ExtensionType.MintCloseAuthority,
      ExtensionType.MetadataPointer,
      ExtensionType.TransferHook,
      ExtensionType.PermanentDelegate,
    ].concat(
      type === "member"
        ? [ExtensionType.GroupMemberPointer]
        : [ExtensionType.GroupPointer]
    )
  );

  // Minimum lamports required for Mint Account
  return connection.getMinimumBalanceForRentExemption(
    mintLen + metadataExtension + metadataLen
  );
}

export async function createMint2022Ix(
  connection: Connection,
  mint: PublicKey,
  authority: PublicKey,
  payer: PublicKey
) {
  // Size of MetadataExtension 2 bytes for type, 2 bytes for length
  const metadataExtension = TYPE_SIZE + LENGTH_SIZE;

  const metadata: TokenMetadata = {
    mint,
    name: "USD Coin",
    symbol: "USDC",
    uri: "https://statics.solscan.io/cdn/imgs/s60?ref=68747470733a2f2f7261772e67697468756275736572636f6e74656e742e636f6d2f736f6c616e612d6c6162732f746f6b656e2d6c6973742f6d61696e2f6173736574732f6d61696e6e65742f45506a465764643541756671535371654d32714e31787a7962617043384734774547476b5a777954447431762f6c6f676f2e706e67",
    additionalMetadata: [],
    updateAuthority: authority,
  };
  // Size of metadata
  const metadataLen = pack(metadata).length;

  const mintLen = getMintLen([ExtensionType.MetadataPointer]);

  // Minimum lamports required for Mint Account
  const mintRent = await connection.getMinimumBalanceForRentExemption(
    mintLen + metadataExtension + metadataLen
  );

  return {
    ixs: [
      SystemProgram.createAccount({
        fromPubkey: payer,
        newAccountPubkey: mint,
        programId: TOKEN_2022_PROGRAM_ID,
        space: mintLen,
        lamports: mintRent,
      }),
      createInitializeMetadataPointerInstruction(
        mint,
        authority,
        mint,
        TOKEN_2022_PROGRAM_ID
      ),
      createInitializeMint2Instruction(
        mint,
        6,
        authority,
        authority,
        TOKEN_2022_PROGRAM_ID
      ),
      createInitializeInstruction({
        metadata: mint,
        mint,
        mintAuthority: authority,
        programId: TOKEN_2022_PROGRAM_ID,
        updateAuthority: authority,
        ...metadata,
      }),
    ],
  };
}

export function mintToBuyerSellerIx(
  mint: PublicKey,
  authority: PublicKey,
  payer: PublicKey,
  buyer: PublicKey,
  buyerTokenAccount: PublicKey,
  seller: PublicKey,
  sellerTokenAccount: PublicKey
) {
  return {
    ixs: [
      createAssociatedTokenAccountInstruction(
        payer,
        buyerTokenAccount,
        buyer,
        mint,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_PROGRAM_ID
      ),
      createMintToCheckedInstruction(
        mint,
        buyerTokenAccount,
        authority,
        10_000 * 10 ** 6,
        6,
        [],
        TOKEN_2022_PROGRAM_ID
      ),
      createAssociatedTokenAccountInstruction(
        payer,
        sellerTokenAccount,
        seller,
        mint,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_PROGRAM_ID
      ),
      createMintToCheckedInstruction(
        mint,
        sellerTokenAccount,
        authority,
        10_000 * 10 ** 6,
        6,
        [],
        TOKEN_2022_PROGRAM_ID
      ),
    ],
  };
}

export async function sendAndConfirmWNSTransaction(
  connection: Connection,
  instructions: TransactionInstruction[],
  provider: AnchorProvider,
  skipPreflight = true,
  additionalSigners: Signer[] = []
) {
  const transaction = new VersionedTransaction(
    new TransactionMessage({
      instructions,
      payerKey: provider.wallet.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash("confirmed")).blockhash,
    }).compileToV0Message()
  );
  const signedTx = await provider.wallet.signTransaction(transaction);
  signedTx.sign(additionalSigners);

  try {
    const signature = await connection.sendTransaction(signedTx, {
      preflightCommitment: "confirmed",
      skipPreflight,
    });
    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash(
      "confirmed"
    );
    await connection.confirmTransaction(
      {
        signature,
        lastValidBlockHeight,
        blockhash,
      },
      "confirmed"
    );
    return signature;
  } catch (err) {
    throw err;
  }
}
