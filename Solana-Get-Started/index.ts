import { getAddMemoInstruction } from "@solana-program/memo";
import { getTransferSolInstruction } from "@solana-program/system";
import {  address, createSolanaClient, createSolanaRpc, createSolanaRpcSubscriptions, createTransaction, getSignatureFromTransaction, LAMPORTS_PER_SOL, sendAndConfirmTransactionFactory, signAndSendTransactionMessageWithSigners, signTransactionMessageWithSigners } from "gill"
import { loadKeypairSignerFromFile} from "gill/node"
(async () => {

    const myKeypair = await loadKeypairSignerFromFile("~/file.json");

    const myAddress =  myKeypair.address; //address("AmitCMgWp9qNxwnkuFPRqXr2DXAxE7Rb75aNhGihC8HE");

    // const rpc = createSolanaRpc("https://api.devnet.solana.com");

    const rpcSubscriptions = createSolanaRpcSubscriptions("wss://api.devnet.solana.com/");
    // get balance ===================
    // const rpcResponse = await rpc.getAccountInfo(myAddress).send();

    const { rpc, sendAndConfirmTransaction } = createSolanaClient({ urlOrMoniker: "devnet" });
    // const accountInfo = rpcResponse.value;

    // const solAmount = Number(accountInfo?.lamports.valueOf()!) / LAMPORTS_PER_SOL;

    // console.log(`The account ${myAddress} has ${solAmount} SOL`);

    const receiver = address("AmitCMgWp9qNxwnkuFPRqXr2DXAxE7Rb75aNhGirever")
    // transfer =======================

    // need to interact 
    const transferSolInstruction = getTransferSolInstruction({
        amount: 0.001 * LAMPORTS_PER_SOL,
        source: myKeypair,
        destination: receiver
    }
    )

    const memoInstruction = getAddMemoInstruction({
        memo: "Thank you!"
    });

    const transaction = createTransaction({
        instructions: [transferSolInstruction, memoInstruction],
        feePayer: myKeypair,
        latestBlockhash: (await rpc.getLatestBlockhash().send()).value
    })

    const signedTransaction = await signTransactionMessageWithSigners(transaction);
    console.log(`signature from transaction: ${getSignatureFromTransaction(signedTransaction)}`);
    // const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({rpc, rpcSubscriptions});

    await sendAndConfirmTransaction(signedTransaction);

    console.log("Confirmed transfer of 0.001 SOL to receiver");
}

)();