import { PublicKey } from "@solana/web3.js";

export const findConfigPda = (pid: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from("config")], pid)[0];

export const findNotaryPda = (pid: PublicKey, auth: PublicKey) =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("notary"), auth.toBuffer()],
    pid
  )[0];

export const findGrantPda = (
  pid: PublicKey,
  ownerPda: PublicKey,
  grantee: PublicKey,
  scope: number
) =>
  PublicKey.findProgramAddressSync(
    [
      Buffer.from("grant"),
      ownerPda.toBuffer(),
      grantee.toBuffer(),
      Buffer.from([scope & 0xff]),
    ],
    pid
  )[0];

export const findOwnerPda = (pid: PublicKey, ownerPk: PublicKey) =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("owner"), ownerPk.toBuffer()],
    pid
  )[0];

export const findOwnerDocumentSeqPda = (pid: PublicKey, ownerPda: PublicKey) =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("owner_doc_seq"), ownerPda.toBuffer()],
    pid
  )[0];

export const findDocumentPda = (
  programId: PublicKey,
  ownerPda: PublicKey,
  seq: bigint | number
) => {
  const seqBuffer = Buffer.alloc(8);
  seqBuffer.writeBigUInt64LE(BigInt(seq));
  return PublicKey.findProgramAddressSync(
    [Buffer.from("document"), ownerPda.toBuffer(), seqBuffer],
    programId
  )[0];
};

export function findDelegatePda(
  programId: PublicKey,
  ownerWallet: PublicKey,
  delegateWallet: PublicKey
) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("delegate"),
      ownerWallet.toBuffer(),
      delegateWallet.toBuffer(),
    ],
    programId
  )[0];
}

export const findHospitalPda = findNotaryPda;
export const findPatientPda = findOwnerPda;
export const findPatientSeqPda = findOwnerDocumentSeqPda;
export const findTrusteePda = findDelegatePda;
