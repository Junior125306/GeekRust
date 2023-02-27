# 第三讲主要内容

## 第一个 rust 程序

- `cargo new project_name` 命令创建项目
- 在 `main.rs` 写程序内容

```rust
    fn main() {
        println!("Hello world!");
    }
```

- `cargo run` 运行

## 陈天推荐的 vscode 插件

1. rust-analyzer：它会实时编译和分析你的 Rust 代码，提示代码中的错误，并对类型进行标注。你也可以使用官方的 Rust 插件取代。 ✅
2. rust syntax：为代码提供语法高亮。⭕ 没有装也有高亮可能主题支持。
3. crates：帮助你分析当前项目的依赖是否是最新的版本。✅
4. better toml：Rust 使用 toml 做项目的配置管理。better toml 可以帮你语法高亮，并展示 toml 文件中的错误。✅
5. rust test lens：可以帮你快速运行某个 Rust 测试 ✅
6. Tabnine：基于 AI 的自动补全，可以帮助你更快地撰写代码。✅

## 第一个实用的 Rust 程序

> 程序需需求很简单，通过 HTTP 请求 Rust 官网首页，然后把获得的 HTML 转换成 Markdown 保存起来
> 使用 reqwest , html2md,reqwest 是一个 HTTP 客户端，它的使用方式和 Python 下的 request 类似；html2md 顾名思义，把 HTML 文本转换成 Markdown。

👇main.rs 代码

```rust

use std::fs;

fn main() {
  let url = "https://www.rust-lang.org/";
  let output = "rust.md";

  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}

```

综上得出结论

- **Rust使用cargo来管理项目，类似node.js的npm**
- **Rust整理语法偏向C/C++风格**
- **Rust虽然是强类型语言但是编译器支持类型推导**
- *Rust支持宏编程 比如 println!() 就是一个宏*

综上没有得出rust的特点有

- Rust的变量默认是不可变的，如果修改变量的值，需要显式的使用mut关键字
- 除了 let / static / const / fn 等少数语句外，Rust 绝大多数代码都是表达式（expression）。所以 if / while / for / loop 都会返回一个值，函数最后一个表达式就是函数的返回值，这和函数式编程语言一致。
- Rust 支持面向接口编程和泛型编程。
- Rust 有非常丰富的数据类型和强大的标准库。
- Rust 有非常丰富的控制流程，包括模式匹配（pattern match）。

## 函数定义 和 返回值

```rust
    // 这是一个 函数 返回 f64 浮点型
    fn pi() -> f64 { 3.1415926}
    // 这是一个函数 返回 unit ()
    fn no_pi() -> f64 { 3.1415926;}
    // 这是一个函数 它可以作为返回值
    println!("pi is {:?}",pi())
```

## 数据结构

``` rust
 // 这是一个枚举
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}
// 这是一个结构体，元组结构体，它的域都是匿名的，可以用索引访问，适用于简单的结构体
#[derive(Debug, Copy, Clone)]
struct UserId(u64);
// 这也是一个结构体，标准结构体，可以是多种类型的组合
#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

```

## 总结一下变量、函数、数据结构

![极客时间·陈天·Rust第一课](https://static001.geekbang.org/resource/image/15/cb/15e5152fe2b72794074cff40041722cb.jpg?wh=1920x1898)

## 控制流程

> 循环可以使用 死循环 loop、 条件循环while、以及对迭代器的循环for。
>
> 循环可以使用 break 提前终止，或者使用 continue 来跳到下一轮循环中



> 满足某个条件时会跳转,比如使用 if/else ，match， 错误/异常跳转



使用rust实现斐波那契数列

```rust

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
```

> 这里需要指出的是，Rust 的 for 循环可以用于任何实现了 IntoIterator trait 的数据结构。在执行过程中，IntoIterator 会生成一个迭代器，for 循环不断从迭代器中取值，直到迭代器返回 None 为止。因而，for 循环实际上只是一个语法糖，编译器会将其展开使用 loop 循环对迭代器进行循环访问，直至返回 None

![极客时间·陈天·Rust第一课](https://static001.geekbang.org/resource/image/e3/6c/e3a96ae58a98f46f98b56yya6378b26c.jpg?wh=1920x2144)

## 模式匹配

```rust
    fn process_event(event: &Event) {
        match event {
            Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
            Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
            Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
        }
    }

    fn process_message(event: &Event) {
        if let Event::Message((_, _, msg)) = event {
            println!("broadcast: {}", msg);
        }
    }

```

## 错误处理

> 之后详细记录

## 项目组织

![极客时间·陈天·Rust第一课](https://static001.geekbang.org/resource/image/8e/1d/8eff27daa16a2bab514590f0b567341d.jpg?wh=1761x1381)

> 一个 workspace 可以包含一到多个 crates，当代码发生改变时，只有涉及的 crates 才需要重新编译。当我们要构建一个 workspace 时，需要先在某个目录下生成一个如图所示的 Cargo.toml，包含 workspace 里所有的 crates，然后可以 cargo new 生成对应的 crates

![极客时间·陈天·Rust第一课](https://static001.geekbang.org/resource/image/2b/62/2bf542e266197e04ededc5c4a6e6cf62.jpg?wh=1920x1134)
