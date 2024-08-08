# diesel

官方文档：[diesel-cli](https://diesel.rs/guides/configuring-diesel-cli)

使用 diesel 迁移数据库。

## 安装 diesel_cli

```bash
cargo install diesel_cli --no-default-features --features sqlite

# 自动补全代码
source <(diesel completions zsh)
```

## 初始化 diesel

```bash
touch diesel.toml
diesel setup --database-url=./test.db
```

## 创建 migration

```bash
diesel migration generate create_users
diesel migration generate create_orders
```

## 执行 migration

```bash
diesel database setup
diesel migration run
```
