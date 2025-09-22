const crypto = require('crypto');

function sha256(data) {
  return crypto.createHash('sha256').update(data).digest('hex');
}

function mineBlock(data, difficultyPrefix = '0000') {
  let nonce = 0;
  let hash = '';

  console.log(`Mining started... Looking for hash starting with '${difficultyPrefix}'`);

  while (true) {
    const input = data + nonce;
    hash = sha256(input);

    if (hash.startsWith(difficultyPrefix)) {
      console.log(`✅ Hash found!`);
      console.log(`Data   : "${data}"`);
      console.log(`Nonce  : ${nonce}`);
      console.log(`Hash   : ${hash}`);
      break;
    }

    if (nonce % 100000 === 0) {
      console.log(`⛏️ Trying nonce ${nonce}... Current hash: ${hash}`);
    }

    nonce++;
  }
}

// Run the miner
mineBlock("Rohit sends 10 BTC to Ram");
