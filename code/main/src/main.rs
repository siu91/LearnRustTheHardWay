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
        // 算法思路：
        // 输入     nums = [2,7,11,15], target = 9
        // 得到差值  subs = [7,2,-3,-6]  带上下标表示 subMap = {<7,0>,<2,1>,<-3,2>,<-6,3>}
        // 1、用 Map<sub,index> 保存每个数的差值和下标
        // 2、遍历数组：如果 Map 中查找匹配到了当前"差值"，直接返回两个数的索引下标，否则保存"差值和下标"
        let mut map = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            let sub = target - nums[i];
            if let Some(s) = map.get(&sub) {
                if *s != i as i32 {
                    return vec![*s, i as i32];
                }
            };

            map.insert(sub, i as i32);
        }
        vec![-1, -1]
    }

    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j &&
                    nums[j] + nums[i] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![-1, -1]
    }
}

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub next: Option<Box<ListNode>>,
    val: i32,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        let mut t = (l1, l2, 0, 0); // (l1,l2,sum,carry)
        loop {
            t = match t {
                // 遍历完两个链表为空，sum=any，carry=0（无进位）；退出
                (None, None, _, 0) => break,
                // 遍历完两个链表为空，还有进位，把进位放在 sum 位置，用于初始化下一个节点（补1）
                (None, None, _, carry) => (None, None, carry, 0),
                // 任意链表为空的情况（1个链表已经遍历完了），但有进位，list.next、None 可以放在1和2任意位置
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    let s = if list.val + carry >= 10 { list.val + carry - 10 } else { list.val + carry };
                    let c = if list.val + carry >= 10 { 1 } else { 0 };
                    (list.next, None, s, c)
                }
                // 正常遍历链表，计算 sum 和 carry
                (Some(l1), Some(l2), _, carry) => {
                    let s1 = l1.val + l2.val + carry;
                    let s = if s1 >= 10 { s1 - 10 } else { s1 };
                    let c = if s1 >= 10 { 1 } else { 0 };
                    (l1.next, l2.next, s, c)
                }
            };
            // 处理 tail
            /*if t.0 != None {
                println!("ttttt1:{:?}",t.0.as_ref().unwrap().val);
            }

            if t.1 != None {
                println!("ttttt2:{:?}",t.1.as_ref().unwrap().val);
            }*/

            // 把 sum(t.2) 用于初始化当前节点
            *tail = Some(Box::new(ListNode::new(t.2)));
            // next
            //tail = &mut tail.next;
            tail = &mut tail.as_mut().unwrap().next;
        }

        return head;
    }
}

use std::collections::HashSet;
use std::cmp::{max, Ordering};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 算法思路：
        // 输入 "anviaj"
        // 转化 字符串转成 char 数组 [a,n,v,i,a,j]
        // 窗口每次滑动1 [a,n,v,i,a,j] [n,v,i,a,j] [v,i,a,j] [i,a,j] ...

        // 1、全局定义一个 longest 用于max 每个窗口的最长子串
        // 2、从 i =0 开始滑动遍历数组：用 start 标记每个窗口内最长的子串（滑动的轨迹），HashSet 记录窗口内重复的字符串

        let mut longest = 0;
        let s: Vec<_> = s.chars().collect(); // anviaj

        for i in 0..s.len() {
            // 用 start 去标记滑动的位置，set 记录窗口内重复的字符串
            let mut set = HashSet::with_capacity(s.len());
            let mut start = i;
            while start < s.len() {
                if !set.contains(&s[start]) {
                    set.insert(s[start]);
                    start += 1;
                } else { break; }
            }
            longest = max(longest, start - i);
        }

        longest as i32
    }
}

