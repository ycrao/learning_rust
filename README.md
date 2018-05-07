# learning rust

>	[Rust](https://www.rust-lang.org/zh-CN/) 学习笔记及示例代码，主要是对 [《Rust 程序设计语言》](https://kaisery.gitbooks.io/rust-book-chinese/content/) *（第一版） 简体中文版* 教程进行学习笔记摘要。笔者所运行的环境为 `macOS` ，文档及示例代码如果不做特殊说明均在此环境描述或运行。

>	其它优秀资源：[A Gentle Introduction To Rust](https://stevedonovan.github.io/rust-gentle-intro/readme.html) (英文原版)。

## 简介

>	Rust 是一个系统编程语言，它注重于三个方面：安全，速度和并发性。为了实现这些目标，它没有采用垃圾回收机制（GC）。这让它在其它语言并不擅长的场景中大展身手：嵌入到其它语言中、在特定的时间和空间要求下编程、编写例如设备驱动和操作系统这样的底层代码。它通过一系列不产生运行时开销的编译时安全检查来提升目前语言所关注的领域，同时消除一切数据竞争。Rust 还致力于实现“零开销抽象”，虽然有些抽象看起来更像一个高级语言的特性。即便如此，你仍然可以使用 Rust 来做一些底层的精准控制。

## 准备

### 安装

安装 `Rust`：

```shell
curl https://sh.rustup.rs -sSf | sh
```

### 卸载

卸载 `Rust` ：

```shell
rustup self uninstall
```

### 终端命令

查看编译器版本：

```shell
rustc --version
```

### `Hello World` 示例

见 `demo/hello_world/main.rs` [📎](demo/hello_world/main.rs) 。

```shell
# 编译
rustc main.rs
# 运行
./main
Hello, world!
```

### 转成 `Cargo` 示例

查看 `cargo` 版本：

```shell
cargo --version
```

转换成 `cargo` 版本见 [hello_world](hello_world/) 。

```shell
# 构建
cargo build
# 运行
./target/debug/hello_world
# 或者运行
cargo run
# 发布构建
cargo build --release
# 基于 `Cargo` 开始一个新项目
cargo new hello_world --bin
```

