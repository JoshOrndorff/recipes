FROM paritytech/substrate-playground-template-base:sha-0793587

ADD . .

RUN cargo build --release -p basic-pow
RUN cargo build --release -p hybrid-consensus
RUN cargo build --release -p kitchen-node
RUN cargo build --release -p rpc-node
