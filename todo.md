# VERSION 2 CHECKLIST

1. module template-ify everything (and update the `Summary.MD` file accordingly)
* add a a link from each recipe to the relevant code in the kitchen?

2. update everything else...

## kitchen
* add executables that correspond to every recipe section

* considering an executable folder that has the same structure as `src` with links back and forth...quite a lot of linking to do, but that's fine...

**GOAL**: using the module template, create executables that go with each recipe, thereby allowing people to easily test and configure each recipe from scratch

### Getting Started

* template explainer via README
* other stuff as well

### Event Recipes

### Module Menu

- [ ] connecting the dots (between `node/executor` and `node/runtime`) (**H**)

### DAO

- [ ] Minimal (**E**)
- [ ] Incentive Management (**E**)
- [ ] Dilution Safety Mechanisms (**M**)

### Safety

- [ ] `safemath` which includes `Perbill`, `Permill`, `saturating`, etc (**M**)
- [ ] Robust Path Handling (**E**)

### Protocol Engineering

- [ ] Transaction Ordering (**E**)
- [ ] Verifying Uniqueness (**E**)
- [ ] Off-Chain Verification (**H**)

### Testing

- [ ] Scaffolding (**M**)
- [ ] Unit Testing (**H**)