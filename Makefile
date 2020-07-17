default: build

target/release/librust_snake.a:
	cargo build --verbose --release

build: target/release/librust_snake.a
	gcc -o snake src/main.c src/backend.c src/frontend.c target/release/librust_snake.a -lncurses

run: build
	./snake

tests:
	checkmk test/test.ts > test/test.c
	gcc -o run_tests src/backend.c src/backend.h test/test.c -lncurses -lcheck
	./run_tests

clean:
	cargo clean
	rm snake run_tests
