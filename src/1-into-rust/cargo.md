# 包管理：Cargo



&#8195;&#8195; 为了让开发者更方便地相互协作，**Rust** 提供了非常好用的包管理器 Cargo。Rust 代码是以包（crate）为编译和分发单位的，Cargo 提供了很多命令，方便开发者创建、构建、分发、管理自己的包。Cargo 也提供插件机制，方便开发者编写自定义的插件，来满足更多的需求。比如官方提供的 rustfmt 和 clippy 工具，分别可以用于自动格式化代码和发现代码中的“坏味道”。再比如，rustfix 工具甚至可以帮助开发者根据编译器的建议自动修复出错的代码。Cargo 还天生拥抱开源社区和 Git，支持将写好的包一键发布到 crates.io 网站，供其他人使用。

&#8195;&#8195; Cargo 就像 Node.js/npm、Java/Maven 一样。

## 动手试试 Cargo

### 创建项目：cargo new

```shell
siu@localhost code % cd hello_world 
siu@localhost hello_world % cargo run
   Compiling hello_world v0.1.0 (/Users/siu/Desktop/LearnRustTheHardWay/code/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/hello_world`
Hello, world!
```

项目结构：

```shell
.
├── Cargo.toml
└── src
    └── main.rs
```



### 运行：cargo run

```shell
siu@localhost code % cd hello_world 
siu@localhost hello_world % cargo run
   Compiling hello_world v0.1.0 (/Users/siu/Desktop/LearnRustTheHardWay/code/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/hello_world`
Hello, world!
```

#### 编译后，再运行：cargo build

编译：

```shell
siu@localhost hello_world % cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
```

运行：

```shell
siu@localhost hello_world % ./target/debug/hello_world 
Hello, world!
```

#### 编译优化：--release

默认运行的是`debug`模式，Rust 编译器不会做任何的优化，`--release`可以做编译优化。

- cargo build --release
- cargo run --release

编译：

```shell
siu@localhost hello_world % cargo build --release
   Compiling hello_world v0.1.0 (/Users/siu/Desktop/LearnRustTheHardWay/code/hello_world)
    Finished release [optimized] target(s) in 0.61s
```

运行：

```shell
siu@localhost hello_world % ./target/release/hello_world 
Hello, world!
```

#### 编译检查：cargo check

&#8195;&#8195; Rust 编译速度相对会比较慢（Golang），当项目 较大时，可以使用 `cargo check`来快速验证代码是否能通过编译。

```shell
siu@localhost hello_world % cargo check
    Checking hello_world v0.1.0 (/Users/siu/Desktop/LearnRustTheHardWay/code/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
```

## Cargo.toml

`Cargo.toml`  是 `cargo` 的依赖库配置文件，类似 Java/.pom，Golang/go.mod 文件。

> 在项目中看到一个`Cargo.lock` 文件，它是 `cargo` 根据同一项目的 `toml` 文件生成的，通常不需要关注，类似 Golang/go.sum 。
>
> 只有当项目是一个可运行的程序时才把`Cargo.lock` 放到 git 仓库；如果是一个依赖库项目则不需要。

### TOML

TOML 是一种旨在成为一个小规模、易于使用的语义化的配置文件格式，它被设计为可以无二义性的转换为一个哈希表。

“TOML”这个名字是“Tom's Obvious, Minimal Language（汤姆的浅显的、极简的语言）”的首字母略写词。“Tom”指它的作者Tom Preston-Werner。

TOML已在一些软件工程中使用，并且很多编程语言都支持TOML格式数据的解析。

```txt
来源：维基百科 https://zh.wikipedia.org/wiki/TOML
```

### Cargo.toml 文件配置

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

#### [package]

&#8195;&#8195; `[package]` 中记录了项目的描述信息：

- `name`: 项目名称
- `version`: 项目版本号，参考 [Semver 2.0.0](https://semver.org/lang/zh-CN/)
- `edition`: Rust 大版本

### [dependencies]

三种方式添加依赖：

- 基于 Rust 官方仓库 `crates.io`，通过版本说明来描述
- 基于项目源代码的 git 仓库地址，通过 URL 来描述
- 基于本地项目的绝对路径或者相对路径，通过类 Unix 模式的路径来描述

```toml
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```

## cartes.io

&#8195;&#8195; cartes.io 相当于 Rust 的一个官方公共仓库，很多基础库和框架都会以 包（crate） 的方式发布到 [crates.io](https://crates.io/) 。可以在上面找到别人已经开发好的库，也可以发布自己的开发的库。