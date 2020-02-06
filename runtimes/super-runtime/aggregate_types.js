// Reads in the type definitions from all pallets in the runtime, naively aggregates them,
// and writes them to disk.

const fs = require('fs');

// A list of all the installed recipe pallets' folder names.
// Does not include system pallets because Apps already supports them.
// Redundant with construct_runtime!
const pallets = [
  "adding-machine",
  "basic-token",
  "check-membership",
  "constant-config",
  "default-instance",
  "double-map",
  "execution-schedule",
  "generic-event",
  "last-caller",
  "linked-map",
  "simple-event",
  "simple-map",
  "charity",
  "single-value",
  "storage-cache",
  "struct-storage",
  "vec-set",
]

// Loop through all pallets aggregating types
let finalTypes = {}
let palletTypes;
for (let dirname of pallets) {
  let path = `../../pallets/${dirname}/types.json`;
  palletTypes = JSON.parse(fs.readFileSync(path, 'utf8'));
  finalTypes = {...finalTypes, ...palletTypes};
}

// Write output to disk
fs.writeFileSync("types.json", JSON.stringify(finalTypes, null, 2), 'utf8');
