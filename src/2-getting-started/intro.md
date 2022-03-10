# Rust 基本概念

下面是 Rust 语言的架构图，之前在知乎上看到的，为了加深体系的理解，自己动手“临摹”了一遍。

![rust-arch](./assets/rust-lang-arch.svg)

- 第一层编程范式：OOP 和 FP，比较熟悉；OOP，面向对象（封装、继承、多态） ，典型如 Java；FP 典型如 Scala；其实很多语言里面都有 FP 的影子
- 第二层语义：相对都会非常陌生，借用/所有权/生命周期
- 第三层类型：一切都是类型是理解的重点
- 第四层内存管理：安全、安全、安全

## ref

[Rust 如何使用堆栈](https://rust-book.junmajinlong.com/ch5/02_rust_mem.html)
