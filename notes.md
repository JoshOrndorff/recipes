# Pallet Coupling, Runtime Upgrades, Storage Migration

This is the draft of my plan for seminar on 2 June 2020

## Start a Network

We want a simple instant-seal network with a vanilla runtime. Start by copying the kitchen node and the super runtime. Strip out all the recipes pallets from the super-runtime, and rename stuff as appropriate.

Create a chain spec and launch the network. Make sure the node running is a validator and in archive mode.

TODO: If there is time, experiment with including a raw spec with the binary.

## Upgrade 1: Installing check-membership

Change the runtime code to include the tightly coupled variant of check-membership and vec-set, bump the spec version. Recompile runtime upgrade.

## Upgrade 2: Loose Coupling

Remove the tightly coupled version of check-membership and replace it with the loosely coupled version. bump the impl version, recompile, runtime upgrade.

## Upgrade 3: Swap to map set

### Code Changes

Remove vec-set, add map-set, change check-membership config trait to point to map-set instead.

### Migration

The code changes we made previously were enough to make a runtime work. If we were starting a new chain, we could use that code as is. But we are caring for a live chain here. We already have a set of members stored in vec-set pallet storage. We need to migrate those to be stored in the storage map.
