// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import {PublicKey} from "@solana/web3.js";

export type MembershipSeeds = {
    plan: PublicKey, 
};

export const deriveMembershipPDA = (
    seeds: MembershipSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("membership"),
            seeds.plan.toBuffer(),
        ],
        programId,
    )
};

export type SubscriptionSeeds = {
    plan: PublicKey, 
    mint: PublicKey, 
};

export const deriveSubscriptionPDA = (
    seeds: SubscriptionSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("subscription"),
            seeds.plan.toBuffer(),
            seeds.mint.toBuffer(),
        ],
        programId,
    )
};

export module CslSplTokenPDAs {
    export type AccountSeeds = {
        wallet: PublicKey, 
        tokenProgram: PublicKey, 
        mint: PublicKey, 
    };
    
    export const deriveAccountPDA = (seeds: AccountSeeds): [PublicKey, number] => {
        return PublicKey.findProgramAddressSync(
            [
                seeds.wallet.toBuffer(),
                seeds.tokenProgram.toBuffer(),
                seeds.mint.toBuffer(),
            ],
            new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        )
    };
    
}

