"use client";
import { useMemo } from "react";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import idl from "../../anchor.json";
import { LOCAL_SOLANA_CLUSTER } from "@/lib/constants";

export function useProgram() {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();

  const programId = useMemo(
    () => new PublicKey(process.env.NEXT_PUBLIC_PROGRAM_ID ?? idl.address),
    []
  );

  const provider = useMemo(() => {
    if (!wallet) return null;
    return new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  }, [connection, wallet]);

  const program = useMemo(() => {
    if (!provider) return null;
    return new anchor.Program(idl as anchor.Idl, provider);
  }, [provider]);

  const cluster = process.env.NEXT_PUBLIC_SOLANA_CLUSTER ?? LOCAL_SOLANA_CLUSTER;
  return { program, programId, cluster, ready: !!program };
}
