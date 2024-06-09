.PHONY: build
build:
	mkdir -p build && \
	rm -f build/wasm_regex_regular.wasm && \
	cargo build --target=wasm32-wasi --release && \
	mv target/wasm32-wasi/release/wasm_regex_regular.wasm build/wasm_regex_regular.wasm
