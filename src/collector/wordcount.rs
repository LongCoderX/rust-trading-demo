use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<&str, u32> {
    let mut ret = HashMap::new();
    
    for word in text.split(" ") {
        if let Some(v) = ret.get_mut(&word) {
            *v += 1;
        } else {
            ret.insert(word, 1);
        }
    }

    return ret;
}

fn word_count_1(text: &str) -> HashMap<&str, u32> {
    let mut ret = HashMap::new();
    for word in text.split_whitespace() {
        let count = ret.entry(word).or_insert(0);
        *count += 1;
    }
    return ret;
}

pub fn test_word_count() {
    println!("\n============== No.1 word count ================");

    let text = "Hello world Hello world Ok are you ok";

    let ret = word_count(text);
    println!("{:?}", ret);
    
    let ret = word_count_1(text);
    println!("{:?}", ret);
}
