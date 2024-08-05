# chat-rs

```bash
cargo new rust-react-chat

cargo add actix actix-files actix-cors, actix-web actix-web-actors rand serde serde_json dotenv uuid

cargo install diesel_cli --no-default-features --features sqlite
```

> 如果遇到 error: could not compile `diesel_cli` (bin "diesel") due to 1 previous error，执行 `sudo apt install libsqlite3-dev` 安装 sqlite3 的头文件。

手动添加 `diesel` 依赖：

```toml
[dependencies]
...
diesel = { version = "2.2.2", features = ["sqlite", "r2d2"] }
...
```
