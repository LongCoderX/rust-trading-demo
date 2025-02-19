
#[derive(Debug, Clone)]
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

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        if self.width == other.width && self.height == other.height {
            return true;
        }
        return false;
    }

    fn ne(&self, other: &Self) -> bool {
        if self == other {
            return false;
        }
        return true;
    }
}

pub fn calculate_rect() {
    // 结构体方法
    println!("============== No.1 Rectangle ==============");

    let rect1 = Rectangle {
        width: 30,
        height: 30
    };
    println!("\nThe area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle::square(33);
    println!("\nThe area of the rectangle is {} square pixels.", rect2.area());

    if rect1 != rect2 {
        println!("The rect1 {:?} is ne rect2 {:?}", rect1, rect2);
    }

    let rect3 = rect1.clone();
    if rect1 == rect3 {
        println!("The rect1 {:?} is eq rect3 {:?}", rect1, rect3);
    }
}

