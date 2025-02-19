# rust-trading-demo

## 学习目标
- 理解rust的基本语法
- 学习如何应用rust编写高性能代码

## 资料
[Rust官方文档](https://www.rust-lang.org/zh-CN/learn)

## 目录

```
src
|-- collector
|   |-- hashmapcollector.rs
|   |-- mod.rs
|   `-- vectorcollector.rs
|-- errorhandler
|   |-- mod.rs
|   `-- resulthandler.rs
|-- generics
|   |-- generics.rs
|   `-- mod.rs
|-- main.rs
|-- ownership
|   |-- base.rs
|   |-- mod.rs
|   |-- reference.rs
|   `-- slice.rs
`-- structrue
    |-- base.rs
    |-- mod.rs
    |-- optionandresult.rs
    |-- rectangles.rs
    |-- user.rs
    `-- webevent.rs
```

- `main.rs` 作为调用其他模块的主函数。
- `collector` 集合结构体学习
    - `vectorcollector.rs` 学习 Vec 和 String 结构体
    - `hashmapcollector.rs` 学习 HashMap 结构体
- `errorhandler` 学习错误处理以及自定义错误处理
    - `resulthandler` 错误处理的一些问题实践
- `generics` 学习模板编程
    - `generics` 函数模板编程
- `ownership` 学习所有权相关的知识。
    - `base.rs` rust的变量所有权。在变量赋值的时候，值的所有权会转移。比如在函数传参场景、函数返回值的场景。其中的所有权的转移的变化。
    - `reference.rs` rust 所有权的引用与借用
    - `slice.rs` rust 字符串中的切片操作
- `structrue` 结构体
    - `base.rs` 结构体的定义语法以及实例方法
    - `optionandresult.rs` 枚举 Option 以及 Result 的应用，在错误处理上面。同时，调用的时候，使用if let语法糖处理Option的返回值
    - `rectangle.rs` 结构体方法以及关联函数，也实现 PartialEq trait，用于比较矩形
    - `user.rs` 实现结构体 User 以及枚举类型 AccountStatus，作为一个字段在User中如何使用。
    - `webevent.rs` 实现枚举 WebEvent 以及通过match处理状态。

    