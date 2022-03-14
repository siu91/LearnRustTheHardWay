# 数据类型

Rust 是静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。当然**类型推断**可以帮我们标注默认的类型。

## 标量类型（Scare Type）

- 数值类型: 有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
- 字符串：字符串字面量和字符串切片 &str
- 布尔类型： true和false
- 字符类型: 表示单个 Unicode 字符，存储为 4 个字节

## 单元类型（Unit Type）

是一种特殊的类型，即 () ，其唯一的值也是 ()。

- unit type是一个类型，有且仅有一个值，都写成小括号()；
类似c/c++/java语言中的void。当一个函数并不需要返回值的时候，c/c++/java中函数返回void，rust则返回()。但语法层面上，void仅仅只是一个类型，该类型没有任何值;而单位类型()既是一个类型，同时又是该类型的值。
- 单元类型()也类似c/c++/java中的null，但却有很大不同。 null是一个特殊值，可以赋给不可类型的值，例如java中的对象，c中指向struct实例的指针，c++中的对象指针。但在rust中，()不可以赋值给除单元类型外的其它的类型的变量，()只能赋值给()。
- Rust标准库中使用单元类型()的一个例子是HashSet。一个HashSet只不过是HashMap的一个非常简单地包裹，写作：HashMap<T, ()>。HashMap的第二个泛型类型参数即用了单元类型()
- 可以用Result<(), MyErrorType>代替Option，某些开发者认为Result<(), MyErrorType>语义上能更简明地表示一个“结果”。

## 复合类型（Compound Types）

复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
