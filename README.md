# rust-trading-demo

## 学习目标
- 理解rust的基本语法
- 学习如何应用rust编写高性能代码

## 资料
[Rust官方文档](https://www.rust-lang.org/zh-CN/learn)

## 目录

```
src
├── main.rs
└── ownership
    ├── base.rs
    ├── mod.rs
    ├── reference.rs
    └── slice.rs
```

- `main.rs` 作为调用其他模块的主函数。
- `ownership` 学习所有权相关的知识。
    - `base.rs` rust的变量所有权。在变量赋值的时候，值的所有权会转移。比如在函数传参场景、函数返回值的场景。其中的所有权的转移的变化。
    - `reference.rs` rust 所有权的引用与借用
    - `slice.rs` rust 字符串中的切片操作