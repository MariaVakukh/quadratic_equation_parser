run: cargo run -- parse equation.txt
input: cargo run -- input
test: cargo test
fmt: cargo fmt	
clippy: cargo clippy --all-targets --all-features -- -D warnings
precommit: fmt clippy test
build: cargo build --release
all: fmt clippy test run
help:
	@echo "Makefile commands:"
	@echo "  make run 				     - run the program with equation.txt file"
	@echo "  make interactive            - interactive mode"
	@echo "  make test                   - run all tests"
	@echo "  make fmt                    - format the code"
	@echo "  make clippy                 - run clippy for code linting"
	@echo "  make precommit              - format + clippy + tests"
	@echo "  make build                  - build release version"
 