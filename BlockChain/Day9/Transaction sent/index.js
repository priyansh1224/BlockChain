import { ethers } from 'ethers';

async function sendTransactionWithWebSocket(privateKey, to, amount) {
  let provider;
  
  try {
    // Try WebSocket first, fallback to HTTP if needed
    console.log('🔗 Attempting WebSocket connection...');
    provider = new ethers.WebSocketProvider('wss://ethereum-rpc.publicnode.com');
    
    // Test connection
    await provider.getBlockNumber();
    console.log('✅ WebSocket connected successfully!');
    
  } catch (wsError) {
    console.log('⚠️  WebSocket failed, falling back to HTTP RPC...');
    provider = new ethers.JsonRpcProvider('https://ethereum-rpc.publicnode.com');
  }
  
  // Create wallet connected to provider
  const wallet = new ethers.Wallet(privateKey, provider);
  
  // Set up real-time event listeners (only for WebSocket)
  if (provider instanceof ethers.WebSocketProvider) {
    console.log('📡 Setting up real-time block monitoring...');
    
    provider.on('block', (blockNumber) => {
      console.log(`📦 New block: ${blockNumber}`);
    });
    
    provider.on('error', (error) => {
      console.error('🔴 WebSocket error:', error.message);
    });
  }
  
  try {
    console.log(`📍 Wallet address: ${wallet.address}`);
    
    // Check balance first
    const balance = await provider.getBalance(wallet.address);
    console.log(`💰 Current balance: ${ethers.formatEther(balance)} ETH`);
    
    if (balance === 0n) {
      throw new Error('Insufficient funds: Wallet balance is 0 ETH');
    }
    
    // Send transaction
    const tx = await wallet.sendTransaction({
      to: to,
      value: ethers.parseEther(amount)
    });
    
    console.log('🚀 Transaction sent:', tx.hash);
    console.log('⏳ Waiting for confirmation...');
    
    // For WebSocket, listen for real-time confirmation
    if (provider instanceof ethers.WebSocketProvider) {
      provider.once(tx.hash, (receipt) => {
        console.log('✅ Transaction confirmed via WebSocket!');
        console.log(`📦 Block: ${receipt.blockNumber}`);
        console.log(`⛽ Gas used: ${receipt.gasUsed}`);
      });
    }
    
    // Wait for confirmation
    const receipt = await tx.wait();
    console.log('🎉 Transaction finalized in block:', receipt.blockNumber);
    console.log(`💸 Total gas cost: ${ethers.formatEther(receipt.gasUsed * receipt.gasPrice)} ETH`);
    
    return tx.hash;
    
  } catch (error) {
    console.error('❌ Transaction failed:', error.shortMessage || error.message);
    throw error;
  } finally {
    // Clean up WebSocket connection
    if (provider instanceof ethers.WebSocketProvider) {
      setTimeout(() => {
        provider.destroy();
        console.log('🔌 WebSocket connection closed');
        process.exit(0);
      }, 3000);
    } else {
      setTimeout(() => process.exit(0), 1000);
    }
  }
}

// Execute with WebSocket (with fallback)
sendTransactionWithWebSocket(
  "0xe13e0a935240c2ac98f19ac0299ee41a1841ca6860a99c858ea55a69107ac61a",
  "0xC81cAe7D1cf6FCe3E3ac0B2d41E871D149ab777A",
  "0.000000177125054145"
);