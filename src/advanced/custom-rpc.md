# Custom RPCs
TODO link to code

Remote Proceedure Calls, or RPCs, are a way for an external program to communicate with a Substrate node. Substrate comes with several RPCs by default. They are used for checking storage values, submitting transactions, and querying the current consensus authorities. In many cases it is useful to add custom RPCs to your node. In this recipe, we will add two custom RPCs to our node. One of which calls into a [custom runtime API](./runtime-api.md).
