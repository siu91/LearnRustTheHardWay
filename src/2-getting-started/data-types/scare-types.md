# 标量类型

**标量**（*scalar*）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

## 整形

| 长度       | 有符号类型 | 无符号类型 |
| ---------- | ---------- | ---------- |
| 8 位       | `i8`       | `u8`       |
| 16 位      | `i16`      | `u16`      |
| 32 位      | `i32`      | `u32`      |
| 64 位      | `i64`      | `u64`      |
| 128-位     | `i128`     | `u128`     |
| 视架构而定 | `isize`    | `usize`    |

知识点：

- i 表示 integer
- u 表示 unsigned，无符号，只取正数
- `let i = 3;`默认是 i32
- 有符号类型数字范围是 -(2<sup>n - 1</sup>) ~ 2<sup>n -
1</sup> - 1，其中 `n` 是长度；i8 -128～127
- 无符号类型数字范围是 0 ~ 2<sup>n</sup> - 1，其中 `n` 是长度；u8 0～255

Rust 中的整型字面值

| 数字字面值                    | 例子          |
| ----------------------------- | ------------- |
| Decimal (十进制)              | `98_222`      |
| Hex (十六进制)                | `0xff`        |
| Octal (八进制)                | `0o77`        |
| Binary (二进制)               | `0b1111_0000` |
| Byte (单字节字符)(仅限于`u8`) | `b'A'`        |

> ##### 整型溢出
>
> 比方说有一个 `u8` ，它可以存放从 0 到 255 的值。那么当你将其修改为范围之外的值，比如 256，则会发生**整型溢出**。关于这一行为 Rust 有一些有趣的规则：当在 debug 模式编译时，Rust 会检查整型溢出，若存在这些问题，则使程序在编译时 *panic*(崩溃,Rust 使用这个术语来表明程序因错误而退出)。
>
> 在当使用 `--release` 参数进行 release 模式构建时，Rust **不**检测溢出。相反，当检测到整型溢出时，Rust 会按照补码循环溢出（*two’s complement wrapping*）的规则处理。简而言之，大于该类型最大值的数值会被补码转换成该类型能够支持的对应数字的最小值。比如在 `u8` 的情况下，256 变成 0，257 变成 1，依此类推。程序不会 *panic*，但是该变量的值可能不是你期望的值。依赖这种默认行为的代码都应该被认为是错误的代码。
>
> 要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
>
> - 使用 `wrapping_*` 方法在所有模式下都按照补码循环溢出规则处理，例如 `wrapping_add`
> - 如果使用 `checked_*` 方法时发生溢出，则返回 `None` 值
> - 使用 `overflowing_*` 方法返回该值和一个指示是否存在溢出的布尔值
> - 使用 `saturating_*` 方法使值达到最小值或最大值

```rust
{{#include ../../../code/main/src/main.rs:int_type}}
{{#include ../../../code/main/src/.template:1:2}}
 int_type();
{{#include ../../../code/main/src/.template:3:3}}
```

## 浮点型（f32/f64）

知识点：

- 在 Rust 中浮点类型数字也有两种基本类型： `f32` 和 `f64`，分别为 32 位和 64 位大小。
- 默认浮点类型是 `f64`，在现代的 CPU 中它的速度与 `f32` 几乎相同，但精度更高。

```rust
{{#include ../../../code/main/src/main.rs:float_type}}
{{#include ../../../code/main/src/.template:1:2}}
 flaot_type();
{{#include ../../../code/main/src/.template:3:3}}
```

#### 浮点数陷阱

浮点数由于底层格式的特殊性，导致了如果在使用浮点数时不够谨慎，就可能造成危险，有两个原因：

1. **浮点数往往是你想要数字的近似表达** 浮点数类型是基于二进制实现的，但是我们想要计算的数字往往是基于十进制，例如 `0.1` 在二进制上并不存在精确的表达形式，但是在十进制上就存在。这种不匹配性导致一定的歧义性，更多的，虽然浮点数能代表真实的数值，但是由于底层格式问题，它往往受限于定长的浮点数精度，如果你想要表达完全精准的真实数字，只有使用无限精度的浮点数才行
2. **浮点数在某些特性上是反直觉的** 例如大家都会觉得浮点数可以进行比较，对吧？是的，它们确实可以使用 `>`，`>=` 等进行比较，但是在某些场景下，这种直觉上的比较特性反而会害了你。因为 `f32` ， `f64` 上的比较运算实现的是 `std::cmp::PartialEq` 特征(类似其他语言的接口)，但是并没有实现 `std::cmp::Eq` 特征，但是后者在其它数值类型上都有定义，说了这么多，可能大家还是云里雾里，用一个例子来举例：

Rust 的 `HashMap` 数据结构，是一个 KV 类型的 HashMap 实现，它对于 `K` 没有特定类型的限制，但是要求能用作 `K` 的类型必须实现了 `std::cmp::Eq` 特征，因此这意味着你无法使用浮点数作为 `HashMap` 的 `Key`，来存储键值对，但是作为对比，Rust 的整数类型、字符串类型、布尔类型都实现了该特征，因此可以作为 `HashMap` 的 `Key`。

这个问题和 Java 中用 Float 作为 HashMap 的 Key 类似，equal() 调用 hashCode() 实现。

为了避免上面说的两个陷阱，你需要遵守以下准则：

- 避免在浮点数上测试相等性
- 当结果在数学上可能存在未定义时，需要格外的小心

## 布尔型（bool）

布尔类型使用 `bool` 声明，只有 `true` 和 `fasle` 两个值，占 1 个字节。


```rust
{{#include ../../../code/main/src/main.rs:bool}}
```

## 字符类型（char） 
Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型。Unicode 值的范围从 `U+0000 ~ U+D7FF` 和 `U+E000 ~ U+10FFFF`。

由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节：
```rust
{{#include ../../../code/main/src/main.rs:char}}
{{#include ../../../code/main/src/.template:1:2}}
 char_type();
{{#include ../../../code/main/src/.template:3:3}}
```