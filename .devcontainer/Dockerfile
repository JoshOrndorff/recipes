FROM jeluard/substrate-playground-template-base@sha256:35683978140807d8e3c9eaa814e9034bd254e74224073984db2159302d44946b

ADD . .

RUN cargo build --release -p basic-pow
RUN cargo build --release -p babe-grandpa-node
RUN cargo build --release -p hybrid-consensus
RUN cargo build --release -p kitchen-node
RUN cargo build --release -p manual-seal
RUN cargo build --release -p rpc-node