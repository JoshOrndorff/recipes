// Reads in the type definitions from all modules in the runtime, naively aggregates them, and writes them to disk.
const fs = require('fs');

// A list of all the installed recipe modules' folder names. Does not include system modules
// Redundant with construct_runtime!
const modules = [
  "simple-event",
  "generic-event",
  "adding-machine",
  "single-value",
  "vec-set",
  "storage-cache",
  "simple-map",
  "double-map",
  "linked-map",
  "struct-storage",
  "module-constant-config",
  "basic-token",
  "check-membership",
  "execution-schedule",
  "smpl-treasury",
  "last-caller",
  "default-instance",
]

// Loop through install modules aggregating types
let finalTypes = {}
let moduleTypes;
for (let dirname of modules) {
  let path = `../../modules/${dirname}/types.json`;
  moduleTypes = JSON.parse(fs.readFileSync(path, 'utf8'));
  finalTypes = {...finalTypes, ...moduleTypes};
}

// Write output to disk
fs.writeFileSync("types.json", JSON.stringify(finalTypes), 'utf8');
