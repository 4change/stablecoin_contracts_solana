import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { UserMng } from "../target/types/user_mng";
import { SystemProgram, Keypair, PublicKey } from "@solana/web3.js";

describe("user_mng program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.UserMng as Program<UserMng>;

  let userMgmtKeypair: Keypair;
  let testUserPubkey: PublicKey;

  before(async () => {
    userMgmtKeypair = Keypair.generate();
    testUserPubkey = Keypair.generate().publicKey;

    // initialize UserManagement account
    await program.rpc.initialize(provider.wallet.publicKey, {
      accounts: {
        userManagement: userMgmtKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [userMgmtKeypair],
    });
  });

  it("initializes user management account", async () => {
    const um = await program.account.userManagement.fetch(
      userMgmtKeypair.publicKey
    );
    assert.equal(
      um.owner.toString(),
      provider.wallet.publicKey.toString(),
      "owner should be set"
    );
  });

  it("adds a user", async () => {
    const name = "Alice";
    const desc = "Test user";

    await program.rpc.addUser(testUserPubkey, name, desc, {
      accounts: {
        userManagement: userMgmtKeypair.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    const um = await program.account.userManagement.fetch(
      userMgmtKeypair.publicKey
    );
    const entry = um.users.find((e: any) =>
      (e.key as PublicKey).equals(testUserPubkey)
    );
    assert.ok(entry, "user entry must exist");
    assert.equal(entry.user.name, name);
    assert.equal(entry.user.description, desc);
    assert.equal(entry.user.isWhitelisted, true);
  });

  it("updates fee rate", async () => {
    const fee = new anchor.BN(500);

    await program.rpc.setFeeRate(testUserPubkey, fee, {
      accounts: {
        userManagement: userMgmtKeypair.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    const um = await program.account.userManagement.fetch(
      userMgmtKeypair.publicKey
    );
    const entry = um.users.find((e: any) =>
      (e.key as PublicKey).equals(testUserPubkey)
    );
    assert.ok(entry, "user entry must exist");
    // feeRate is returned as BN
    assert.equal(entry.user.feeRate.toNumber(), 500);
  });

  it("updates whitelist status", async () => {
    await program.rpc.updateWhitelist(testUserPubkey, false, {
      accounts: {
        userManagement: userMgmtKeypair.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    const um = await program.account.userManagement.fetch(
      userMgmtKeypair.publicKey
    );
    const entry = um.users.find((e: any) =>
      (e.key as PublicKey).equals(testUserPubkey)
    );
    assert.ok(entry, "user entry must exist");
    assert.equal(entry.user.isWhitelisted, false);
  });

  it("removes a user", async () => {
    await program.rpc.removeUser(testUserPubkey, {
      accounts: {
        userManagement: userMgmtKeypair.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    const um = await program.account.userManagement.fetch(
      userMgmtKeypair.publicKey
    );
    const entry = um.users.find((e: any) =>
      (e.key as PublicKey).equals(testUserPubkey)
    );
    assert.isUndefined(entry, "user entry should be removed");
  });
});