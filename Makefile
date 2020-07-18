default: build

backend/target/release/libsnake_backend.a:
	cargo build --verbose --release --manifest-path backend/Cargo.toml

build: backend/target/release/libsnake_backend.a
	gcc -o snake frontend/src/main.c frontend/src/frontend.c backend/target/release/libsnake_backend.a -lncurses -ldl -lpthread -lm

run: build
	./snake

tests: backend/target/release/libsnake_backend.a
	checkmk backend/test/test.ts > backend/test/test.c
	gcc -o run_tests backend/test/test.c backend/target/release/libsnake_backend.a -lncurses -lcheck -lsubunit -lrt -lpthread -lm -ldl
	./run_tests
	cargo test --manifest-path backend/Cargo.toml

clean:
	cargo clean --manifest-path backend/Cargo.toml
	cargo clean --manifest-path frontend/Cargo.toml
	rm -f snake run_tests
