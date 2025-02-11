use std::collections::HashMap;

pub fn print_hashmap_collector() {
    println!("\nHashMap 实践");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score is {score}");

    scores.entry(String::from("Green")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(90);
    println!("{scores:?}");

    // 遍历
    for (key, value) in scores {
        println!("{key}, {value}");
    }

    // 计数
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

}
  