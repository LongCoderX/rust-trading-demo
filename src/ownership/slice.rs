

pub fn slice_function() {
    println!("\n切片操作");

    // 不使用切片，需要重新申请空间，然后拷贝过来
    let s = String::from("Hello World");
    let ret = first_word_no_slice(&s);
    println!("No slice: Origin string: {s}, First word: {ret}");

    // 使用切片获取字符串
    let ret: &str = first_word_with_slice(&s);
    println!("Slice: Origin string: {s}, First word: {ret}");
}

fn first_word_no_slice(s: &String) -> String {
    let mut result: String = String::new();

    for item in s.chars() {
        if item == ' ' {
            break;
        }
        result.push(item);
    }

    return result;
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
