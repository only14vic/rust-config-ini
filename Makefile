-include .env
export

SHELL = sh
.DEFAULT_GOAL = help

ifndef VERBOSE
.SILENT:
endif

make = make --no-print-directory
CARGO_ARGS = --release
RUSTFLAGS = -Ctarget-cpu=native \
			-Clinker-plugin-lto \
			-Clink-arg=-fuse-ld=lld \
			-Clink-arg=-lc

ifeq ($(static),)
	#CARGO_BUILD_TARGET = x86_64-unknown-linux-gnu
	RUSTFLAGS += # -Cprefer-dynamic
else
	CARGO_BUILD_TARGET = x86_64-unknown-linux-musl
	RUSTFLAGS += -Ctarget-feature=+crt-static
endif

ifneq ($(no_std),)
	RUSTFLAGS += -Cpanic=abort
	CARGO_ARGS += --no-default-features
endif

CARGO_ARGS += $(args)

all:
	$(make) examples
	$(make) examples static=1
	$(make) examples-no-std
	$(make) examples-no-std static=1
	$(make) check info

.PHONY: examples
examples:
	@echo TARGET: $(CARGO_BUILD_TARGET)
	@echo CARGO_ARGS: $(CARGO_ARGS)
	@echo RUSTFLAGS: $(RUSTFLAGS)
	cargo run  --example example1 $(CARGO_ARGS)

.PHONY: examples-no-std
examples-no-std:
	$(make) examples no_std=1

.PHONY: check
check:
	find target -type f -executable -path "*/release/examples/example1" \( \
			-exec echo -e "-----------------------\n" \; \
			-exec ls -sh {} \; -exec ldd {} \; \
			-exec valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all --error-exitcode=1 {} \; \
		-o -quit \)

.PHONY: perf
perf:
	perf record -F99 --call-graph dwarf \
		"$(shell find target -type f -executable -path "*/release/examples/example1" | head -n1)"
	perf report


info:
	find target -type f -executable -path "**/release/examples/example1" \
		-exec ls -sh {} \; -exec ldd {} \; -exec echo -e "------------------------" \;
