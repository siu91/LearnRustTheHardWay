/* ANCHOR: all */

// ANCHOR: hello_world
fn hello_world() {
    println!("Hello World!");
}
// ANCHOR_END: hello_world


// ANCHOR: data_types
// 熟悉基本的数据类型
fn data_types() {
    // integer 整型
    let int1 = 3; // 默认就是 i32，整形 32
    let int2: i32 = 3;// 也可以这样定义
    let int3 = 3_i32;// 还可以这样定义
    let int4 = 3i32;// 还可以这样定义
    println!("{}/{}/{}/{}", int1, int2, int3, int4);
    // float 浮点型
    let float1 = 3.1;
    let float2: f32 = 3.1;
    assert_eq!(float1, float2);

    // range 打印1-4，有点像 python是不是
    for i in 1..5 {
        println!("{}", i)
    }
    // 打印 1-5
    for i in 1..=5 {
        println!("{}", i)
    }
}
// ANCHOR_END: data_types

// ANCHOR: some_syntax
// 熟悉一些基础的语法
fn some_syntax() {
    // 使用 let 声明变量，绑定值，x 为不可变；
    // 默认编译器为进行"类型推断"，所以 x 类型为 i32
    // 语句末尾分号;
    let x0 = 3;
    let x1: i32 = 3; // 显示指定类型
    let x2 = 3i32; // 显示指定类型
    let x3 = 3_i32;// 显示指定类型
    // x0 = 5; // 这一行会报错，因为 x 声明是不可变：Cannot assign twice to immutable variable [E0384]

    // 和 Java 一样，函数的返回值可以作为函数的参数
    let x4 = add(add(x0, x1), add(x2, x3));

    // m1 是可变的， mutable
    let mut m1 = 3;

    // println! 是宏调用，这个不理解，需要后面关注
    // {} 用于占位，区别于其它语言 %d %s
    println!("可变 m1={}", m1);
    m1 = 4;
    println!("可变 m1={}", m1);
}

// 定义一个函数：参数，返回值
// fn <函数名>([参数名: 参数类型,...])[ -> 返回值类型] {}
fn add(a: i32, b: i32) -> i32 {
    // return 可以省略，这里会返回 a+b 的值；表达式是可返回的
    a + b
}
// ANCHOR_END: some_syntax


// ANCHOR: mutable
fn mutable() {
    let a = 5;
    // a = 6; // 报错:Cannot assign twice to immutable variable [E0384]

    let mut b = 5;
    b - 6;
}

// ANCHOR_END: mutable

// ANCHOR: destructure
fn destructure() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a={},b={}", a, b);
    b = true;
    println!("a={},b={}", a, b);
    assert_eq!(a, b)
}
// ANCHOR_END: destructure

// ANCHOR: const
const MAX_LEVEL: i32 = 4;
// ANCHOR_END: const

// ANCHOR: int_type
// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
// 如果使用 checked_* 方法时发生溢出，则返回 None 值
// 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
// 使用 saturating_* 方法使值达到最小值或最大值
fn int_type() {
    // deal with wrapping.
    println!("{} {} {} {}",
             200u8.wrapping_add(57),  // 1.
             200u8.overflowing_add(57).0,  // (1, true) -> 1.
             if 200u8.checked_add(57) == None { "overflow" } else { "not overflow" },
             200u8.saturating_add(57),  // 255 (bound to the edge values).
    );

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
}
// ANCHOR_END: int_type

// ANCHOR: float_type
fn float_type() {
    let f1 = 32.4;
    let f2: f32 = 34.8;

    // 四则运算
    let add = 5 + 6;
    let sub = 3.2 - 1.6;
    let mul = 3 * 4;
    let div = 5 / 6;
    // 取余
    let remainder = 41 % 5;

    // let i = 3 + 3.4; // 不允许不同类型的运算： no implementation for `{integer} + {float}`

    // 定义一个 f32 的数组
    let arr = [31.0, 31.0_f32, 31.0f32];
    // 打印保留两位小数
    println!("{:.2}", arr[0])
}

//ANCHOR_END: float_type

fn float_error() {
    let abc: (f32, f32, f32) = (30.1, 30.1, 30.1);
}

