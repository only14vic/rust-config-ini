-include .env
export

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory
RUSTFLAGS = -Clink-arg=-fuse-ld=lld -Clink-args=-lc

ifeq ($(static),)
	#CARGO_BUILD_TARGET = x86_64-unknown-linux-gnu
	RUSTFLAGS += -Clinker-plugin-lto
else
	CARGO_BUILD_TARGET = x86_64-unknown-linux-musl
	RUSTFLAGS += -Ctarget-feature=+crt-static
endif

ARGS = --release

ifneq ($(no_std),)
	RUSTFLAGS += -Cpanic=abort
	ARGS += --no-default-features
endif

ARGS += $(args)

all:
	$(make) examples
	$(make) examples static=1
	$(make) examples-no-std
	$(make) examples-no-std static=1
	$(make) check

.PHONY: examples
examples:
	@echo ARGS: $(ARGS) RUSTFLAGS: $(RUSTFLAGS)
	cargo run  --example example1 $(ARGS)

.PHONY: examples-no-std
examples-no-std:
	$(make) examples no_std=1

.PHONY: check
check:
	find target -type f -executable -path "**/release/*/example1" -print \
		-exec valgrind --tool=memcheck --leak-check=full --error-exitcode=1 {} \;

info:
	find target -type f -executable -path "**/release/*/example1" \
		-exec ls -sh {} \; -exec ldd {} \; -exec echo \;
