import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { VotingManager } from "../target/types/voting_manager";
import { Voting } from "../target/types/voting";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { Clock } from "solana-bankrun";

describe("Voting System Full Flow", () => {
    let context;
    let provider;
    let managerProgram: Program<VotingManager>;
    let votingProgram: Program<Voting>;
    let authority: Keypair;

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
    async function setClock(context: any, newTimestamp: number | bigint) {
  const client = context.banksClient;
  const oldClock = await client.getClock();

  const newClock = new Clock(
    oldClock.slot,
    oldClock.epochStartTimestamp,
    oldClock.epoch,
    oldClock.leaderScheduleEpoch,
    BigInt(newTimestamp), //tuż przed zakończeniem głosowania: 1700009999 , równo zakończenie możliwości głosowania: 1700010000
  );

  context.setClock(newClock);
  return newClock;
}
function logClock(clock: Clock) {
  console.log("---------------------------");
  console.log("Aktualny czas w symulacji (Unix Timestamp):", clock.unixTimestamp);
  console.log("Aktualny czas w symulacji:", new Date(Number(clock.unixTimestamp) * 1000).toLocaleString());
  console.log("---------------------------");
  return clock.unixTimestamp;
}
it("Wyświetla tylko aktualny czas blockchaina", async () => {
       const clockBefore = await context.banksClient.getClock();
        logClock(clockBefore);
        const startTs = clockBefore.unixTimestamp + 5n;
        const endTs = startTs + 1000n;
        const startTime = new BN(startTs.toString());
        const endTime = new BN(endTs.toString());
        console.log(startTime);
        console.log(endTime);
    });
   it("1. Tworzy event i wyświetla jego dane", async () => {
        const [managerPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("manager_seed"), authority.publicKey.toBuffer()],
            managerProgram.programId
        );

        // Inicjalizacja managera (jeśli nie był zainicjalizowany wcześniej)
        try {
            await managerProgram.methods
                .initializeManager()
                .accounts({
                    authority: authority.publicKey,
                    manager: managerPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([authority])
                .rpc();
        } catch (e) {
            console.log("Manager już zainicjalizowany lub błąd inicjalizacji");
        }

        const pollId = new BN(0);
        const pollName = "Głosowanie na Przewodniczącego";
        const candidateNames = ["Alice", "Bob"];

        const [pollPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("poll_seed"), pollId.toArrayLike(Buffer, "le", 8)],
            votingProgram.programId
        );

        const candidateRemainingAccounts = candidateNames.map(name => {
            const [pda] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("candidate_seed"),
                    pollId.toArrayLike(Buffer, "le", 8),
                    Buffer.from(name)
                ],
                votingProgram.programId
            );
            return { pubkey: pda, isWritable: true, isSigner: false };
        });
        // funkcja do wyświetlania aktualnego czasu
        const clockBefore = await context.banksClient.getClock();
        logClock(clockBefore);
        const startTs = clockBefore.unixTimestamp + 1n;
        const endTs = startTs + 1000n;
        const startTime = new BN(startTs.toString());
        const endTime = new BN(endTs.toString());


        await managerProgram.methods
            .createEvent(startTime, endTime, pollName, "Opis ankiety", candidateNames)
            .accounts({
                authority: authority.publicKey,
                manager: managerPda,
                poll: pollPda,
                votingProgram: votingProgram.programId,
                systemProgram: SystemProgram.programId,
            })
            .remainingAccounts(candidateRemainingAccounts)
            .signers([authority])
            .rpc();

        // --- WYŚWIETLANIE EVENTU ---
        const pollAccount = await votingProgram.account.poll.fetch(pollPda);
        
        console.log("\n--- DANE STWORZONEGO EVENTU ---");
        console.log(`Nazwa: ${pollAccount.pollName}`);
        console.log(`Opis:  ${pollAccount.pollDescription}`);
        console.log(`Start: ${new Date(pollAccount.startTime.toNumber() * 1000).toLocaleString()}`);
        console.log(`Koniec: ${new Date(pollAccount.endTime.toNumber() * 1000).toLocaleString()}`);
        console.log("-------------------------------\n");
console.log("\n--- KANDYDACI (OPCJE) ---");

        // Musimy przejść przez listę nazw, które podaliśmy przy tworzeniu
        for (const name of candidateNames) {
            const [cPda] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("candidate_seed"),
                    pollId.toArrayLike(Buffer, "le", 8),
                    Buffer.from(name)
                ],
                votingProgram.programId
            );

            try {
                const candData = await votingProgram.account.candidate.fetch(cPda);
                console.log(`- [ ] ${candData.candidateName.padEnd(10)} | Głosów: ${candData.candidateVotes.toNumber()}`);
            } catch (e) {
                console.log(`- [ ] ${name.padEnd(10)} | (Nie zainicjalizowano)`);
            }
        }
    });


