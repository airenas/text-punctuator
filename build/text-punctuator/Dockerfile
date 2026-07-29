#########################################################################################
## docker will invoke this file from ../.. dir in order to access code
#########################################################################################
FROM rust:1.97-trixie AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends  binutils

ARG BUILD_VERSION=0.1

ENV RUSTFLAGS="-C target-cpu=haswell -C opt-level=3 -C codegen-units=1"
ENV CARGO_PROFILE_RELEASE_DEBUG=false

WORKDIR /src/

COPY src /src/src
COPY Cargo.toml /src/Cargo.toml
COPY Cargo.lock /src/Cargo.lock
COPY build.rs /src/build.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
      CARGO_APP_VERSION=$BUILD_VERSION cargo build --release
RUN strip /src/target/release/text-punctuator-ws      
#########################################################################################
FROM debian:trixie AS sherpa_builder

RUN apt-get update && apt-get install -y \
    git \
    cmake \
    ninja-build \
    build-essential \
    python3 \
    wget \
    unzip \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src

RUN git clone --branch v1.13.4-punct --depth 1 \
    https://github.com/airenas/sherpa-onnx.git

WORKDIR /src/sherpa-onnx

RUN cmake -B build \
    -G Ninja \
    -DCMAKE_BUILD_TYPE=Release \
    -DBUILD_SHARED_LIBS=ON \
    -DSHERPA_ONNX_ENABLE_C_API=ON

RUN cmake --build build --target sherpa-onnx-c-api
#########################################################################################
FROM debian:trixie-slim AS runner

RUN apt-get update && \
    apt-get install -y --no-install-recommends  binutils \
    ca-certificates \
    passwd \
    openssl && \
    rm -rf /var/lib/apt/lists/*

ENV UID=999
ENV LD_LIBRARY_PATH=/libs/

ARG BUILD_VERSION=0.1

WORKDIR /app
EXPOSE 8000

RUN groupadd --gid $UID app && useradd -r -M -u $UID -g app app
COPY LICENSE /licenses/LICENSE
RUN mkdir -p /app && chown -R app:app /app   

COPY --from=builder /src/target/release/text-punctuator-ws /app/

COPY --from=sherpa_builder /src/sherpa-onnx/build/lib/libsherpa-onnx-c-api.so /libs/
COPY --from=sherpa_builder /src/sherpa-onnx/build/_deps/onnxruntime-src/lib/libonnxruntime.so /libs/

RUN chown app:app /app/* /app
 
USER app

ENTRYPOINT ["./text-punctuator-ws"]
