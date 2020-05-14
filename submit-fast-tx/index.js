// A demonstration of interacting with custom RPCs using Polkadot js API

const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { readFileSync } = require('fs');

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const types = JSON.parse(readFileSync('../runtimes/super-runtime/types.json', 'utf8'));

// Alice and Bob addresses
const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

// The number of transactions to send and the delay between sending them.
const num_txs = 3;
const delay = 1000; //milliseconds

async function main() {
  // Construct the actual api
  const api = await ApiPromise.create({
    provider: wsProvider,
    types,
  });

  // Constuct a keying with Alice's keys after the API (crypto has an async init)
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

	// Read the initial nonce from chain state
	let initial_nonce = (await api.query.system.account(ALICE)).nonce;
	console.log(initial_nonce);

  // Initialize nonce to 0. We will manage nonce manually to ensure transactions are all valid
  // and reduce roundtrips to the node.
  for (let nonce = initial_nonce; nonce < initial_nonce + num_txs; nonce++) {
		console.log("Kicking off transaction with nonce " + nonce);

    // Transfer 1 token to bob
    api.tx.balances.transfer(BOB, 1000)
      .signAndSend(alice, {nonce}, (response) => {
        console.log('Transaction status:', response.status.type);
	  });

		// Wait a short while before sending again.
		await new Promise(resolve => setTimeout(resolve, delay));
	}
}

main().catch(console.error).finally(() => process.exit());
