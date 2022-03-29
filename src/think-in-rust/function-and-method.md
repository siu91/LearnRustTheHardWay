# 函数和方法

- 在面向对象（OOP）时很少会考虑什么是函数，如 Java 在反射（reflect）中有 Method 的概念，即 Class 中的方法：someClass.getMethods()；
- 理解 1：函数（Function）是面向过程的”产物“，在具有函数式编程（PF）范式的语言中都有很多具体的体现：C、Rust

- 理解 2：方法（Method）是面向对象的”产物“，在具有面向对象编程（OOP）范式的语言中都有很多具体的体现：Java、Rust
  
  ```java
  // class in Java
  Class MyClass {
      // data
      // methods
  }
  ```

  ```rust
  // struct/enum in Rust
  struct MyStruct {
      // data
  }

  impl MyStruct {
      // methods
  }
  
  ```
