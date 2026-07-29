-include Makefile.options
log?=INFO
port?=8000
###############################################################################
run:
	RUST_LOG=$(log) cargo run --bin text-punctuator-ws -- --onnx-model ${ONNX_MODEL} --bpe-vocab ${BPE_VOCAB} --port ${port}
.PHONY: run
###############################################################################
run/build: build/local
	RUST_LOG=$(log) target/release/text-punctuator-ws --onnx-model ${ONNX_MODEL} --bpe-vocab ${BPE_VOCAB} --port ${port}
.PHONY: run/build
run/build/debug: build/debug
	RUST_LOG=$(log) target/debug/text-punctuator-ws --onnx-model ${ONNX_MODEL} --bpe-vocab ${BPE_VOCAB} --port ${port}
.PHONY: run/build/debug
###############################################################################
build/local: 
	cargo build --release
.PHONY: build/local
build/debug: 
	cargo build --features profiling
.PHONY: build/debug
###############################################################################
test/unit:
	RUST_LOG=DEBUG cargo test --no-fail-fast
.PHONY: test/unit		
test/lint:
	@cargo clippy -V
	cargo clippy --all-targets --all-features -- -D warnings
.PHONY: test/lint	
###############################################################################
clean:
	rm -r -f target
.PHONY: clean
