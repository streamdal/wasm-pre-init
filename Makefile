.PHONY: build
build:
	rm -f wasm*/build/*.wasm && \
	cd wasm-regex-init && make build && cd .. && \
	cd wasm-regex-regular && make build && cd .. && \
	cd wasm-sleep-regular && make build && cd .. && \
	cd wasm-sleep-init && make build && cd .. && \
	rm -rf build && \
	mkdir -p build && \
	cp wasm*/build/*.wasm build/
	@echo "Builds created in ./build/*.wasm"
	for wasm in build/*init*.wasm; do \
		wasm_file=$$(basename $$wasm | cut -d . -f1) ; \
		wizer --wasm-bulk-memory true --allow-wasi $$wasm -o build/$$wasm_file.wizer.wasm ; \
	done
	@echo "All Wasm artifacts created; run 'make run' to test";

.PHONY: run
run:
	@echo "Testing wasm-regex-regular..."
	time wazero run -env NUM=123 build/wasm_regex_regular.wasm
	@echo "Testing wasm-regex-init..."
	time wazero run -env NUM=123 build/wasm_regex_init.wizer.wasm
	@echo "Testing wasm-sleep-regular..."
	time wazero run build/wasm_sleep_regular.wasm
	@echo "Testing wasm-sleep-init..."
	time wazero run build/wasm_sleep_init.wizer.wasm
