[Learning Rust](../../README.md) / [the-rust-programming-language](../zz_generated_mdi.md) / [ch01-02-hello-world](zz_generated_mdi.md) / Rust Hello World

# Rust Hello World

创建项目以及 main.rs 文件，内容如下：

```bash
mkdir ch01-02-hello-world
cd ch01-02-hello-world
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

删除编译生成的文件：

```bash
rm ./main
```
