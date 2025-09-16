import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { UserMng } from "../target/types/user_mng";
import { SystemProgram, Keypair, PublicKey } from "@solana/web3.js";

describe("user_mng program", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.UserMng as Program<UserMng>;

    let testUserKeypair: Keypair;
    let userPda: PublicKey;
    let configPda: PublicKey;

    before(async () => {
        testUserKeypair = Keypair.generate();

        // derive config PDA and initialize
        const [cfgPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("config")],
            program.programId
        );
        configPda = cfgPda;

        // only initialize if config PDA does not exist yet
        let configExists = true;
        try {
            await program.account.config.fetch(configPda);
        } catch (err) {
            configExists = false;
        }
        if (!configExists) {
            await program.rpc.initialize(provider.wallet.publicKey, {
                accounts: { config: configPda, payer: provider.wallet.publicKey, systemProgram: SystemProgram.programId },
            });
        }

        // derive user PDA for test user
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user"), testUserKeypair.publicKey.toBuffer()],
            program.programId
        );
        userPda = pda;
    });

    it("creates a user", async () => {
        const name = "Alice";
        const desc = "Test user";

        await program.rpc.createUser(name, desc, {
            accounts: {
                userAccount: userPda,
                userKey: testUserKeypair.publicKey,
                payer: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            },
        });

        const user = await program.account.userAccount.fetch(userPda);
        assert.equal(user.owner.toString(), testUserKeypair.publicKey.toString());
        assert.equal(user.name, "Alice");
        assert.equal(user.description, "Test user");
        assert.equal(user.isWhitelisted, true);
    });

    it("sets fee rate (admin)", async () => {
        const fee = new anchor.BN(500);

        await program.rpc.setFeeRate(fee, {
            accounts: {
                config: configPda,
                authority: provider.wallet.publicKey,
                userAccount: userPda,
                userKey: testUserKeypair.publicKey,
            },
        });

        const user = await program.account.userAccount.fetch(userPda);
        assert.equal((user.feeRateBps as anchor.BN).toNumber(), 500);
    });

    it("updates whitelist status (admin)", async () => {
        await program.rpc.updateWhitelist(false, {
            accounts: {
                config: configPda,
                authority: provider.wallet.publicKey,
                userAccount: userPda,
                userKey: testUserKeypair.publicKey,
            },
        });

        const user = await program.account.userAccount.fetch(userPda);
        assert.equal(user.isWhitelisted, false);
    });

    it("removes user", async () => {
        await program.rpc.removeUser({
            accounts: {
                userAccount: userPda,
                userKey: testUserKeypair.publicKey,
                payer: provider.wallet.publicKey,
            },
        });

        try {
            await program.account.userAccount.fetch(userPda);
            assert.fail("account should be closed");
        } catch (err: any) {
            assert.include(err.message, "Account does not exist");
        }
    });
});