default: build

backend/target/release/librust_snake.a:
	cargo build --verbose --release --manifest-path backend/Cargo.toml

build: backend/target/release/librust_snake.a
	gcc -o snake src/main.c src/frontend.c backend/target/release/librust_snake.a -lncurses -ldl -lpthread -lm

run: build
	./snake

tests: backend/target/release/librust_snake.a
	checkmk test/test.ts > test/test.c
	gcc -o run_tests test/test.c backend/target/release/librust_snake.a -lncurses -lcheck -lsubunit -lrt -lpthread -lm -ldl
	./run_tests
	cargo test --manifest-path backend/Cargo.toml

clean:
	cargo clean --manifest-path backend/Cargo.toml
	rm -f snake run_tests
