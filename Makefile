IMAGE = ./target/x86_64-test_os/debug/test_os

build:
	cargo build

run: build
	cargo run

dbg: build
	cargo run -- -s -S

clean:
	cargo clean