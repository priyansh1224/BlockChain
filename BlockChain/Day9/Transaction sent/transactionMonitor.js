import { ethers } from 'ethers';

async function monitorTransactions() {
  console.log('🔗 Connecting to Ethereum WebSocket for transaction monitoring...');
  
  const provider = new ethers.WebSocketProvider('wss://ethereum-rpc.publicnode.com');
  
  try {
    const blockNumber = await provider.getBlockNumber();
    console.log(`✅ Connected! Current block: ${blockNumber}\n`);
    
    console.log('📡 Monitoring pending transactions...\n');
    
    let txCount = 0;
    
    // Monitor pending transactions
    provider.on('pending', (txHash) => {
      txCount++;
      console.log(`⏳ Pending TX #${txCount}: ${txHash}`);
      
      // Stop after 10 transactions
      if (txCount >= 10) {
        console.log('\n🎉 Transaction monitoring demo complete!');
        provider.destroy();
        process.exit(0);
      }
    });
    
    // Also monitor new blocks
    provider.on('block', (newBlockNumber) => {
      console.log(`📦 New block: ${newBlockNumber}`);
    });
    
    console.log('⏳ Listening for pending transactions... (will show 10 then exit)');
    
  } catch (error) {
    console.error('❌ Connection failed:', error.message);
    process.exit(1);
  }
}

monitorTransactions();