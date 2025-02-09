/// 所有权的学习
/// https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html

pub fn ownership() {
    println!("rust 所有权学习\n");
    
    // 简单的类型，内存分配在栈上
    // 本质而言是实现了 Copy trait 注解
    // Copy trait 与 Drop trait 是矛盾的，在实现Drop后，是无法使用Copy的
    let x = 5;
    println!("x = {x}");

    let y = x;
    println!("x = {x}");
    println!("y = {y}");
    
    // 复杂的类型，比如String类型，分配在堆上面
    let str = String::from("Hello rust!");
    println!("{str}");
    let str1 = str;
    // 此处编译的时候，会报错，因为已经将值的所有权在 let str1 = str; 的时候，已经转移。
    // 类似 C++ 中的语义移动，但是C++是针对将亡值进行的移动。
    // println!("{str}");
    println!("{str1}");

    // 使用clone的方式，可以进行“深拷贝”
    let str = str1.clone();
    println!("{str}");
}

pub fn function_ownership() {
    let s = String::from("function and ownership");
    println!("\nBefore transfrom 's' to function: {s}");
    takes_ownership(s);
    // 后一句在编译的时候，会报错。由于 s 的值已经传递给函数，所以无法再外部的生命周期中使用。
    // println!("After transfrom 's' to function: {s}");

    let x = 5;
    println!("\nBefore transfrom 'x' to function: {x}");
    makes_copy(x);
    println!("After transfrom 'x' to function: {x}")
}

/// 所有权与函数
/// 向函数传递值可能会移动或者复制，就像赋值语句一样。
fn takes_ownership(str: String) {
    println!("所有权与函数\n");
    println!("In takes_ownership: {str}");
}

fn makes_copy(num: i32) {
    println!("In makes_copy: {num}");
}

/// 返回值与作用域
/// 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
/// 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
/// % 相当于在赋值的时候，可以从父作用域转移到子作用域，然后也可以转出来。也可以随着子作用域消亡。
pub fn return_val_ownership() {
    println!("返回值与作用域\n");

    let s1 = gives_ownership();
    println!("return_val_ownership s1: {s1}");
    
    let s2 = String::from("return_val_ownership s2");
    println!("Before transfrom in takes_and_gives_back. return_val_ownership s2: {s2}");
    let s3 = takes_and_gives_back(s2);
    // 下一句无法编译，因为已经作为参数传递给函数
    // println!("After transfrom in takes_and_gives_back. return_val_ownership s2: {s2}");
    println!("After transfrom in takes_and_gives_back. return_val_ownership s3: {s3}");
}

fn gives_ownership() -> String {
    let s = String::from("gives_ownership");
    return s;
}

fn takes_and_gives_back(s: String) -> String {
    return s;
}
