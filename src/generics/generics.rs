///


fn largest_element(v: &Vec<i32>) -> &i32 {
    let mut largest = &v[0];
    for number in v.iter() {
        if largest < number {
            largest = number;
        }
    }
    return largest;
}

pub fn print_generics_info() {
    println!("\n泛型相关内容");

    let v = vec![1, 2, 44, 1, 4, 56, 7342];
    let largest = largest_element(&v);
    println!("The largest element of {v:?} is {largest}");

}

