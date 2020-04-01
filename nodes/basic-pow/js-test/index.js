// This whole thing is about diagnosing why transactions submitted to the basic-pow
// node fail.

//Original Problem
// After entering the no-indices-pallet types the error is
// Could not convert parameter `tx` between node and runtime: Error decoding field CheckEra.0
// Removing the check
// This was solved by updating the runtime to use MultiSignature

//Current Problem
// Transactions are always `Invalid`

const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const provider = new WsProvider('ws://localhost:9944');
const types = {
    "Address": "AccountId",
    "LookupSource": "AccountId"
};


const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';


async function main() {
    const api = await ApiPromise.create({ provider, types });

    const keyring = new Keyring({ type: 'sr25519' });
    const alicePair = keyring.addFromUri('//Alice');

    // Check the genesis hash. Sanity check this against the node output.
    console.log(`Genesis hash according to chain" ${api.genesisHash.toHex()}`);

    // A Transaction to send 12345 tokens to Bob
    // This transaction is not yet "from" anyone.
    // That happens when signing the tx.
    const transferTx = api.tx.balances.transfer(BOB, 12345);
    console.log(`Unsigned Transfer:\n ${JSON.stringify(transferTx.toHuman())}`);

    // Sign and send the transaction in one step.
    const hash = await transferTx.signAndSend(alicePair, ({ events = [], status }) => {
      console.log('Transaction status:', status.type);

      if (status.isInBlock) {
        console.log('Included at block hash', status.asInBlock.toHex());
        console.log('Events:');

        events.forEach(({ phase, event: { data, method, section } }) => {
          console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
        });
      } else if (status.isFinalized) {
        console.log('Finalized block hash', status.asFinalized.toHex());

        process.exit(0);
      }
    });
}

main().catch(console.error);
