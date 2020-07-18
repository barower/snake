default: build

target/release/librust_snake.a:
	cargo build --verbose --release

build: target/release/librust_snake.a
	gcc -o snake src/main.c src/frontend.c target/release/librust_snake.a -lncurses -ldl -lpthread -lm

run: build
	./snake

tests: target/release/librust_snake.a
	checkmk test/test.ts > test/test.c
	gcc -o run_tests test/test.c target/release/librust_snake.a -lncurses -lcheck -lsubunit -lrt -lpthread -lm -ldl
	./run_tests
	cargo test

clean:
	cargo clean
	rm -f snake run_tests