use std::cmp::min;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 算法思路：
        // 1、确定中位数的下标 i 或 i、j
        // 2、在 nums1 nums2 中找到第 i 或 i、j 大的值
        //
        // 算法思路较为简单，实际还是有很多边界值要处理，所以最终用 Rust 模式匹配处理边界值：
        //      空数组、空数组：返回 0
        //      非空数组、空数组：返回 第一个数组内的 i、j
        //      空数组、非空数组：返回第二个数组内的 i，j
        //      非空数组、非空数组：设置两个起点标记两个数组的遍历 a_start、b_start
        //
        //      时间复杂度：O(N/2+1)
        //      空间复杂度：O(N/2+1)，N 数组总长度为，主要为寻找中位数是存下已排序的值
        let len = nums1.len() + nums2.len();
        let mut i = 0;
        let mut j = 0;

        if len % 2 == 0 {
            i = (len / 2) - 1;
            j = len / 2;
        } else {
            i = ((len + 1) / 2) - 1
        }


        return match (nums1.len(), nums2.len()) {
            (0, 0) => 0 as f64,
            (_, 0) => if len % 2 == 0 {
                ((nums1[i] as f64 + nums1[j] as f64) / 2f64) as f64
            } else {
                nums1[i] as f64
            },
            (0, _) => if len % 2 == 0 {
                ((nums2[i] as f64 + nums2[j] as f64) / 2f64) as f64
            } else {
                nums2[i] as f64
            },
            (_, _) => {
                let mut a_start = 0;
                let mut b_start = 0;
                let mut nums = Vec::with_capacity(len);
                loop {
                    if b_start >= nums2.len() || (a_start < nums1.len() && nums1[a_start] <= nums2[b_start]) {
                        nums.push(nums1[a_start]);
                        a_start += 1;
                    } else {
                        nums.push(nums2[b_start]);
                        b_start += 1;
                    }
                    if nums.len() - 1 >= max(i, j) {
                        break;
                    }
                }
                return if len % 2 == 0 {
                    ((nums[i] as f64 + nums[j] as f64) / 2f64) as f64
                } else {
                    nums[i] as f64
                };
            }
        };
    }
    pub fn find_median_sorted_arrays2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = Vec::with_capacity(nums1.len() + nums2.len());
        //nums1.as_ref().iter_mut().map(|_,x|(nums.push(x)));
        for n in nums1 {
            nums.push(n);
        }
        for n in nums2 {
            nums.push(n);
        }
        nums.sort();
        println!("{}", 111);
        return if nums.len() % 2 == 0 {
            ((nums[(nums.len() / 2) - 1] as f64 + nums[nums.len() / 2] as f64) / 2f64) as f64
        } else {
            nums[((nums.len() + 1) / 2) - 1] as f64
        };
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let chars: Vec<_> = s.chars().collect();
        // 算法思路：
        // 1、chars.len() < 2，返回
        // 2、分析1：回文字符串长度除了中心字符，一定四偶数
        // 3、分析2：回文字符串的中心字符串长度一定是 n 个相同的字符组成，n >=0
        // 3、分析3：回文字符串中心字符两边字符相等，表示为：
        //          中心字符串：s[i..j]
        //          中心字符串两边的字符：s[i-1]=s[j+1]
        // 4、算法：遍历字符串中每一个字符，把每个字符当作回文字符串的中心字符：


        let mut longest = 0;
        let mut start = 0;
        let mut end = 0;
        for it in 0..s.len() {
            if longest >= s.len() - 1 {
                break;
            }
            let mut i = it;
            let mut j = it;
            // 确定中心字符串 s[i..j]
            while j + 1 < s.len() && chars[i] == chars[j + 1] {
                j += 1;
            }
            // 判断 s[i-1]=s[j+1]
            while i > 0 && j + 1 < chars.len() && chars[i - 1] == chars[j + 1] {
                i -= 1;
                j += 1;
            }
            if j - i > longest {
                longest = j - i;
                start = i;
                end = j;
            }
        }

        (&s[start..end + 1]).to_string()
    }
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // 算法思路：
        // 最开始的想法是模拟一个二维数组来填充字符，想了好一会还是没有其它特别思路，看了官方的题解，理解了思路，用 Rust 写一遍。
        // a  *  *  g  *  *  m
        // b  *  f  h  *  l  n
        // c  e  *  i  k  *  o
        // d  *  *  j  *  *  p
        // 用一个有序的字符串表示 Z 字型变换：r = 4
        // 1、观察得到：先向下遍历了 r 个字符，再右向上遍历了 r-2 个字符；再是向下 r 个字符，向右上 r -2 个字符
        // 2、推导得到：字形变换周期内字符数 t = r + r -2 = 2r -2
        //            字符串下标为 i
        //            字符串长度为 n
        //            单个周期列数 1 + r - 2 = r -1
        //            字形占的列数 c = n/t . (r -1)
        //            变形条件 if i mod t < r - 1 { 下一个向下 } else { 下一个右上 } ：例，i = 0..3 向下，i = 4..

        if num_rows <= 1 || s.len() as i32 <= num_rows {
            return s;
        }

        let t = 2 * num_rows - 2;
        let mut x = 0;
        let mut y = 0;

        let chars: Vec<_> = s.chars().collect();
        let mut array = Vec::with_capacity(s.len());

        for (i, v) in chars.iter().enumerate() {
            array.push((*v, x, y));
            if i as i32 % t < num_rows - 1 {
                // 向下
                x += 1;
            } else {
                // 右上
                x -= 1;
                y += 1;
            }
        }
        // sort
        array.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.2.cmp(&b.2)));
        // map
        array.iter().map(|v| v.0).collect()
    }

    pub fn convert_print(s: String, num_rows: i32) -> String {
        // 算法思路：
        // 最开始的想法是模拟一个二维数组来填充字符，想了好一会还是没有其它特别思路，看了官方的题解，理解了思路，用 Rust 写一遍。
        // a  *  *  g  *  *  m
        // b  *  f  h  *  l  n
        // c  e  *  i  k  *  o
        // d  *  *  j  *  *  p
        // 用一个有序的字符串表示 Z 字型变换：r = 4
        // 1、观察得到：先向下遍历了 r 个字符，再右向上遍历了 r-2 个字符；再是向下 r 个字符，向右上 r -2 个字符
        // 2、推导得到：字形变换周期内字符数 t = r + r -2 = 2r -2
        //            字符串下标为 i
        //            字符串长度为 n
        //            单个周期列数 1 + r - 2 = r -1
        //            字形占的列数 c = n/t . (r -1)
        //            变形条件 if i mod t < r - 1 { 下一个向下 } else { 下一个右上 } ：例，i = 0..3 向下，i = 4..

        let t = 2 * num_rows - 2;
        let c = ((s.len() as f64 / t as f64) * (num_rows - 1) as f64) as i32;
        let mut x = 0;
        let mut y = 0;

        let chars: Vec<_> = s.chars().collect();
        let mut array_map = HashMap::with_capacity(s.len());

        for (i, v) in chars.iter().enumerate() {
            array_map.insert((x, y), *v);
            if i as i32 % t < num_rows - 1 {
                // 向下
                x += 1;
            } else {
                // 右上
                x -= 1;
                y += 1;
            }
        }

        let mut ret = String::from("");
        for i in 0..num_rows {
            for j in 0..c {
                if array_map.contains_key(&(i, j)) {
                    print!("{} ", array_map.get(&(i, j)).unwrap());
                    ret.push(*array_map.get(&(i, j)).unwrap());
                    ret.push_str(" ");
                } else {
                    print!("* ");
                    ret.push_str("  ");
                }
            }
            ret.pop();
            ret.push_str("\n");
            println!();
        }

        println!("{}", ret);
        return ret;
    }
}

