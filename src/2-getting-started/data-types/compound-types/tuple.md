# 元组类型（tuple）

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组**长度固定**：一旦声明，其长度不会增大或缩小。

元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的：

```rust
{{#include ../../../../code/main/src/main.rs:tuple}}
{{#include ../../../../code/main/src/.template:1:2}}
 tuple_type();
{{#include ../../../../code/main/src/.template:3:3}}
```