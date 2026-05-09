import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { BN } from 'bn.js';
import { expect } from 'chai';
import { Anchor } from '../target/types/anchor';

const SEED_CONFIG = 'config';
const SEED_OWNER = 'owner';
const SEED_OWNER_DOC_SEQ = 'owner_doc_seq';
const SEED_NOTARY = 'notary';
const SEED_GRANT = 'grant';
const SEED_DOCUMENT = 'document';

const SCOPE_READ = 1 << 0;
const SCOPE_WRITE = 1 << 1;
const SCOPE_ADMIN = 1 << 2;
const ALLOWED = SCOPE_READ | SCOPE_WRITE | SCOPE_ADMIN;

const COMMITMENT: anchor.web3.Commitment = 'confirmed';

const u64LeBytes = (n: BN | number): Buffer => {
  const bn = BN.isBN(n as any) ? (n as BN) : new BN(n);
  const buf = Buffer.alloc(8);
  buf.writeBigUInt64LE(BigInt(bn.toString()));
  return buf;
};

describe('titlevault end-to-end (happy + unhappy paths)', () => {
  const base = anchor.AnchorProvider.local();
  const provider = new anchor.AnchorProvider(base.connection, base.wallet, {
    commitment: COMMITMENT,
    preflightCommitment: COMMITMENT,
  });
  anchor.setProvider(provider);

  const program = (anchor.workspace.Anchor as Program<Anchor>) as Program<any>;
  const connection = provider.connection;

  const admin = anchor.web3.Keypair.generate();
  const ownerAuthority = anchor.web3.Keypair.generate();
  const notaryAuthority = anchor.web3.Keypair.generate();
  const rando = anchor.web3.Keypair.generate();

  let configPda: anchor.web3.PublicKey;
  let ownerPda: anchor.web3.PublicKey;
  let ownerDocSeqPda: anchor.web3.PublicKey;
  let notaryPda: anchor.web3.PublicKey;
  let grantWritePda: anchor.web3.PublicKey;
  let documentPda: anchor.web3.PublicKey;

  const airdrop = async (pk: anchor.web3.PublicKey, lamports = 2e9) => {
    const sig = await connection.requestAirdrop(pk, lamports);
    await connection.confirmTransaction(sig, COMMITMENT);
  };

  const deriveConfigPda = () => {
    [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_CONFIG)],
      program.programId
    );
  };

  const deriveOwnerPdas = () => {
    [ownerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_OWNER), ownerAuthority.publicKey.toBuffer()],
      program.programId
    );

    [ownerDocSeqPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_OWNER_DOC_SEQ), ownerPda.toBuffer()],
      program.programId
    );
  };

  const deriveNotaryPda = () => {
    [notaryPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_NOTARY), notaryAuthority.publicKey.toBuffer()],
      program.programId
    );
  };

  const deriveGrantPda = (
    grantee: anchor.web3.PublicKey,
    scopeByte: number
  ) => {
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_GRANT),
        ownerPda.toBuffer(),
        grantee.toBuffer(),
        Buffer.from([scopeByte & 0xff]),
      ],
      program.programId
    );
    return pda;
  };

  const deriveDocumentPda = (seq: BN | number) => {
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_DOCUMENT), ownerPda.toBuffer(), u64LeBytes(seq)],
      program.programId
    );
    return pda;
  };

  before('airdrop & derive PDAs', async () => {
    await Promise.all([
      airdrop(provider.wallet.publicKey),
      airdrop(admin.publicKey),
      airdrop(ownerAuthority.publicKey),
      airdrop(notaryAuthority.publicKey),
      airdrop(rando.publicKey),
    ]);

    deriveConfigPda();
    deriveOwnerPdas();
    deriveNotaryPda();
  });

  it('HAPPY: init_config sets program authority and vault namespace', async () => {
    const kmsNs = 'titlevault-dev';

    let ev: any | null = null;
    const sub = await program.addEventListener(
      'ConfigInitialized',
      (e: any) => (ev = e)
    );

    await program.methods
      .initConfig(kmsNs)
      .accounts({
        config: configPda,
        authority: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await program.removeEventListener(sub);

    const cfg = await program.account.config.fetch(configPda);
    expect(cfg.authority.toBase58()).to.eq(admin.publicKey.toBase58());
    expect(cfg.kmsNamespace).to.eq(kmsNs);

    if (ev) {
      expect(ev.authority.toBase58()).to.eq(admin.publicKey.toBase58());
      expect(ev.kmsNamespace).to.eq(kmsNs);
    }
  });

  it('HAPPY: upsert_owner creates owner profile and document sequence', async () => {
    const ownerRef = 'owner:did:example:kresna';

    let ev: any | null = null;
    const sub = await program.addEventListener(
      'OwnerUpserted',
      (e: any) => (ev = e)
    );

    await program.methods
      .upsertOwner(ownerRef)
      .accounts({
        authority: ownerAuthority.publicKey,
        owner: ownerPda,
        ownerDocSeq: ownerDocSeqPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([ownerAuthority])
      .rpc();

    await program.removeEventListener(sub);

    const owner = await program.account.propertyOwner.fetch(ownerPda);
    expect(owner.ownerPubkey.toBase58()).to.eq(
      ownerAuthority.publicKey.toBase58()
    );
    expect(owner.ownerRef).to.eq(ownerRef);

    const seq = await program.account.ownerDocumentSeq.fetch(ownerDocSeqPda);
    expect(seq.owner.toBase58()).to.eq(ownerPda.toBase58());
    expect(seq.value.toNumber()).to.eq(0);

    if (ev) {
      expect(ev.owner.toBase58()).to.eq(ownerPda.toBase58());
      expect(ev.ownerPubkey.toBase58()).to.eq(
        ownerAuthority.publicKey.toBase58()
      );
    }
  });

  it('HAPPY: register_notaries creates Notary PDA', async () => {
    const name = 'Jakarta Central Notary Office';
    const kmsRef = 'vault://hsm/titlevault-notary';

    let ev: any | null = null;
    const sub = await program.addEventListener(
      'NotaryRegistered',
      (e: any) => (ev = e)
    );

    await program.methods
      .registerNotaries(name, kmsRef)
      .accounts({
        registrar: admin.publicKey,
        config: configPda,
        notaryAuthority: notaryAuthority.publicKey,
        notary: notaryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await program.removeEventListener(sub);

    const notary = await program.account.notary.fetch(notaryPda);
    expect(notary.authority.toBase58()).to.eq(
      notaryAuthority.publicKey.toBase58()
    );
    expect(notary.name).to.eq(name);
    expect(notary.kmsRef).to.eq(kmsRef);

    if (ev) {
      expect(ev.notary.toBase58()).to.eq(notaryPda.toBase58());
      expect(ev.notaryAuthority.toBase58()).to.eq(
        notaryAuthority.publicKey.toBase58()
      );
    }
  });

  it('HAPPY: grant_access WRITE from owner to notary', async () => {
    grantWritePda = deriveGrantPda(notaryAuthority.publicKey, SCOPE_WRITE);

    let ev: any | null = null;
    const sub = await program.addEventListener(
      'GrantCreated',
      (e: any) => (ev = e)
    );

    await program.methods
      .grantAccess(SCOPE_WRITE)
      .accounts({
        authority: ownerAuthority.publicKey,
        config: configPda,
        owner: ownerPda,
        grant: grantWritePda,
        grantee: notaryAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([ownerAuthority])
      .rpc();

    await program.removeEventListener(sub);

    const grant = await program.account.grant.fetch(grantWritePda);
    expect(grant.owner.toBase58()).to.eq(ownerPda.toBase58());
    expect(grant.grantee.toBase58()).to.eq(
      notaryAuthority.publicKey.toBase58()
    );
    expect(grant.scope).to.eq(SCOPE_WRITE);
    expect(grant.revoked).to.eq(false);
    expect(grant.viaDelegate).to.eq(false);

    if (ev) {
      expect(ev.grant.toBase58()).to.eq(grantWritePda.toBase58());
      expect(ev.owner.toBase58()).to.eq(ownerPda.toBase58());
      expect(ev.grantee.toBase58()).to.eq(notaryAuthority.publicKey.toBase58());
    }
  });

  it('HAPPY: create_document uses WRITE grant and owner co-signature', async () => {
    const seqAcc = await program.account.ownerDocumentSeq.fetch(ownerDocSeqPda);
    const seq = seqAcc.value.toNumber();
    documentPda = deriveDocumentPda(seq);

    const cidEnc = 'bafy-titlevault-land-cert';
    const metaMime = 'application/json';
    const metaCid = 'bafy-titlevault-meta';
    const sizeBytes = new BN(123456);
    const blake2b = Buffer.alloc(32, 9);
    const edekRoot = Buffer.from('deadbeef', 'hex');
    const edekForOwner = Buffer.from('c0ffeec0ffee', 'hex');
    const edekForNotary = Buffer.from('abbaabba', 'hex');
    const edekRootAlgo = { kms: {} };
    const edekOwnerAlgo = { sealedBox: {} };
    const edekNotaryAlgo = { sealedBox: {} };
    const kmsRef = 'vault://hsm/titlevault-doc-key';
    const encVersion = 1;
    const encAlgo = { xChaCha20: {} };
    const notaryName = 'Jakarta Central Notary Office';
    const documentType = 'land_certificate';
    const propertyRef = 'SHM-3175-2026-0001';
    const counterpartyName = 'PT Terra Buyer';

    let ev: any | null = null;
    const sub = await program.addEventListener(
      'DocumentCreated',
      (e: any) => (ev = e)
    );

    await program.methods
      .createDocument(
        new BN(seq),
        cidEnc,
        metaMime,
        metaCid,
        sizeBytes,
        Array.from(blake2b),
        edekRoot,
        edekForOwner,
        edekForNotary,
        edekRootAlgo,
        edekOwnerAlgo,
        edekNotaryAlgo,
        kmsRef,
        encVersion,
        encAlgo,
        notaryName,
        documentType,
        propertyRef,
        counterpartyName
      )
      .accounts({
        uploader: notaryAuthority.publicKey,
        payer: ownerAuthority.publicKey,
        config: configPda,
        owner: ownerPda,
        ownerDocSeq: ownerDocSeqPda,
        notary: notaryPda,
        grantWrite: grantWritePda,
        document: documentPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([notaryAuthority, ownerAuthority])
      .rpc();

    await program.removeEventListener(sub);

    const doc = await program.account.propertyDocument.fetch(documentPda);
    expect(doc.owner.toBase58()).to.eq(ownerPda.toBase58());
    expect(doc.notary.toBase58()).to.eq(notaryPda.toBase58());
    expect(doc.seq.toNumber()).to.eq(seq);
    expect(doc.documentType).to.eq(documentType);
    expect(doc.propertyRef).to.eq(propertyRef);
    expect(doc.counterpartyName).to.eq(counterpartyName);

    const seqAfter = await program.account.ownerDocumentSeq.fetch(
      ownerDocSeqPda
    );
    expect(seqAfter.value.toNumber()).to.eq(seq + 1);

    if (ev) {
      expect(ev.document.toBase58()).to.eq(documentPda.toBase58());
      expect(ev.seq.toNumber()).to.eq(seq);
    }
  });

  it('UNHAPPY: grant_access by unauthorized authority -> UnauthorizedGrant', async () => {
    try {
      const badGrant = deriveGrantPda(notaryAuthority.publicKey, SCOPE_READ);
      await program.methods
        .grantAccess(SCOPE_READ)
        .accounts({
          authority: rando.publicKey,
          config: configPda,
          owner: ownerPda,
          grant: badGrant,
          grantee: notaryAuthority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([rando])
        .rpc();
      expect.fail('Expected UnauthorizedGrant');
    } catch (e: any) {
      expect(e.error?.errorCode?.code ?? `${e}`).to.include(
        'UnauthorizedGrant'
      );
    }
  });

  it('UNHAPPY: grant_access invalid scope bits -> InvalidScope', async () => {
    const badScope = ALLOWED | (1 << 6);
    try {
      const badGrant = deriveGrantPda(notaryAuthority.publicKey, badScope);
      await program.methods
        .grantAccess(badScope)
        .accounts({
          authority: ownerAuthority.publicKey,
          config: configPda,
          owner: ownerPda,
          grant: badGrant,
          grantee: notaryAuthority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([ownerAuthority])
        .rpc();
      expect.fail('Expected InvalidScope');
    } catch (e: any) {
      expect(e.error?.errorCode?.code ?? `${e}`).to.include('InvalidScope');
    }
  });

  it('HAPPY: revoke_grant by owner', async () => {
    const scope = SCOPE_READ;
    const grantPda = deriveGrantPda(notaryAuthority.publicKey, scope);

    await program.methods
      .grantAccess(scope)
      .accounts({
        authority: ownerAuthority.publicKey,
        config: configPda,
        owner: ownerPda,
        grant: grantPda,
        grantee: notaryAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([ownerAuthority])
      .rpc();

    let ev: any | null = null;
    const sub = await program.addEventListener(
      'GrantRevoked',
      (e: any) => (ev = e)
    );

    await program.methods
      .revokeGrant()
      .accounts({
        owner: ownerPda,
        grant: grantPda,
        grantee: notaryAuthority.publicKey,
        authority: ownerAuthority.publicKey,
      })
      .signers([ownerAuthority])
      .rpc();

    await program.removeEventListener(sub);

    const grant = await program.account.grant.fetch(grantPda);
    expect(grant.revoked).to.eq(true);
    expect(grant.revokedAt?.toNumber() ?? 0).to.be.greaterThan(0);

    if (ev) {
      expect(ev.grant.toBase58()).to.eq(grantPda.toBase58());
      expect(ev.owner.toBase58()).to.eq(ownerPda.toBase58());
      expect(ev.grantee.toBase58()).to.eq(notaryAuthority.publicKey.toBase58());
    }
  });

  it('UNHAPPY: revoke_grant twice -> AlreadyRevoked', async () => {
    const grantPda = deriveGrantPda(notaryAuthority.publicKey, SCOPE_READ);
    try {
      await program.methods
        .revokeGrant()
        .accounts({
          owner: ownerPda,
          grant: grantPda,
          grantee: notaryAuthority.publicKey,
          authority: ownerAuthority.publicKey,
        })
        .signers([ownerAuthority])
        .rpc();
      expect.fail('Expected AlreadyRevoked');
    } catch (e: any) {
      expect(e.error?.errorCode?.code ?? `${e}`).to.include('AlreadyRevoked');
    }
  });

  it('UNHAPPY: revoke_grant by wrong authority -> UnauthorizedRevoke', async () => {
    const grantPda = deriveGrantPda(notaryAuthority.publicKey, SCOPE_ADMIN);

    await program.methods
      .grantAccess(SCOPE_ADMIN)
      .accounts({
        authority: ownerAuthority.publicKey,
        config: configPda,
        owner: ownerPda,
        grant: grantPda,
        grantee: notaryAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([ownerAuthority])
      .rpc();

    try {
      await program.methods
        .revokeGrant()
        .accounts({
          owner: ownerPda,
          grant: grantPda,
          grantee: notaryAuthority.publicKey,
          authority: rando.publicKey,
        })
        .signers([rando])
        .rpc();
      expect.fail('Expected UnauthorizedRevoke');
    } catch (e: any) {
      expect(e.error?.errorCode?.code ?? `${e}`).to.include(
        'UnauthorizedRevoke'
      );
    }
  });
});