use std::thread;

fn thread_test() {
    let name = String::from("thread1");
    let t1 = thread::spawn(move || {
        for i in 0..10 {
            println!("{} i={}", name, i);
        }
    });
    t1.join().unwrap();
}

use std::sync::mpsc;

fn mpsc_test() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
        s.send("i am from thread inner");
    });

    println!("{}", r.recv().unwrap())
}


fn main() {
    mpsc_test();
    thread_test();
    let m = Solution::convert(String::from("A"), 1);
    assert_eq!("A".to_string(), m);
    let m = Solution::convert(String::from("PAYPALISHIRING"), 3);
    assert_eq!("PAHNAPLSIIGYIR".to_string(), m);
    let m = Solution::convert(String::from("abcdefghijklmnopqrstuvwxyz"), 3);
    assert_eq!("bab".to_string(), m);

    let m = Solution::longest_palindrome(String::from("babad"));
    assert_eq!("bab".to_string(), m);
    let m = Solution::longest_palindrome(String::from("b"));
    assert_eq!("b".to_string(), m);

    let m = Solution::longest_palindrome(String::from("bb"));
    assert_eq!("bb".to_string(), m);


    let m = Solution::find_median_sorted_arrays(vec![100001], vec![100000]);
    assert_eq!(m, 100000.5f64);
    let m = Solution::find_median_sorted_arrays(vec![100000], vec![100001]);
    assert_eq!(m, 100000.5f64);
    let m = Solution::find_median_sorted_arrays(vec![], vec![2, 3]);
    assert_eq!(m, 2.5f64);
    let m = Solution::find_median_sorted_arrays(vec![2], vec![]);
    assert_eq!(m, 2f64);
    let m = Solution::find_median_sorted_arrays(vec![], vec![2]);
    assert_eq!(m, 2f64);
    let m = Solution::find_median_sorted_arrays(vec![2], vec![0, 0]);
    assert_eq!(m, 0f64);
    let m = Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]);
    assert_eq!(m, 0f64);
    let m = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
    assert_eq!(m, 2f64);

    let m = Solution::find_median_sorted_arrays(vec![1, 4, 5], vec![3, 7, 8]);
    assert_eq!(m, 4.5);

    let m = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
    assert_eq!(m, 2.5f64);

    let s = Solution::length_of_longest_substring(String::from("aaaaabc"));
    assert_eq!(s, 3);
    let s = Solution::length_of_longest_substring(String::from("abcadbcbb"));
    assert_eq!(s, 4);

    let s = Solution::length_of_longest_substring(String::from("bbbbb"));
    assert_eq!(s, 1);

    let s = Solution::length_of_longest_substring(String::from("pwwkew"));
    assert_eq!(s, 3);

    let s = Solution::length_of_longest_substring(String::from("dvdf"));
    assert_eq!(s, 3);

    let s = Solution::length_of_longest_substring(String::from("anviaj"));
    assert_eq!(s, 5);

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

    let nums = vec![3];
    let target = 6;
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
    iters();

    let mut map = HashMap::with_capacity(3);
    map.insert(1, 7);
    map.insert(2, 2);

    let v = Some(8i32);
    let kk = 1;

    if let Some(k) = map.get(&kk) {
        if *k == 7 {
            println!("yes")
        }
        println!("{}", *k);
        println!("{}", k);
        println!("s")
    }


    //输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    //输出：[8,9,9,9,0,0,0,1]

    let n1 = ListNode::new(9);
    let n1 = ListNode { val: 8, next: Some(Box::new(n1)) };
    let n1 = ListNode { val: 7, next: Some(Box::new(n1)) };
    let n1 = ListNode { val: 6, next: Some(Box::new(n1)) };
    let n1 = ListNode { val: 5, next: Some(Box::new(n1)) };
    let n1 = ListNode { val: 4, next: Some(Box::new(n1)) };
    let n1 = ListNode { val: 3, next: Some(Box::new(n1)) };
    let n1 = ListNode { val: 2, next: Some(Box::new(n1)) };
    let mut l1 = Some(Box::new(n1));


    // 1->2->2->4->3->5
    let n = ListNode::new(9);
    let n = ListNode { val: 9, next: Some(Box::new(n)) };
    let n = ListNode { val: 9, next: Some(Box::new(n)) };
    let n = ListNode { val: 9, next: Some(Box::new(n)) };
    let l2 = Some(Box::new(n));


    Solution::add_two_numbers(l1, l2);
}



/* ANCHOR_END: all */