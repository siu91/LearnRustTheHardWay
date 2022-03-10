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
    let int32 = 3; // 默认就是 i32，整形 32
    let int32_1: i32 = 3;// 也可以这样定义
    let int32_2 = 3_i32;// 还可以这样定义
    println!("{}",3)
}
// ANCHOR_END: data_types

fn main() {
    hello_world();
    data_types();
}



/* ANCHOR_END: all */