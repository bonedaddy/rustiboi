// explicitly opt in to make debug trait available
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn square(size: i32) -> Rectangle {
        return Rectangle { width: size, height: size };
    }
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            return true;
        }
        return false;
    }
    fn is_square(&self) -> bool {
        if self.width == self.height {
            return true;
        }
        return false;
    }
}

fn main() {
    let rect1 = Rectangle { width: 100, height: 125 };
    let rect2 = Rectangle { width: 50, height: 50};
    let rect3 = Rectangle { width: 1000, height: 1000};
    let rect4 = Rectangle::square(10);
    println!(
        "the area of the rectangle is {}",
        rect1.area()
    );
    println!("rect1 can hold rect2 {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 {}", rect1.can_hold(&rect3));
    println!("rect1 can hold rect4 {}", rect1.can_hold(&rect4));
    println!("rect1 is square {}", rect1.is_square());
    println!("rect4 is square {}", rect4.is_square());
}
