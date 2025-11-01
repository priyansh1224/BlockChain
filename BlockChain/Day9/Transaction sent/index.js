import { ethers } from 'ethers';

async function sendTransactionWithProvider(privateKey, to, amount) {
  // Create provider using GetBlock
  const provider = new ethers.JsonRpcProvider(`https://go.getblock.us/81990708e37a492c89af1f1b7a82cb9a`);
  
  // Create wallet connected to provider
  const wallet = new ethers.Wallet(privateKey, provider);
  
  try {
    // Send transaction (ethers handles nonce, gas price, gas Limit, sign etc.)
    const tx = await wallet.sendTransaction({
      to: to,
      value: ethers.parseEther(amount)
    });
    
    console.log('Transaction sent:', tx.hash);
    
    // Wait for confirmation
    const receipt = await tx.wait();
    console.log('Transaction confirmed in block:', receipt.blockNumber);
    
    return tx.hash;
  } catch (error) {
    console.error('Transaction failed:', error);
    throw error;
  }
}


sendTransactionWithProvider("0xe13e0a935240c2ac98f19ac0299ee41a1841ca6860a99c858ea55a69107ac61a","0xC81cAe7D1cf6FCe3E3ac0B2d41E871D149ab777A","0.000000177125054145");