const pollId = new BN(0);
    const candidateName = "Alice";

    // Pomocnicza funkcja do pobierania PDA wewnątrz testów
    const getPdas = () => {
        const [pollPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("poll_seed"), pollId.toArrayLike(Buffer, "le", 8)],
            votingProgram.programId
        );
        const [candidatePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("candidate_seed"), pollId.toArrayLike(Buffer, "le", 8), Buffer.from(candidateName)],
            votingProgram.programId
        );
        return { pollPda, candidatePda };
    };

    it("❌ Nie można oddać głosu PRZED rozpoczęciem", async () => {
        const { pollPda, candidatePda } = getPdas();
        const pollAcc = await votingProgram.account.poll.fetch(pollPda);
        
        // Ustawiamy czas na 1 sekundę PRZED startem
        const targetTime = BigInt(pollAcc.startTime.sub(new BN(1)).toString());
        await setClock(context, targetTime);
        
        try {
            await votingProgram.methods.vote(pollId, candidateName)
                .accounts({ participant: authority.publicKey, poll: pollPda, candidate: candidatePda, systemProgram: SystemProgram.programId })
                .rpc();
            assert.fail("Powinien wystąpić błąd VotingNotStarted");
        } catch (err) {
            assert.include(err.toString(), "VotingNotStarted"); // Kod 6002
            console.log("✅ Prawidłowo zablokowano głosowanie przed czasem.");
        }
    });

    it("✅ Można oddać głos W TRAKCIE (2 minuty po starcie)", async () => {
        const { pollPda, candidatePda } = getPdas();
        const pollAcc = await votingProgram.account.poll.fetch(pollPda);
        
        // Ustawiamy czas na Start + 120s
        const targetTime = BigInt(pollAcc.startTime.add(new BN(120)).toString());
        await setClock(context, targetTime);

        await votingProgram.methods.vote(pollId, candidateName)
            .accounts({ participant: authority.publicKey, poll: pollPda, candidate: candidatePda, systemProgram: SystemProgram.programId })
            .rpc();

        const candData = await votingProgram.account.candidate.fetch(candidatePda);
        assert.strictEqual(candData.candidateVotes.toNumber(), 1);
        console.log("✅ Głos oddany poprawnie w trakcie trwania.");
    });

    it("❌ Nie można oddać głosu DRUGI RAZ przez tego samego użytkownika", async () => {
        const { pollPda, candidatePda } = getPdas();
        // Czas zostaje ten sam co w poprzednim teście (wciąż trwa)

        try {
            await votingProgram.methods.vote(pollId, candidateName)
                .accounts({ participant: authority.publicKey, poll: pollPda, candidate: candidatePda, systemProgram: SystemProgram.programId })
                .rpc();
            assert.fail("Powinien wystąpić błąd VoteHaveBeenPlaced");
        } catch (err) {
            assert.include(err.toString(), "VoteHaveBeenPlaced");
            console.log("✅ Prawidłowo zablokowano ponowne głosowanie.");
        }
    });

    it("❌ Nie można oddać głosu PO zakończeniu", async () => {
        const { pollPda, candidatePda } = getPdas();
        const pollAcc = await votingProgram.account.poll.fetch(pollPda);
        
        // 1. Ustawiamy czas na 1 sekundę PO końcu (np. 3:57 PM)
        const targetTime = BigInt(pollAcc.endTime.add(new BN(1)).toString());
        await setClock(context, targetTime);

        // 2. GENERUJEMY NOWEGO UŻYTKOWNIKA (żeby uniknąć VoteHaveBeenPlaced)
        const newVoter = Keypair.generate();
        
        // W Bankrun musimy zasilić nowe konto, aby mogło podpisać transakcję
        await context.setAccount(newVoter.publicKey, {
            lamports: 1_000_000_000,
            data: Buffer.alloc(0),
            owner: SystemProgram.programId,
            executable: false,
            rentEpoch: 0,
        });

        // 3. PDA dla głosu nowego użytkownika
        const [newVoterVotePda] = PublicKey.findProgramAddressSync(
            [newVoter.publicKey.toBuffer(), pollId.toArrayLike(Buffer, "le", 8)],
            votingProgram.programId
        );

        try {
            await votingProgram.methods.vote(pollId, candidateName)
                .accounts({ 
                    participant: newVoter.publicKey, // Nowy głosujący!
                    poll: pollPda, 
                    candidate: candidatePda, 
                    systemProgram: SystemProgram.programId 
                })
                .signers([newVoter]) // Musi podpisać
                .rpc();
            
            assert.fail("Powinien wystąpić błąd VotingEnded");
        } catch (err) {
            // Logujemy błąd, aby zobaczyć co dokładnie zwraca
            console.log("Złapany błąd:", err.toString());
            assert.include(err.toString(), "VotingEnded");
        }
    });
});