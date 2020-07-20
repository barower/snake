default: build

build:
	cargo build --verbose --release --manifest-path Cargo.toml

run: build
	cargo run --release --manifest-path Cargo.toml

tests:
	cargo test --release --manifest-path Cargo.toml

clean:
	cargo clean --manifest-path Cargo.toml
