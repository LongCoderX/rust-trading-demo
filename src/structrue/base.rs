/// 结构体学习

// 结构体定义
// struct 结构体名称 {
//      字段1：字段类型,
//      字段2：字段类型,
//      字段3：字段类型,
// }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// 结构体实例
pub fn print_struct() {
    println!("\n结构体练习");

    // 创建实例
    let mut user1 = User {
        active: true,
        username: String::from("LongCoderX"),
        email: String::from("skrlong1997@gmail.com"),
        sign_in_count: 1,
    };

    println!("active:{0}\nusername:{1}\nemail:{2}\nsign_in_count:{3}\n", 
        user1.active, user1.username, user1.email, user1.sign_in_count);
    user1.email = String::from("ggggg@gmail.com");
    println!("active:{0}\nusername:{1}\nemail:{2}\nsign_in_count:{3}\n", 
    user1.active, user1.username, user1.email, user1.sign_in_count);

    // 实例化：参数名与字段名都完全相同时，字段初始化简写语法（field init shorthand）
    let user1 = build_user(String::from("xxxx@gmail.com"), 
        String::from("LongCoderX"));
    println!("active:{0}\nusername:{1}\nemail:{2}\nsign_in_count:{3}\n", 
        user1.active, user1.username, user1.email, user1.sign_in_count); 

    // 实例化：使用其他实例填充新实例, `..instance` 必须放在最后。
    let user1 = User {
        email: String::from("yyyyy@gmail.com"),
        ..user1
    };
    println!("active:{0}\nusername:{1}\nemail:{2}\nsign_in_count:{3}\n", 
        user1.active, user1.username, user1.email, user1.sign_in_count); 

    
}

fn build_user(email: String, username: String) -> User {
    // 字段初始化简写语法（field init shorthand）, 只要字段名相同，顺序不一样也可以初始化。
    return User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    };
}