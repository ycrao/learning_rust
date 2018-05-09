# Learning Rust

>   [Rust](https://www.rust-lang.org/zh-CN/) 学习笔记及示例代码，主要是对 [《Rust 程序设计语言》](https://kaisery.gitbooks.io/rust-book-chinese/content/) *（第一版） 简体中文版* 教程进行学习笔记摘要。笔者所运行的环境为 `macOS` ，文档及示例代码如果不做特殊说明均在此环境描述或运行。

>   其它优秀资源：[A Gentle Introduction To Rust](https://stevedonovan.github.io/rust-gentle-intro/readme.html) (英文原版)。

## 1. 介绍

>   Rust 是一个系统编程语言，它注重于三个方面：安全，速度和并发性。为了实现这些目标，它没有采用垃圾回收机制（GC）。这让它在其它语言并不擅长的场景中大展身手：嵌入到其它语言中、在特定的时间和空间要求下编程、编写例如设备驱动和操作系统这样的底层代码。它通过一系列不产生运行时开销的编译时安全检查来提升目前语言所关注的领域，同时消除一切数据竞争。Rust 还致力于实现“零开销抽象”，虽然有些抽象看起来更像一个高级语言的特性。即便如此，你仍然可以使用 Rust 来做一些底层的精准控制。

## 2. 准备

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

## 3. 教程：猜猜看

>   这里不再赘述，参考原始文档。代码见 [guessing_game](guessing_game/src/main.rs) 。

## 4. 语法和语义

### 4.1 变量绑定

```rust
let x = 5;
// 模式
let (x, y) = (1, 2);
// 类型注解(type annotations)
let x: i32 = 5;
// 可变性
let mut x = 5;
// 初始化绑定
let x: i32;
```

>   注意：Rust 是不会让我们使用一个没有经过初始化的值的。

#### 作用域和隐藏(scope and shadowing)

```rust
fn main() {
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);  // This won't work.
}
```

>   隐藏和可变绑定可能表现为同一枚硬币的两面，他们是两个不同的概念，不能互换使用。举个例子，隐藏允许我们将一个名字重绑定为不同的类型。它也可以改变一个绑定的可变性。注意隐藏并不改变和销毁被绑定的值，这个值会在离开作用域之前继续存在，即便无法通过任何手段访问到它。

```rust
fn main() {
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "8".
    let x =  42;
    println!("{}", x); // Prints "42".
}
```

### 4.2 函数

```rust
# main入口函数
fn main() {
}
# 自定义函数
fn foo() {
}
# 携带参数
fn print_number(x: i32) {
    println!("x is: {}", x);
}
# 箭头 限定返回类型
fn add_one(x: i32) -> i32 {
    x + 1
}
```

#### 发散函数

Rust有些特殊的语法叫“发散函数”，这些函数并不返回：

```rust
fn diverges() -> ! {
    panic!("This function never returns!");
}
```

`panic!` 是一个宏，类似我们已经见过的 `println!()` 。与 `println!()` 不同的是，`panic!()` 导致当前的执行线程崩溃并返回指定的信息。因为这个函数会崩溃，所以它不会返回，所以它拥有一个类型!，它代表“发散”。


如果你想要更多信息，你可以设定 `RUST_BACKTRACE` 环境变量来获取 `backtrace` ：

```shell
RUST_BACKTRACE=0 ./diverges
RUST_BACKTRACE=1 cargo run
```

发散函数可以被用作任何类型。

#### 函数指针


我们也可以创建指向函数的变量绑定：`f` 是一个指向一个获取 `i32` 作为参数并返回 `i32` 的函数的变量绑定。

```rust
fn plus_one(i: i32) -> i32 {
    i + 1
}

// without type inference
let f: fn(i32) -> i32 = plus_one;

// with type inference
let f = plus_one;
let six = f(5);
```
