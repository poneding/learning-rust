# chat-rs

学习地址：<https://blog.logrocket.com/real-time-chat-app-rust-react/>

## 程序架构

![20240806084935](https://images.poneding.com/2024/06/20240806084935.png)

## 搭建项目

```bash
cargo new rust-react-chat

cargo add actix actix-files actix-cors, actix-web actix-web-actors rand serde serde_json dotenv uuid chrono

cargo install diesel_cli --no-default-features --features sqlite
```

> 如果遇到 error: could not compile `diesel_cli` (bin "diesel") due to 1 previous error，执行 `sudo apt install libsqlite3-dev` 安装 sqlite3 的头文件。

手动添加 `diesel` 和 `uuid` 依赖：

```toml
[dependencies]
...
diesel = { version = "2.2.2", features = ["sqlite", "r2d2"] }
uuid = { version = "1.10.0", features = [
    "v4",
    "fast-rng",
    "macro-diagnostics",
] }
...
```

## 数据库设计

[202408060847434](https://images.poneding.com/2024/06/202408060847434.png)

数据库初始化：

```bash
mkdir migrations
diesel migration generate create_users
diesel migration generate create_rooms
diesel migration generate create_conversations
diesel migration generate dummy_data
diesel database setup --database-url chat.db
diesel migration run --database-url chat.db

diesel print-schema > src/schema.rs
```

## 前端

```bash
yarn create next-app --js ui
cd ui
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

运行：

```bash
yarn install
yarn dev
```
