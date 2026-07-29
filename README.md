# text-punctuator

Restores casing and punctuation for a plain speech. Rust web service backed by sherpa onnx ...

### Build

#### Prebuild

It requires shared libs:
- libsherpa-onnx-c-api.so
- libonnxruntime.so

To build these libs see the [sherpa_builder docker](build/text-punctuator/Dockerfile).

```bash
git clone --branch v1.13.4-punct --depth 1 \
    https://github.com/airenas/sherpa-onnx.git

cd sherpa-onnx

cmake -B build \
    -G Ninja \
    -DCMAKE_BUILD_TYPE=Release \
    -DBUILD_SHARED_LIBS=ON \
    -DSHERPA_ONNX_ENABLE_C_API=ON

cmake --build build --target sherpa-onnx-c-api
## copy onnxruntime into build/lib for simplicity
cp build/_deps/onnxruntime-src/lib/libonnxruntime.so build/lib
```

#### Compile

```bash
export SHERPA_ONNX_LIB_DIR=./sherpa-onnx/build/lib
export LD_LIBRARY_PATH=./sherpa-onnx/build/lib
cargo build
```

### Running

To run ou need model and bpe vocabulary prepared and exported using this repo: https://github.com/airenas/Edge-Punct-Casing

```bash
export ONNX_MODEL=./model/model.int8.onnx
export BPE_VOCAB=./model/bpe_vocab
cargo run
```

### Example


```bash
curl -X POST http://localhost:6007/punctuate -H "content-type: application/json" -d '{"text":"p jonaitis atvyko iš kauno kad pamatytų vilnių ir priimtų sprendimą tai buvo nelengva užduotis"}'
```
```json

{"text":"P. Jonaitis atvyko iš Kauno, kad pamatytų Vilnių ir priimtų sprendimą. Tai buvo nelengva užduotis,"}⏎   
```
---
