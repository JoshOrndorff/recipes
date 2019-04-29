# Proposal Patterns

* lock-in vs instant withdrawal
* ways of increasing the pace
* more research on speed

* start with security assumptions
* keep it simple stupid
    * start with a simple design that works and then iterate with increased complexity
    * example: has anyone looked at the dashboard of widgets on facebook; this UI wouldn't work for onboarding, but iterative AB testing has determined that users appreciate this level 

### Reputational, Non-Monetary Incentives

* not everything needs to be incentivizes
* participation in a DAO is incentivizes by the power that it endows the members who control funding decisions
    * **the development of reputation is enough incentive in many cases**

* token economics design => (1) reward with shares and align long-term incentives (2) award over time to provide liquid rewards (which also minimizes complexity involved with monitoring share inflation by limiting minting to proposals only)

### Punishment in Non-Adversarial Environment

* instead of assuming a malicious environment, we can be much more transparent 

## Design Choices

* Dilution Bound?

In more ways than one, the logic for coordinating action within a DAO mirrors Proof of Stake consensus mechanisms

* the proposal process
    * **the grace period is for instant withdrawals which are important for security (the dao -> plasma chains)**
    * open problem: implementing a fast, asynchronous proposal algorithm that does not suffer from head of line blocking but also ensures against *malicious exits* (exits by validators that voted yes on a pending proposal)
        * consider incorporating fraud proofs into this scheme? Used in plasma for sure as well as asynchronous consensus algorithms
        * futures as an optimization to prevent head-of-line blocking for the proposal process