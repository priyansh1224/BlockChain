const bip39 = require('bip39');
const ethers = require('ethers');
const bitcoin = require('bitcoinjs-lib');
const { BIP32Factory } = require('bip32');
const ecc = require('tiny-secp256k1');
const { Keypair } = require('@solana/web3.js');
const { derivePath } = require('ed25519-hd-key');


const bip32 = BIP32Factory(ecc);


function deriveEthereumWallet(seed) {
  const ethPath = "m/44'/60'/0'/0/0";
  const rootNode = ethers.HDNodeWallet.fromSeed(seed);
  const ethNode = rootNode.derivePath(ethPath);

  console.log("\n--- Ethereum ---");
  console.log("Derivation Path:   ", ethPath);
  console.log("Private Key:       ", ethNode.privateKey);
  console.log("Public Key:        ", ethNode.publicKey);
  console.log("Address:           ", ethNode.address);
}


function deriveBitcoinWallet(seed) {
  const btcPath = "m/44'/0'/0'/0/0";
  
  const rootNode = bip32.fromSeed(seed);
  const btcNode = rootNode.derivePath(btcPath);
  const btcAddress = bitcoin.payments.p2pkh({
    pubkey: Buffer.from(btcNode.publicKey),
  }).address;


  // publicKey = uint8[12,123,1,21,123,1,4,23,211,6,8,9] == hexdecimal

  const publicKey = Array.from(btcNode.publicKey)
  .map(byte => byte.toString(16).padStart(2, '0'))
  .join('');

  console.log("\n--- Bitcoin ---");
  console.log("Derivation Path:   ", btcPath);
  console.log("Private Key (WIF): ", btcNode.toWIF());
  console.log("Public Key:        ", publicKey);
  console.log("Address:           ", btcAddress);
}


function deriveSolanaWallet(seed) {
  const solanaPath = "m/44'/501'/0'/0'";
  const solanaDerivedSeed = derivePath(solanaPath, seed).key;
  const solanaKeypair = Keypair.fromSeed(solanaDerivedSeed);
  const solanaAddress = solanaKeypair.publicKey.toBase58();

 
  const solanaPrivateKey = Buffer.from(solanaKeypair.secretKey).toString('hex');

  console.log("\n--- Solana ---");
  console.log("Derivation Path:   ", solanaPath);
  console.log("Private Key (Hex): ", solanaPrivateKey); 
  console.log("Public Key/Address:", solanaAddress);
}


async function main() {
  const mnemonic = bip39.generateMnemonic();
  console.log("========================================================================");
  console.log("✅ Generated 12-Word Mnemonic Phrase:");
  console.log(mnemonic);
  console.log("========================================================================");

  const seed = await bip39.mnemonicToSeed(mnemonic);

  deriveEthereumWallet(seed);
  deriveBitcoinWallet(seed);
  deriveSolanaWallet(seed);

  console.log("\n========================================================================");
  console.log("✅ Wallet generation complete.");
}

main().catch(console.error);