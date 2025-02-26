all:
	cargo build
	cp target/debug/P2 ./

check:
	cargo test

.PHONY: clean
clean:
	cargo clean

.PHONY: install-deps
install-deps:
	sudo apt-get update -y
	sudo apt-get install -y cargo