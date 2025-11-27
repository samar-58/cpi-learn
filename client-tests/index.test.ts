import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import {test, beforeAll, expect} from "bun:test"
import { LiteSVM } from "litesvm"

test("Should create pda",()=>{
const svm = new LiteSVM();
const contract = Keypair.generate();
const payer = Keypair.generate();
svm.addProgramFromFile(contract.publicKey,"./cpi.so");

svm.airdrop(payer.publicKey,BigInt(2*LAMPORTS_PER_SOL));
const [pda,bump]=PublicKey.findProgramAddressSync([Buffer.from("user"),payer.publicKey.toBuffer()],contract.publicKey);

let ix = new TransactionInstruction({
keys:[
    {pubkey: payer.publicKey,
        isSigner:true,
isWritable:true
    },
    {pubkey: pda,
        isSigner:false,
isWritable:true
    },
    {pubkey: SystemProgram.programId,
        isSigner:false,
isWritable:false
    },
],
programId:contract.publicKey
})
const tx = new Transaction()
tx.add(ix);
const blockhash = svm.latestBlockhash()
tx.recentBlockhash = blockhash;
tx.feePayer = payer.publicKey;
tx.sign(payer);
let res = svm.sendTransaction(tx);
console.log(res.toString());

    const balance = svm.getBalance(pda);
    console.log("balance", balance);
    
    expect(balance).toBe(BigInt(LAMPORTS_PER_SOL))
})