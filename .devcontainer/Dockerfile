FROM paritytech/substrate-playground-template-base:sha-e8f33a4

# Here the whole repo is already accessible at . (thanks to the inherited image)

RUN cargo build --release -p basic-pow
RUN cargo build --release -p hybrid-consensus
RUN cargo build --release -p kitchen-node
RUN cargo build --release -p rpc-node
