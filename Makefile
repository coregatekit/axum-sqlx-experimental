dev:
	cargo watch -c -w src -x run

run:
	cargo run

test:
	cargo test

build:
	cargo build

clean:
	cargo clean