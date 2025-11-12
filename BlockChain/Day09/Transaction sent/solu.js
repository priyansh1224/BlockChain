import { Connection, Keypair, Transaction, SystemProgram, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import bs58 from 'bs58';

async function sendSolanaTransactionRaw(privateKey, to, amount) {
  const connection = new Connection(`https://go.getblock.us/bbcb5a2482ba4a86a3d1e633fcdb36fe`);
  
  const fromKeypair = Keypair.fromSecretKey(bs58.decode(privateKey));
  const toPublicKey = new PublicKey(to);
  
  try {
    // Get recent blockhash
    const latestBlockhash = await connection.getLatestBlockhash();
    
    // Create transaction
    const transaction = new Transaction();
    transaction.recentBlockhash = latestBlockhash.blockhash;
    transaction.feePayer = fromKeypair.publicKey;
    
    // Add the transfer instruction
    transaction.add(
      SystemProgram.transfer({
        fromPubkey: fromKeypair.publicKey,
        toPubkey: toPublicKey,
        lamports: amount * LAMPORTS_PER_SOL
      })
    );
    
    // Sign the transaction
    transaction.sign(fromKeypair);
    
    // Send raw transaction
    const signature = await connection.sendRawTransaction(
      transaction.serialize(),
      {
        skipPreflight: false,
        preflightCommitment: 'confirmed'
      }
    );
    
    console.log('Transaction sent:', signature);
    
    // Confirm transaction
    const confirmation = await connection.confirmTransaction({
      signature,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
    });
    
    if (confirmation.value.err) {
      throw new Error(`Transaction failed: ${confirmation.value.err}`);
    }
    
    console.log('Transaction confirmed:', confirmation);
    
    return signature;
  } catch (error) {
    console.error('Transaction failed:', error);
    throw error;
  }
}


sendSolanaTransactionRaw("62Nxioza63NMNTVKBHbLbKRqtwqu96JpU7XnX31dnAK2F2TuFfuyNxtaCynsbHqouCoU6ku1asdtRBbiCFN74QvL","APz7hnkedimxjWMLMfP42epujhoHWWDGMyGV4ghYXdjc",0.001);


