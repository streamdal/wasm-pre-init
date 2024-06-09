.PHONY: build
build:
	mkdir -p build && \
	rm -f build/wasm_sleep_init.wasm && \
	cargo build --target=wasm32-wasi --release && \
	mv target/wasm32-wasi/release/wasm_sleep_init.wasm build/wasm_sleep_init.wasm
