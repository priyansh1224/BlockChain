import { ethers } from 'ethers';

async function checkBalance(privateKey) {
  const provider = new ethers.JsonRpcProvider(`https://go.getblock.us/81990708e37a492c89af1f1b7a82cb9a`);
  const wallet = new ethers.Wallet(privateKey, provider);
  
  const balance = await provider.getBalance(wallet.address);
  const balanceInEth = ethers.formatEther(balance);
  
  console.log(`Address: ${wallet.address}`);
  console.log(`Balance: ${balanceInEth} ETH`);
  console.log(`Balance: ${balance.toString()} wei`);
}

checkBalance("0xe13e0a935240c2ac98f19ac0299ee41a1841ca6860a99c858ea55a69107ac61a");