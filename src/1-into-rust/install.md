# 安装 Rust 环境
> Mac 上安装 Rust 方式可以选择 brew 和官方安装方式，这里推荐[官方的安装方法](https://www.rust-lang.org/tools/install)

## 安装
```shell
siu@localhost ~ % curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````
## 检查是否安装成功
```shell
siu@localhost ~ % rustc -V
rustc 1.59.0 (9d1b2106e 2022-02-23)
siu@localhost ~ % cargo -V
cargo 1.59.0 (49d8809dc 2022-02-10)
````
## 卸载
```shell
siu@localhost ~ % rustup self uninstall
````

