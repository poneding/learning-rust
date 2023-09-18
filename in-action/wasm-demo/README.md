# wasm-demo

## 编码实现

1、 Rust 编译器添加 wasm32-wasi 目标。

```bash
rustup target add wasm32-wasi
```

2、Cargo.toml

编辑 Cargo.toml 添加如下依赖。这里我们使用 wrap_wasi 来开发一个简单的 HTTP Server， warp_wasi 构建在 Warp 框架之上，Warp 是一个轻量级的 Web 服务器框架，用于构建高性能的异步 Web 应用程序。

原生的 Warp 框架编写的代码无法直接编译成 Wasm 模块。因此我们可以使用 warp_wasi，通过它我们可以在 Rust 中利用 Wasi 接口来开发 Web 应用程序。

```toml
[dependencies]
tokio_wasi = { version = "1", features = ["rt", "macros", "net", "time", "io-util"]}
warp_wasi = "0.3"
```

3、src/main.rs

```bash
use warp::Filter;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hello = warp::get()
	.and(warp::path::end())
	.map(|| "Hello World!");

    warp::serve(hello).run(([0,0,0,0], 8080)).await
}
```

## 本地编译

```bash
cargo build --target wasm32-wasi --release
```

## 本地运行

```bash
wasmedge target/wasm32-wasi/release/wasm-demo.wasm

curl localhost:8080
```

## 编译镜像

```bash
./build.sh
```

## 运行镜像

> 注意：需要容器运行时支持，目前新版的 Docker Desktop 已经支持开启 Wasm 支持，可以参考 [Docker + WASM](https://wasmedge.org/docs/zh/start/build-and-run/docker_wasm/)。

```bash
docker run -dp 8080:8080 --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm poneding/wasm-demo-rust

curl localhost:8080
```