-include .env
export

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory

RUSTFLAGS = -Clinker-plugin-lto -Clink-arg=-fuse-ld=lld -Clink-args=-lclang

.PHONY: examples
examples:
	cargo run --example example1 --release

.PHONY: examples-no-std
examples-no-std:
	RUSTFLAGS="-Cpanic=abort $(RUSTFLAGS)" \
			cargo run --example example1 --no-default-features --release

.PHONY: check
check:
	valgrind --tool=memcheck -q target/vim-local/release/examples/example1
