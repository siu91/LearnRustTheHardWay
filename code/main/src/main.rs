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
    for i in 1..5{
        println!("{}",i)
    }
    // 打印 1-5
    for i in 1..=5{
        println!("{}",i)
    }
}
// ANCHOR_END: data_types

fn main() {
    hello_world();
    data_types();
}



/* ANCHOR_END: all */