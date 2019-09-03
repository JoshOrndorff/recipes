# srml-treasury

* keeps the proportion of tokens staked constant
* why? security in PoS systems
* see `OnDilution` trait as well as the facts regarding dilution (where can I show in other files where dilution is taking place)

* the `Imbalances` type provides safe-esque accounting

* discuss how proposals are collateralized with `reserve` and `unreserve` `=>` sybil mechanism basically

* scheduling spending funds with `on_finalize` `=>` this could supplement `blockchain event loop`...keep the recipe in `balances` for schedulinhg execution as a minimal example (could provide commentary with it in the repo instead of in the book...?)...use it to introduce treasury logic for scheduling execution quickly `=>` then reference the other recipe

## open questions

If we read [here](), we realize that there are a few things that are not yet done for this treasury module. How do assess proposals for spending? 

* set spending targets
* set inflow targets based on total stake amount (targets basically)
* use futarchy for governance of changes and price proposals in proportion to how far they veer from norms `=>` allow crowdfunding proposals with proportional governance?