fn bool_type() {
    // ANCHOR: bool
    let t = true;
    let f: bool = false; // 显式声明
    // ANCHOR_END: bool
}

// ANCHOR: char
fn char_type() {
    let emoji: char = '😊';
    let cn = '中';
    println!("字符占{}个字节", std::mem::size_of_val(&emoji))
}
// ANCHOR_END: char


// ANCHOR: tuple
fn tuple_type() {
    let tup: (i32, f32, u8) = (32, 32.1, 32);
    // 使用"模式匹配"来"解构"元组
    let (x, y, z) = tup;
    println!("y 的值：{}", y)
}
// ANCHOR_END: tuple

// ANCHOR: var_scope
fn var_scope() {
    let var1 = "test"; // 声明，作用域开始
    // ... 使用 var1
}
// 作用域结束
// ANCHOR_END: var_scope

// ANCHOR: move_ownership1
fn move_ownership1() {
    let x = 5;
    let y = x;

    println!("x={},y={}", x, y)
}
// ANCHOR_END: move_ownership1

// ANCHOR: move_ownership2
fn move_ownership2() {
    let s1 = "s111";
    let s2 = s1;

    println!("s1={},s2={}", s1, s2)
}
// ANCHOR_END: move_ownership2

// ANCHOR: move_ownership3
fn move_ownership3() {
    let s1 = String::from("s111");
    let s2 = s1;

    //println!("s1={},s2={}", s1, s2)
}
// ANCHOR_END: move_ownership3


fn super_add1<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}


// ANCHOR: trait
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}

struct Teacher {}


impl Hello for Student {
    fn say_something(&self) -> String {
        "i am a Student".to_string()
    }
}

impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("hello")
    }

    fn say_something(&self) -> String {
        "i am a teacher".to_string()
    }
}


// ANCHOR_END: trait


// ANCHOR: hashmap
use std::collections::HashMap;

fn word_count() {
    let text = "i am Rust developer,i love Rust";
    let mut counter = HashMap::new();
    for word in text.split_whitespace() {
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", counter)
}
// ANCHOR_END: hashmap

// ANCHOR：panic

use std::net::IpAddr;

fn panic_unwrap() {
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
}

// ANCHOR_END：panic


use std::fs::File;
use std::io::ErrorKind;


fn open_file() {
    let f = File::open("Cargo1.toml");
    let f = match f {
        Ok(file) => {
            println!("open file success");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Cargo1.toml") {
                Ok(f1) => {
                    println!("create file");
                    f1
                }
                Err(e) => panic!("create file error")
            },
            oe => panic!("open file error:{:?}", oe)
        }
    };
    println!("s");


    let f2 = File::open("Cargo1.toml").expect("can not open file");
}

fn print_something(s: &'static str) {
    println!("some:{}", s);
}

fn iters() {
    let values = vec![1, 2, 3];

    for v in values.iter() {
        println!("v={}", v)
    }

    println!("{:?}", values);


    let values = vec![1, 2, 3];
}

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = nums.len();
        return if length >= 2 {
            for i in 0..length - 1 {
                for j in 0..=length - 1 {
                    if i == j {
                        continue;
                    }
                    if nums.get(j).unwrap() + nums.get(i).unwrap() == target {
                        return vec![i as i32, j as i32];
                    }
                }
            }
            vec![-1, -1]
        } else {
            vec![-1, -1]
        };
    }
}


fn main() {
    let nums = vec![3, 3];
    let target = 6;
    let s = Solution::two_sum(nums, target);

    let nums = vec![2, 5, 5, 11];
    let target = 10;
    let s = Solution::two_sum(nums, target);

    let nums = vec![3, 2, 4];
    let target = 6;
    let s = Solution::two_sum(nums, target);


    let nums = vec![-1, -2, -3, -4, -5];
    let target = -8;
    let s = Solution::two_sum(nums, target);

    hello_world();
    data_types();
    some_syntax();
    destructure();
    int_type();
    float_type();
    char_type();
    tuple_type();
    move_ownership1();
    move_ownership2();
    //move_ownership3();
    super_add1(1.1, 1.3);

    let s = Student {};
    s.say_hi();
    s.say_something();
    word_count();
    open_file();

    let s = "hiiiii";
    print_something(s);
    iters()
}



/* ANCHOR_END: all */