.PHONY: build test fmt clippy clean install completions

build:
	cargo build --release

test:
	cargo test --all-features

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-features -- -D warnings

check: fmt clippy test

clean:
	cargo clean

install: build
	install -Dm755 target/release/rustwhy $(DESTDIR)/usr/local/bin/rustwhy

completions:
	mkdir -p assets/completions
	rustwhy completions bash  > assets/completions/rustwhy.bash
	rustwhy completions zsh   > assets/completions/rustwhy.zsh
	rustwhy completions fish  > assets/completions/rustwhy.fish
	rustwhy completions powershell > assets/completions/_rustwhy.ps1
