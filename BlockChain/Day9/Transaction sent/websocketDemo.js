import { ethers } from 'ethers';

async function websocketDemo() {
  console.log('🔗 Connecting to Ethereum WebSocket...');
  
  // Create WebSocket provider
  const provider = new ethers.WebSocketProvider('wss://ethereum-rpc.publicnode.com');
  
  try {
    // Test connection
    const blockNumber = await provider.getBlockNumber();
    console.log(`✅ Connected! Current block: ${blockNumber}`);
    
    // Real-time block monitoring
    console.log('📡 Starting real-time block monitoring...\n');
    
    let blockCount = 0;
    provider.on('block', (newBlockNumber) => {
      blockCount++;
      console.log(`📦 Block #${newBlockNumber} (${blockCount} blocks received)`);
      
      // Stop after 5 blocks
      if (blockCount >= 5) {
        console.log('\n🎉 Demo complete! WebSocket is working perfectly.');
        provider.destroy();
        process.exit(0);
      }
    });
    
    // Error handling
    provider.on('error', (error) => {
      console.error('🔴 WebSocket error:', error.message);
    });
    
    // Keep alive for demo
    console.log('⏳ Waiting for new blocks... (will show 5 blocks then exit)');
    
  } catch (error) {
    console.error('❌ Connection failed:', error.message);
    process.exit(1);
  }
}

websocketDemo();