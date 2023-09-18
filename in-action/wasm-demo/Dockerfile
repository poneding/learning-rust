FROM --platform=$BUILDPLATFORM rust:1.72 AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasi
EOT

FROM buildbase AS build
COPY Cargo.toml .
COPY src ./src
# Build the WASM binary
RUN cargo build --target wasm32-wasi --release

FROM scratch
WORKDIR /run
COPY --link --from=build /src/target/wasm32-wasi/release/wasm-demo.wasm ./wasm-demo.wasm
ENTRYPOINT [ "./wasm-demo.wasm" ]
