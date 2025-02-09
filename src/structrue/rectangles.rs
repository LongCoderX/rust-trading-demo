
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 成员方法
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    // 关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn calculate_rect() {
    // 结构体方法
    println!("结构体方法");

    let rect1 = Rectangle {
        width: 30,
        height: 30
    };
    println!("\nThe area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle::square(33);
    println!("\nThe area of the rectangle is {} square pixels.", rect2.area());
}

