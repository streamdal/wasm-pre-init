.PHONY: build
build:
	mkdir -p build && \
	rm -f build/wasm_sleep_regular.wasm && \
	cargo build --target=wasm32-wasi --release && \
	mv target/wasm32-wasi/release/wasm_sleep_regular.wasm build/wasm_sleep_regular.wasm
