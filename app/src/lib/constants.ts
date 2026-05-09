export const MAX_NAME_LEN = 100;
export const MAX_KMS_REF_LEN = 64;

export const LOCAL_RPC_URL = "http://127.0.0.1:8899";
export const LOCAL_SOLANA_CLUSTER = "localnet";

export const EXPLORER = (
  sig: string,
  cluster = process.env.NEXT_PUBLIC_SOLANA_CLUSTER ?? LOCAL_SOLANA_CLUSTER
) => {
  if (cluster === LOCAL_SOLANA_CLUSTER) {
    return `https://explorer.solana.com/tx/${sig}?cluster=custom&customUrl=${encodeURIComponent(
      process.env.NEXT_PUBLIC_RPC_URL ?? LOCAL_RPC_URL
    )}`;
  }

  return `https://explorer.solana.com/tx/${sig}?cluster=${cluster}`;
};

export const SCOPE_READ  = 0b0001;
export const SCOPE_WRITE = 0b0010;
export const SCOPE_ADMIN = 0b0100;

export const SCOPE_OPTIONS = [
  { label: "Read",  bit: SCOPE_READ },
  { label: "Write", bit: SCOPE_WRITE },
  { label: "Admin", bit: SCOPE_ADMIN },
];

export const MAX_DID_LEN = 128;
