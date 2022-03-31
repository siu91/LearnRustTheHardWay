IDEA / CLion 开发快捷键

对于编码场景来说，开发人员的双手不离开键盘区域是能对于开发来说使用快捷键能提高很大的效率，所以在最佳实践部分，首先是总结这部分。会按照场景来整理。

> 以下针对 MacOS

## 场景 1：选择并复制多行代码

- 1、使用方向键盘、`home`、`end` 调整光标的起始位置
- 2、按住 `shift` 用方向键选择范围
- 3、复制

使用 `option`+`方向上下键` 可以按照代码块的方式去选中代码

## 场景 2：查找和替换

- Double Shift 查询任何东西
- Command+Shift+F 全局查找
- Command+F 文件内查找
- Command+G 查找模式下，向下查找
- Command+Shift+G 查找模式下，向上查找
- Command+R 文件内替换
- Command+Shift+R 全局替换
- Command+O 查找类文件
- Command+Shift+O 查找所类型文件

## 场景 3：代码编辑

- Command+左右方向 跳到行首/行尾
- Command+/ 行注释（再按一下取消注释）
- Command+Option+/ 块注释（再按一下取消注释）
- Command+D+/ 复制当前行到下一行
- Command+Delete+/ 删除当前行
- option+shift+[CLIKC] 任意位置插入光标（比如可以用于多行同时输入）
- Control+Shift+J 将光标后代码拼接到当前行
- Command+Shift+V 粘贴板历史
- Tab 缩进代码
- Shift+Tab 反缩进代码
- Control+Option+O 优化import
- Command+Option+L 格式化代码
- Command+N 生成代码（需要设置样板代码，默认可以添加头注释）
- Control+O 覆盖方法(重写父类方法)
- Control+I 实现方法(实现接口中的方法)
- Command+Option+T 包围代码(使用if...else等包围选中的代码)

## 场景 4：阅读代码

- Command+[CLICK] 跳转到代码声明
- Command+Option+向左箭头/向右箭头 退回/前进到上一个位置
- Command+L 在当前文件跳转到某一行的指定处
- Command+E 显示最近打开的文件记录列表
- fn+Command+F12 打开当前文件结构层

## 场景 5：提交代码

- Command+K 提交代码
- Command+T 更新代码
- Option+Shift+C 查看最近的变更记录

## 场景 6：Debugging

| No.  | 快捷键              | 描述                                                         |
| :--- | :------------------ | :----------------------------------------------------------- |
| 1    | fn+F8               | 进入下一步，如果当前行断点是一个方法，则不进入当前方法体内   |
| 2    | fn+F7               | 进入下一步，如果当前行断点是一个方法，则进入当前方法体内，如果该方法体还有方法，则不会进入该内嵌的方法中 |
| 3    | fn+Shift+F8         | 跳出                                                         |
| 4    | fn+Command+F8       | 增加/取消断点                                                |
| 5    | fn+Command+Shift+F8 | 查看断点信息                                                 |

## 场景 7：编译和运行

------

| No.  | 快捷键           | 描述                |
| :--- | :--------------- | :------------------ |
| 1    | fn+Command+F9    | 编译项目            |
| 3    | Control+Option+R | 弹出Run的可选择菜单 |
| 5    | Control+R        | 运行                |
| 6    | Control+D        | 调试                |

