[Learning Rust](../../README.md) / [rust-lang-book](../zz_gneratered_mdi.md) / [rlb00_hello-world](zz_gneratered_mdi.md) / Rust Hello World

# Rust Hello World

创建项目以及 main.rs 文件，内容如下：

```bash
mkdir rlb00_hello-world
cd rlb00_hello-world
cat > main.rs << EOF
fn main() {
    println!("Hello, world!");
}
EOF
```

运行：

```bash
rustc main.rs
./main
```

将得到输出：

```bash
Hello, world!
```
