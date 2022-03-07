> 走进 Rust 的世界

## Rust 的发展历程
&#8195;&#8195; Rust是由Mozilla主导开发的通用、编译型编程语言。设计准则为“安全、并发、实用”，支持函数式、并发式、过程式以及面向对象的程序设计风格。

&#8195;&#8195; Rust语言原本是Mozilla员工Graydon Hoare的私人计划，而Mozilla于2009年开始赞助这个计划，并且在2010年首次公开。也在同一年，其编译器源代码开始由原本的OCaml语言转移到用Rust语言，进行bootstrapping工作，称做“rustc”，并于2011年实际完成。这个可自我编译的编译器在架构上采用了LLVM做为它的后端。

&#8195;&#8195; 第一个有版本号的Rust编译器于2012年1月发布。Rust 1.0是第一个稳定版本，于2015年5月15日发布。

&#8195;&#8195; Rust在完全开放的情况下开发，并且相当欢迎社区的反馈。在1.0稳定版之前，语言设计也因为透过撰写Servo网页浏览器排版引擎和rustc编译器本身，而有进一步的改善。它虽然由Mozilla资助，但其实是一个共有项目，有很大部分的代码是来自于社区的贡献者。

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


## 知名的 Rust 项目
- TiDB： HTAP 数据库，兼容 Mysql
- Databend：The Modern Cloud Data Warehouse for Everyone.
- Yew：是一个设计先进的 Rust 框架，目的是使用 WebAssembly 来创建多线程的前端 web 应用。




## Ref
 - [Rsut 维基百科](https://zh.wikipedia.org/wiki/Rust)
