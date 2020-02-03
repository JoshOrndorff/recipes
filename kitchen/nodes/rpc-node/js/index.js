// A demonstration of interacting with custom RPCs using Polkadot js API

const { ApiPromise, WsProvider } = require('@polkadot/api');

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const types = {};
const rpc = {
  silly: [
    {
      description: "Always returns 7",
      name: "seven",
      params: [],
      type: "u32",
    },
    {
      description: "Doubles the parameter",
      name: "double",
      params: ["u32"],
      type: "u32",
    }
  ],
  sumStorage: [
    {
      description: "Gets the sum of the two storage values in sum-storage pallet via a runtime api.",
      name: "getSum",
      params: [],
      type: "u32",
    }
  ]
}

async function main() {
  // Construct the actual api
  const api = await ApiPromise.create({
    provider: wsProvider,
    types,
    rpc,
  });

  // Query the custom SillyRpc
  let silly7 = await api.rpc.silly.seven();
  let silly14 = await api.rpc.silly.double(7);
  console.log(`The value from the silly_seven is ${silly7}\n`);
  console.log(`The double of 7 according to silly_double is ${silly14}\n`);

  // Query raw storage values, the oldschool way
  const v1 = ( await api.query.sumStorage.thing1() ).unwrap().toNumber();
  const v2 = ( await api.query.sumStorage.thing2() ).unwrap().toNumber();
  console.log(`The individual storage values are ${v1}, and ${v2}.`);
  console.log(`The sum calculated in javascript is ${v1 + v2}\n`);

  // Query the custom RPC that uses the runtimeAPI
  let directSum = ( await api.rpc.sumStorage.getSum() ).toNumber();
  console.log(`The sum queried directly from the RPC is ${directSum}`);
}

main().catch(console.error).finally(() => process.exit());
