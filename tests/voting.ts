import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { VotingManager } from "../target/types/voting_manager";
import { Voting } from "../target/types/voting";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { assert, expect } from "chai";
import { Clock } from "solana-bankrun";

describe("Voting System - Final Optimized Flow", () => {
    let context: any;
    let provider: BankrunProvider;
    let managerProgram: Program<VotingManager>;
    let votingProgram: Program<Voting>;
    let authority: Keypair;

    const pollId = new BN(0);
    const candidateNames = ["Alice", "Bob"];
    const mainCandidate = "Alice";

    const VOTING_IDL = require("../target/idl/voting.json");
    const MANAGER_IDL = require("../target/idl/voting_manager.json");

    before(async () => {
        authority = Keypair.generate();
        context = await startAnchor(process.cwd(), [
            { name: "voting_manager", programId: new PublicKey("HqFLnX37E2rVeU3QN9ZdiKHiQrRj5C1t3Sqzyvy9HZES") },
            { name: "voting", programId: new PublicKey("3QtBbSDvHi2wAZe1akqUSbbWQ2VSN9iADkqsTgT6J5SR") }
        ], [
            {
                address: authority.publicKey,
                info: { lamports: 10 * 10 ** 9, data: Buffer.alloc(0), owner: SystemProgram.programId, executable: false, rentEpoch: 0 }
            }
        ]);

        provider = new BankrunProvider(context);
        provider.wallet = new anchor.Wallet(authority);
        managerProgram = new Program<VotingManager>(MANAGER_IDL, provider);
        votingProgram = new Program<Voting>(VOTING_IDL, provider);
    });

    // --- REFAKTORYZACJA FUNKCJI POMOCNICZYCH ---

    const getPdas = () => {
        const [managerPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("manager_seed"), authority.publicKey.toBuffer()],
            managerProgram.programId
        );
        const [pollPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("poll_seed"), pollId.toArrayLike(Buffer, "le", 8)],
            votingProgram.programId
        );
        const [candidatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("candidate_seed"), pollId.toArrayLike(Buffer, "le", 8), Buffer.from(mainCandidate)],
            votingProgram.programId
        );
        
        // Dynamiczne generowanie wszystkich kandydatów dla remainingAccounts
        const candidateRemainingAccounts = candidateNames.map(name => {
            const [pda] = PublicKey.findProgramAddressSync(
                [Buffer.from("candidate_seed"), pollId.toArrayLike(Buffer, "le", 8), Buffer.from(name)],
                votingProgram.programId
            );
            return { pubkey: pda, isWritable: true, isSigner: false };
        });

        return { managerPda, pollPda, candidatePda, candidateRemainingAccounts };
    };

    async function jumpToTime(timestamp: number | BN) {
        const ts = timestamp instanceof BN ? timestamp.toNumber() : timestamp;
        const oldClock = await context.banksClient.getClock();
        const newClock = new Clock(
            oldClock.slot,
            oldClock.epochStartTimestamp,
            oldClock.epoch,
            oldClock.leaderScheduleEpoch,
            BigInt(ts),
        );
        context.setClock(newClock);
    }

    // --- TESTY ---

    it("1. Inicjalizuje Manager i tworzy Event", async () => {
        const { managerPda, pollPda, candidateRemainingAccounts } = getPdas();

        await managerProgram.methods.initializeManager().accounts({
            authority: authority.publicKey,
            manager: managerPda,
            systemProgram: SystemProgram.programId,
        }).rpc();

        const clock = await context.banksClient.getClock();
        const startTime = new BN((clock.unixTimestamp + 5n).toString());
        const endTime = new BN((clock.unixTimestamp + 1000n).toString());

        await managerProgram.methods
            .createEvent(startTime, endTime, "Wybory 2026", "Głosowanie testowe", candidateNames)
            .accounts({
                authority: authority.publicKey,
                manager: managerPda,
                poll: pollPda,
                votingProgram: votingProgram.programId,
                systemProgram: SystemProgram.programId,
            })
            .remainingAccounts(candidateRemainingAccounts)
            .rpc();

        console.log("✅ Event i Kandydaci utworzeni poprawnie.");
    });

    it("❌ Głosowanie: Blokada czasowa oraz masowe głosowanie", async () => {
        const { pollPda, candidatePda } = getPdas();
        const pollAcc = await votingProgram.account.poll.fetch(pollPda);

        // --- 1. TEST: PRZED ROZPOCZĘCIEM ---
        await jumpToTime(pollAcc.startTime.subn(1));
        try {
            await votingProgram.methods.vote(pollId, mainCandidate)
                .accounts({ participant: authority.publicKey, poll: pollPda, candidate: candidatePda, systemProgram: SystemProgram.programId })
                .rpc();
            assert.fail("Nie powinno pozwolić na głosowanie przed startem");
        } catch (e) {
            assert.include(e.toString(), "VotingNotStarted");
        }

        // --- 2. TEST: W TRAKCIE (Głosowanie Authority + Masowe) ---
        await jumpToTime(pollAcc.startTime.addn(10));
        
        // Pierwszy głos (Authority)
        await votingProgram.methods.vote(pollId, mainCandidate)
            .accounts({ participant: authority.publicKey, poll: pollPda, candidate: candidatePda, systemProgram: SystemProgram.programId })
            .rpc();
        console.log("✅ Authority oddał głos na Alice.");

        // DODATKOWE 5 GŁOSÓW
        const extraVoters = Array.from({ length: 5 }, () => Keypair.generate());
        console.log("--- Symulacja 5 dodatkowych głosów ---");
        
        for (const [index, voter] of extraVoters.entries()) {
            // Zasilamy konto wyborcy
            await context.setAccount(voter.publicKey, {
                lamports: 10**9,
                data: Buffer.alloc(0),
                owner: SystemProgram.programId,
                executable: false,
                rentEpoch: 0
            });

            // Losujemy kandydata z listy candidateNames
            const votedFor = candidateNames[Math.floor(Math.random() * candidateNames.length)];
            const [votedCandidatePda] = PublicKey.findProgramAddressSync(
                [Buffer.from("candidate_seed"), pollId.toArrayLike(Buffer, "le", 8), Buffer.from(votedFor)],
                votingProgram.programId
            );

            await votingProgram.methods.vote(pollId, votedFor)
                .accounts({ 
                    participant: voter.publicKey, 
                    poll: pollPda, 
                    candidate: votedCandidatePda, 
                    systemProgram: SystemProgram.programId 
                })
                .signers([voter])
                .rpc();
            
            console.log(`Wyborca #${index + 1} oddał głos na: ${votedFor}`);
        }

        // --- 3. TEST: PO ZAKOŃCZENIU ---
        await jumpToTime(pollAcc.endTime.addn(1));
        const newVoterPoCzasie = Keypair.generate();
        await context.setAccount(newVoterPoCzasie.publicKey, {
            lamports: 10**8, data: Buffer.alloc(0), owner: SystemProgram.programId, executable: false, rentEpoch: 0
        });

        try {
            await votingProgram.methods.vote(pollId, mainCandidate)
                .accounts({ participant: newVoterPoCzasie.publicKey, poll: pollPda, candidate: candidatePda, systemProgram: SystemProgram.programId })
                .signers([newVoterPoCzasie])
                .rpc();
            assert.fail("Nie powinno pozwolić na głosowanie po zakończeniu");
        } catch (e) {
            assert.include(e.toString(), "VotingEnded");
        }
        
        console.log("✅ Blokady czasowe i masowe głosowanie przetestowane pomyślnie.");
    });

    it("5. Rozstrzyga, odzyskuje SOL i wyświetla zwycięzcę", async () => {
        const { managerPda, pollPda, candidateRemainingAccounts } = getPdas();
        
        // Sprawdzamy saldo przed zamknięciem
        const balanceBefore = await context.banksClient.getBalance(authority.publicKey);

        const ixs = await managerProgram.methods
            .resolveEvent(pollId)
            .accounts({
                authority: authority.publicKey,
                manager: managerPda,
                poll: pollPda,
                votingProgram: votingProgram.programId,
                systemProgram: SystemProgram.programId,
            })
            .remainingAccounts(candidateRemainingAccounts)
            .instruction();

        const tx = new Transaction().add(ixs);
        tx.recentBlockhash = context.lastBlockhash;
        tx.feePayer = authority.publicKey;
        tx.sign(authority);

        const txResult = await context.banksClient.processTransaction(tx);
        const logs = txResult.logMessages;

        // Wyświetlanie sformatowane
        console.log("\n" + "╔" + "═".repeat(43) + "╗");
        console.log("║         📊 FINALNE WYNIKI ANKIETY         ║");
        console.log("╠" + "═".repeat(43) + "╣");
        
        logs.filter(l => l.includes("Kandydat:")).forEach(l => {
            console.log("║ " + l.split("Program log: ")[1].padEnd(41) + " ║");
        });

        const winnerLog = logs.find(l => l.includes("ZWYCIĘZCA:"));
        if (winnerLog) {
            console.log("╠" + "═".repeat(43) + "╣");
            console.log("║ 🏆 " + winnerLog.split("Program log: ")[1].padEnd(38) + " ║");
        }
        console.log("╚" + "═".repeat(43) + "╝\n");

        // Weryfikacja zamknięcia kont i salda
        const pollAccount = await context.banksClient.getAccount(pollPda);
        const balanceAfter = await context.banksClient.getBalance(authority.publicKey);

        expect(pollAccount).to.be.null;
        expect(Number(balanceAfter)).to.be.greaterThan(Number(balanceBefore));
        
        const recovered = (Number(balanceAfter) - Number(balanceBefore)) / 10**9;
        console.log(`✅ Sukces! Odzyskano około ${recovered.toFixed(4)} SOL z Rent.`);
    });
});