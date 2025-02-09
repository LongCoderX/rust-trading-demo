/// 引用与借用
/// https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html
/// 将创建一个引用的行为称为 借用（borrowing）。引用则是 &，解引用是 *
/// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
/// 引用必须总是有效的。


pub fn func_arg_reference() {
    println!("所有权：函数传值引用\n");

    // 函数通过借用变量值的方式，从而不获取值的所有权，所有权依旧在变量那边
    let s = String::from("func_args_reference");
    println!("Before transforming, the s is {s}");
    let s_len = calculate_str_length(&s);
    println!("After transforming, the s is {s}");
    println!("After transforming, the length of s is {s_len}");

    // 如果要在函数的作用域，通过引用的方式，修改原有的值，需要设置为 &mut 可变引用
    let mut s1 = String::from("func_args_reference");
    println!("Before transforming, the s1 is {s1}");
    modify_mutable_str(&mut s1);
    println!("After transforming, the s1 is {s1}\n");

    // 可变引用存在限制：
    // 同时只能存在最多一个可变引用，好处在于避免数据竞争。
    // 数据竞争(data race):
    // 1. 两个或者更多指针同时访问同一个数据
    // 2. 至少有一个指针用来写入数据
    // 3. 没有同步数据访问的机制
    let mut s2 = String::from("Hello");
    let r1 = &mut s2;
    r1.push_str(" r1 str");
    println!("r1: {r1}");
    {
        let r2 = &mut s2;
        r2.push_str(" r2 str");
        println!("r2: {r2}");
    }
    // 下一句会编译失败，因为 r2 已经在子作用域拥有一个可变的引用
    // println!("r1: {r1}");
    let r3 = &mut s2;
    r3.push_str(" r3 str");
    println!("r3: {r3}");

    // 可变与不可变引用，同样存在上述规则
    let mut s3 = String::from("Hello s3");
    s3.push_str(" s3");
    println!("\ns3 -> {s3}");
    let r1 = &s3;
    let r2 = &s3;
    // let r3 = &mut s3;
    println!("r1 {r1}");
    println!("r2 {r2}");
    // 声明可变引用r3的时候，也会报错。因为可变引用在同时私用
    // println!("r3 {r3}");

}

fn calculate_str_length(s: &String) -> usize {
    return s.len();
}

fn modify_mutable_str(s: &mut String) {
    s.push_str(" -append modifystr");
}