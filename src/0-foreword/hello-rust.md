# Hello Rust

## Rust 的发展历程
&#8195;&#8195; Rust是由Mozilla主导开发的通用、编译型编程语言。设计准则为“安全、并发、实用”，支持函数式、并发式、过程式以及面向对象的程序设计风格。

&#8195;&#8195; Rust语言原本是Mozilla员工Graydon Hoare的私人计划，而Mozilla于2009年开始赞助这个计划，并且在2010年首次公开。也在同一年，其编译器源代码开始由原本的OCaml语言转移到用Rust语言，进行bootstrapping工作，称做“rustc”，并于2011年实际完成。这个可自我编译的编译器在架构上采用了LLVM做为它的后端。

&#8195;&#8195; 第一个有版本号的Rust编译器于2012年1月发布。Rust 1.0是第一个稳定版本，于2015年5月15日发布。

&#8195;&#8195; Rust在完全开放的情况下开发，并且相当欢迎社区的反馈。在1.0稳定版之前，语言设计也因为透过撰写Servo网页浏览器排版引擎和rustc编译器本身，而有进一步的改善。它虽然由Mozilla资助，但其实是一个共有项目，有很大部分的代码是来自于社区的贡献者。

##  Rust 语言的特点
 - 高性能 - Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务，可以在嵌入式设备上运行，还能轻松和其他语言集成。

 - 可靠性 - Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就能够消除各种各样的错误。

 - 生产力 - Rust 拥有出色的文档、友好的编译器和清晰的错误提示信息， 还集成了一流的工具 —— 包管理器和构建工具， 智能地自动补全和类型检验的多编辑器支持， 以及自动格式化代码等等。

## C++、Java & Rust

...
你看C++就很相信人类，它要求人类自己把自己new出来的东西给delete掉。


C++：“这点小事我相信你可以的！”

人类：“没问题！包在我身上！”然后呢，内存泄漏、double free、野指针满世界飘……
C++：“……”



Java选择不相信人类，但替人类把事办好。

Java：“别动，让我来，我有gc！”

人类：“你怎么做事这么慢呀？你怎么还stop the world了呀？你是不是不爱我了呀？

”Java：“……”




Rust发现唯一的办法就是既不相信人类，也不惯着人类。

Rust：“按老子说的做，不做就不编译！”

人类：“你反人类！”

Rust：“滚！”

```txt
作者：左之了
链接：https://www.zhihu.com/question/328066906/answer/708085473
来源：知乎
``````

## Rust 的应用
Rust 语言可以用于开发：

 - 传统命令行程序 - Rust 编译器可以直接生成目标可执行程序，不需要任何解释程序。
 - Web 应用 - Rust 可以被编译成 WebAssembly，WebAssembly 是一种 JavaScript 的高效替代品。
  - 网络服务器 - Rust 用极低的资源消耗做到安全高效，且具备很强的大规模并发处理能力，十分适合开发普通或极端的服务器程序。
 - 嵌入式设备 - Rust 同时具有JavaScript 一般的高效开发语法和 C 语言的执行效率，支持底层平台的开发。

### 知名的 Rust 项目
- TiDB： HTAP 数据库，兼容 Mysql
- Databend：The Modern Cloud Data Warehouse for Everyone.
- Yew：是一个设计先进的 Rust 框架，目的是使用 WebAssembly 来创建多线程的前端 web 应用。

## 试试在线 Play
Rust 官方在线工具: [https://play.rust-lang.org/](https://play.rust-lang.org/)


## Ref
 - [Rust 维基百科](https://zh.wikipedia.org/wiki/Rust)
 - [Rust 菜鸟教程](https://m.runoob.com/rust/rust-tutorial.html)
