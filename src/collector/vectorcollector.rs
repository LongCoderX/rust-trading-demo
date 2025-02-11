/// Vec容器的实验

pub fn print_vector_collector() {
    println!("\n常见的集合容器实验");

    vector_instance();
    string_instance();
}

fn vector_instance() {
    // 使用 new 函数创建空的vec集合，最好是mut类型，因为需要填充数值
    let mut v: Vec<i32> = Vec::new();
    // 更新vec内容
    v.push(12);
    v.push(13);
    println!("{:?}", v);
    
    // 使用 vec! 宏实例化并初始化 Vec<i32> 类型
    let v = vec![1, 2, 3];
    println!("v: {:?}", v);

    // 访问 vec 的内容
    // 使用 [] 访问
    // 使用 get 函数访问，并且需要使用 unwrap 进行错误处理
    println!("v: {0}, {1}", v[0], v.get(2).unwrap());

    let third = v.get(3);
    match third {
        Some(third) => println!("v.get(2): {}", third),
        None => println!("There is not third element."),
    }

    let mut v = vec![1, 2, 3, 4];
    v.push(22);
    let first = &v[0];
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    // v.push(22);
    println!("The first element is: {first}");

    // 遍历vec
    for i in &v {
        print!("{i}");
    }
    println!();

    for i in &mut v {
        // 解引用
        *i += 20;
        println!("{i}");
    }
    println!();
}

fn string_instance() {
    println!("\nString本质是封装的Vec");

    // 在C++ String的类型更像是Vec的模板偏特化
    let mut s = String::new();
    s.push('1');
    s.push('2');
    s.push('3');
    // push 传入参数类型为 slice str，即 &str
    s.push_str(" LongCoderX");
    let s2 = " GGBoy";
    s += &s2;
    println!("{s}");
    s += s2;
    println!("{s}");
    println!("{s2}");

    let s1 = String::from("\nHello ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{s3}");
    // 编译报错，因为 s1 被移动了，而且必须添加 &s2
    // println!("{s1}");
    println!("{s2}");
    
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");
    let s = "initial contents".to_string();
    println!("{s}");
    let s = String::from("initial contents");
    println!("{s}");
    
    println!();
    let s = String::from("السلام عليكم");
    println!("{s}");
    let s = String::from("Dobrý den");
    println!("{s}");
    let s = String::from("Hello");
    println!("{s}");
    let s = String::from("שלום");
    println!("{s}");
    let s = String::from("नमस्ते");
    println!("{s}");
    let s = String::from("こんにちは");
    println!("{s}");
    let s = String::from("안녕하세요");
    println!("{s}");
    let s = String::from("你好");
    println!("{s}");
    let s = String::from("Olá");
    println!("{s}");
    let s = String::from("Здравствуйте");
    println!("{s}");
    let s = String::from("Hola");
    println!("{s}");

    // 遍历方法
    let s = "Зд";

    // C/C++ 中的宽字符，占用2个字节
    println!();
    for c in s.chars() {
        println!("{c}");
    }

    // byte 会打印字符数字，而且是4个，因为使用的Unicode编码，单字符2字节
    println!();
    for b in s.bytes() {
        println!("{b}");
    }

    // 编译报错 &str 不是一个 iterator
    // for i in s {
    //     println!("{}", *i);
    // }
}