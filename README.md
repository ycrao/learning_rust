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
//# main入口函数
fn main() {
}

//# 自定义函数
fn foo() {
}

//# 携带参数
fn print_number(x: i32) {
    println!("x is: {}", x);
}

//# 箭头 限定返回类型
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

### 4.3 原生类型

```rust
//# 布尔型
let x = true;
let y: bool = false;

//# char
// 不像其它语言，这意味着 Rust 的 char 并不是 1 个字节，而是 4 个。
let x = 'x';
let two_hearts = '💕';  # unicode
```

#### 数字类型

Rust有一些分类的大量数字类型：有符号和无符号，定长和变长，浮点和整型。
这些类型包含两部分：分类，和大小。例如，u16是一个拥有 16 位大小的无符号类型。更多字节让你拥有更大的数字。

```rust
let x = 42; // `x` has type `i32`.

let y = 1.0; // `y` has type `f64`.
```

这里有一个不同数字类型的列表，以及它们在标准库中的文档：

* [i8](http://doc.rust-lang.org/nightly/std/primitive.i8.html)
* [i16](http://doc.rust-lang.org/nightly/std/primitive.i16.html)
* [i32](http://doc.rust-lang.org/nightly/std/primitive.i32.html)
* [i64](http://doc.rust-lang.org/nightly/std/primitive.i64.html)
* [u8](http://doc.rust-lang.org/nightly/std/primitive.u8.html)
* [u16](http://doc.rust-lang.org/nightly/std/primitive.u16.html)
* [u32](http://doc.rust-lang.org/nightly/std/primitive.u32.html)
* [u64](http://doc.rust-lang.org/nightly/std/primitive.u64.html)
* [isize](http://doc.rust-lang.org/nightly/std/primitive.isize.html)
* [usize](http://doc.rust-lang.org/nightly/std/primitive.usize.html)
* [f32](http://doc.rust-lang.org/nightly/std/primitive.f32.html)
* [f64](http://doc.rust-lang.org/nightly/std/primitive.f64.html)

#### 数组

```rust
let a = [1, 2, 3]; // a: [i32; 3]
let mut m = [1, 2, 3]; // m: [i32; 3]
```

数组的类型是[T; N]。我们会在泛型部分的时候讨论这个T标记。N是一个编译时常量，代表数组的长度。

有一个可以将数组中每一个元素初始化为相同值的简写。在这个例子中，a的每个元素都被初始化为0：

```rust
let a = [0; 20]; // a: [i32; 20]
```

你可以用 `a.len()` 来获取数组a的元素数量：

```rust
let a = [1, 2, 3];

println!("a has {} elements", a.len());
```

你可以用下标（subscript notation）来访问特定的元素：

```rust
let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]

println!("The second name is: {}", names[1]);
```

就跟大部分编程语言一个样，下标从0开始，所以第一个元素是 `names[0]` ，第二个是 `names[1]` 。


#### 切片(Slices)

>   一个切片（slice）是一个数组的引用（或者“视图”）。它有利于安全，有效的访问数组的一部分而不用进行拷贝。比如，你可能只想要引用读入到内存的文件中的一行。原理上，片段并不是直接创建的，而是引用一个已经存在的变量。片段有预定义的长度，可以是可变也可以是不可变的。

>   在底层，slice 代表一个指向数据开始的指针和一个长度。

```rust
let a = [0, 1, 2, 3, 4];
let complete = &a[..]; // A slice containing all of the elements in `a`.
let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.
```

#### str

>   Rust 的 `str` 类型是最原始的字符串类型。作为一个不定长类型，它本身并不是非常有用，不过当它用在引用后是就有用了，例如 `&str` 。如你所见，我们到时候再讲。

#### 元组(Tuples)

```rust
let x = (1, "hello");

//# 注明数据类型
let x: (i32, &str) = (1, "hello");

//# 元组赋值
let mut x = (1, 2); // x: (i32, i32)
let y = (2, 3); // y: (i32, i32)

x = y;

# 解构let
let (x, y, z) = (1, 2, 3);

println!("x is {}", x);

//# 消除一个单元素元组和一个括号中的值的歧义
(0,); // single-element tuple
(0); // zero in parentheses

//# 通过索引语法访问一个元组的字段
let tuple = (1, 2, 3);

// 就像数组索引，它从0开始，不过也不像数组索引，它使用.，而不是[]。
let x = tuple.0;
let y = tuple.1;
let z = tuple.2;

println!("x is {}", x);
```

#### 函数

函数也是一个类型！它们看起来像这样：

```rust
fn foo(x: i32) -> i32 { x }

let x: fn(i32) -> i32 = foo;
```

在这个例子中，`x` 是一个“函数指针”，指向一个获取一个 `i32` 参数并返回一个 `i32` 值的函数。

### 4.4 注释

```rust
//# 行注释（line comments）

// Line comments are anything after ‘//’ and extend to the end of the line.

let x = 5; // This is also a line comment.

//# 文档注释（doc comments），内建 Markdown 标记支持

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
```

### 4.5 if语句

略去不提。

```rust
let x = 5;

if x == 5 {
    println!("x is five!");
}

if x == 5 {
    println!("x is five!");
} else if x == 6 {
    println!("x is six!");
} else {
    println!("x is not five or six :(");
}

let y = if x == 5 {
    10
} else {
    15
}; // y: i32

let y = if x == 5 { 10 } else { 15 }; // y: i32
```

### 4.6 循环

```rust
//# loop
loop {
    println!("Loop forever!");
}

//# while
let mut x = 5; // mut x: i32
let mut done = false; // mut done: bool

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}

//# for
for x in 0..10 {  // 范围 0 <= x <10
    println!("{}", x); // x: i32
}
for (index, value) in (5..10).enumerate() {
    println!("index = {} and value = {}", index, value);
}

let lines = "hello\nworld".lines();

for (linenumber, line) in lines.enumerate() {
    println!("{}: {}", linenumber, line);
}

//# break 提前退出循环
let mut x = 5;

loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
}

//# continue 不退出循环，直接进行下一次迭代
for x in 0..10 {
    if x % 2 == 0 { continue; }

    println!("{}", x);
}

//# 循环标签
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
        println!("x: {}, y: {}", x, y);
    }
}
```

### 4.7 Vectors

“Vector”是一个动态或“可增长”的数组，被实现为标准库类型 `Vec<T>`（其中<T>是一个泛型语句）。`vector` 总是在堆上分配数据。`vector` 与切片就像 `String` 与 `&str` 一样。

```rust
let v = vec![1, 2, 3, 4, 5];  // v: Vec<i32>
let v = vec![0; 10];  // ten zeros

let v = vec![1, 2, 3, 4, 5];
// vector 特定索引的值，使用[]
println!("The third element of v is {}", v[2]);


let v = vec![1, 2, 3, 4, 5];

let i: usize = 0;
let j: i32 = 0;

// Works:
v[i];

// Doesn’t:
v[j];

//# 越界访问
let v = vec![1, 2, 3];
println!("Item 7 is {}", v[7]);


let v = vec![1, 2, 3];
match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
}

//# 迭代
let mut v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}
    // 你不能在使用 vector 的所有权遍历之后再次遍历它。你可以使用它的引用多次遍历 vector。
for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
```

### 4.8 所有权

>   Rust 中的变量绑定有一个属性：它们有它们所绑定的的值的所有权。这意味着当一个绑定离开作用域，它们绑定的资源就会被释放。

然而这里有更巧妙的地方：Rust 确保了对于任何给定的资源都正好（只）有一个绑定与之对应。例如，如果我们有一个 vector，我们可以把它赋予另外一个绑定：

```rust
let v = vec![1, 2, 3];

let v2 = v;
```

不过，如果之后我们尝试使用v，我们得到一个错误：

```rust
let v = vec![1, 2, 3];

let v2 = v;

println!("v[0] is: {}", v[0]);
```

它看起来像这样：

```rs
error: use of moved value: `v`
println!("v[0] is: {}", v[0]);
                        ^
```

一样的错误：“use of moved value”。当我们把所有权转移给别的绑定时，我们说我们“移动”了我们引用的值。这里你并不需要什么类型的特殊注解，这是 Rust 的默认行为。

### 4.9 引用和借用

#### &T 借用

```rust
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // Do stuff with `v1` and `v2`.

    //  Return the answer.
    42
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let answer = foo(&v1, &v2);

// We can use `v1` and `v2` here!
println!("v1 : {}, v2: {}, answer: {}", v1, v2, answer);
```

#### &mut T 引用

```rust
let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);
```

这会打印 `6` 。我们让 `y` 是一个 `x` 的可变引用，接着把 `y` 指向的值加一。你会注意到 `x` 也必须被标记为 `mut` ，如果它不是，我们不能获取一个不可变值的可变引用。

你也会发现我们在 `y` 前面加了一个星号（`*`），成了 `*y` ，这是因为 `y` 是一个 `&mut` 引用。你也需要使用他们（星号）来访问引用的内容。

#### 规则

Rust 中的借用有一些规则：

第一，任何借用必须位于比拥有者更小的作用域。第二，对于同一个资源（resource）的借用，以下情况不能同时出现在同一个作用域下：

- 1 个或多个不可变引用（&T）
- 唯一 1 个可变引用（&mut T）

译者注：即同一个作用域下，要么只有一个对资源 A 的可变引用（&mut T），要么有 N 个不可变引用（&T），但不能同时存在可变和不可变的引用

#### 理解作用域

这是代码：

```rust
fn main() {
    let mut x = 5;
    let y = &mut x;

    *y += 1;

    println!("{}", x);
}
```

这些代码给我们如下错误：

```rust
error: cannot borrow `x` as immutable because it is also borrowed as mutable
    println!("{}", x);
                   ^
```

这是因为我们违反了规则：我们有一个指向x的&mut T，所以我们不允许创建任何&T。一个或另一个。

......

### 4.10 生命周期

待续
