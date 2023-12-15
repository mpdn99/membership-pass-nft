import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import * as fs from "fs/promises";
import * as path from "path";
import * as os from "os";
import {
    burnSendAndConfirm,
    CslSplTokenPDAs,
    deriveSubscriptionPDA,
    decodeMembership,
    getSubscription,
    getMembership,
    initializeClient,
    mintSendAndConfirm,
    transferSendAndConfirm,
    deriveMembershipPDA,
    addPlanSendAndConfirm,
} from "./index";
import {getMinimumBalanceForRentExemptAccount, getMint, TOKEN_PROGRAM_ID,} from "@solana/spl-token";

async function main(feePayer: Keypair) {
    const args = process.argv.slice(2);
    const connection = new Connection("https://api.devnet.solana.com", {
        commitment: "confirmed"
    })

    const progId = new PublicKey(args[0]!);

    initializeClient(progId, connection);

    const membership = Keypair.generate();
    console.info("+==== Membership plan Address  ====+");
    console.info(membership.publicKey.toBase58());


    const mint = Keypair.generate();
    console.info("+==== Mint Address  ====+");
    console.info(mint.publicKey.toBase58());

    const systemWallet = Keypair.generate();
    console.info("+==== System Wallet ====+");
    console.info(systemWallet.publicKey.toBase58());

    const userWallet = Keypair.generate();
    console.info("+==== User Wallet ====+");
    console.info(userWallet.publicKey.toBase58());

    const rent = await getMinimumBalanceForRentExemptAccount(connection);
    await sendAndConfirmTransaction(
        connection,
        new Transaction()
            .add(
                SystemProgram.createAccount({
                    fromPubkey: feePayer.publicKey,
                    newAccountPubkey: systemWallet.publicKey,
                    space: 0,
                    lamports: rent,
                    programId: SystemProgram.programId,
                })
            )
            .add(
                SystemProgram.createAccount({
                    fromPubkey: feePayer.publicKey,
                    newAccountPubkey: userWallet.publicKey,
                    space: 0,
                    lamports: rent,
                    programId: SystemProgram.programId
                })
            ),
            [feePayer, systemWallet, userWallet]
    )

    const [systemATA] = CslSplTokenPDAs.deriveAccountPDA({
        wallet: systemWallet.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
    })
    console.info("+==== System ATA ====+");
    console.info(systemATA.toBase58());

    const [userATA] = CslSplTokenPDAs.deriveAccountPDA({
        wallet: userWallet.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
    });
    console.info("+==== User ATA ====+");
    console.info(userATA.toBase58());

    const [membershipPub] = deriveMembershipPDA(
        {
            plan: membership.publicKey,
        },
        progId
    )
    console.info("+==== Membership Address ====+");
    console.info(membershipPub.toBase58());


    const [subscriptionPub] = deriveSubscriptionPDA(
        {
            plan: membership.publicKey,
            mint: mint.publicKey,
        },
        progId
    )
    console.info("+==== Subscription Address ====+");
    console.info(subscriptionPub.toBase58());

    console.info("+==== Adding Plan... ====+");
    await addPlanSendAndConfirm({
        plan: membership.publicKey,
        name: "free tier",
        shortDescription: 'free tier membership subscription',
        price: BigInt(0.03 * 10 ** 9),
        signers: {
            feePayer: feePayer,
        }
    })
    console.info("+==== Added Plan ====+");

    console.info("+==== Minting... ====+");
    await mintSendAndConfirm({
        wallet: systemWallet.publicKey,
        plan: membership.publicKey,
        assocTokenAccount: systemATA,
        signers: {
            feePayer: feePayer,
            funding: feePayer,
            mint: mint,
            owner: systemWallet,
        }
    })
    console.info("+==== Minted ====+");

    let mintAccount = await getMint(connection, mint.publicKey);
    console.info("+==== Mint ====+");
    console.info(mintAccount);

    let subscription = await getSubscription(subscriptionPub);
    console.info("+==== memebership Metadata ====+");
    console.info(subscription);

    console.info("+==== Transferring... ====+");
    await transferSendAndConfirm({
        plan: membership.publicKey,
        wallet: userWallet.publicKey,
        assocTokenAccount: userATA,
        mint: mint.publicKey,
        source: systemATA,
        destination: userATA,
        signers: {
            feePayer: feePayer,
            funding: feePayer,
            authority: systemWallet,
        },
    });
    console.info("+==== Transferred ====+");

    mintAccount = await getMint(connection, mint.publicKey);
    console.info("+==== Mint ====+");
    console.info(mintAccount);

    subscription = await getSubscription(subscriptionPub);
    console.info("+==== membership Metadata ====+");
    console.info(subscription);

    console.info("+==== Burning... ====+");
    await burnSendAndConfirm({
        plan: membership.publicKey,
        mint: mint.publicKey,
        wallet: userWallet.publicKey,
        signers: {
            feePayer: feePayer,
            owner: userWallet,
        },
    });
    console.info("+==== Burned ====+");

    mintAccount = await getMint(connection, mint.publicKey);
    console.info("+==== Mint ====+");
    console.info(mintAccount);

    subscription = await getSubscription(subscriptionPub);
    console.info("+==== daky Metadata ====+");
    console.info(subscription);
}

fs.readFile(path.join(os.homedir(), ".config/solana/id.json")).then((file) =>
    main(Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString())))),
);
