default: build

build:
	cargo build --verbose --release --manifest-path Cargo.toml

run: build
	cargo run --manifest-path Cargo.toml

# TODO: Invalid
tests: backend/target/release/libsnake_backend.a
	checkmk backend/test/test.ts > backend/test/test.c
	gcc -o run_tests backend/test/test.c backend/target/release/libsnake_backend.a -lncurses -lcheck -lsubunit -lrt -lpthread -lm -ldl
	./run_tests
	cargo test --manifest-path backend/Cargo.toml

clean:
	cargo clean --manifest-path Cargo.toml
	rm -f snake run_tests
