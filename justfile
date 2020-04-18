alias re := remote
remote:
	cargo remote -- build --release --features ocw

alias bo := build-ocw
build-ocw:
	cd nodes/kitchen-node && cargo build --release --features ocw -p kitchen-node

alias r := run
run:
	./target/release/kitchen-node purge-chain --dev
	./target/release/kitchen-node --dev
