# Copyright (c) 2022 Espresso Systems (espressosys.com)
# This file is part of the AtomicStore library.
# 
# This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
# You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.


.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	cargo clean
	cargo clean --release
	rm -rf ./target grcov-*.profraw

.PHONY: check
check:
	cargo check


.PHONY: test
test:
	cargo test --release

.PHONY: doc
doc:
	cargo doc --no-deps

.PHONY: coverage
coverage: export RUSTUP_TOOLCHAIN=nightly
coverage: export LLVM_PROFILE_FILE=grcov-%p-%m.profraw
coverage: export RUSTFLAGS=-Zinstrument-coverage
coverage:
	rm -rf grcov-*.profraw default.profraw
	cargo test
	grcov .                                \
	    --binary-path ./target/debug/      \
	    -s .                               \
	    -t html                            \
	    --branch                           \
	    --ignore-not-existing              \
	    -o ./coverage/ &&                  \
	echo "See file://${PWD}/coverage/index.html for coverage."

.PHONY: setup
setup:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
