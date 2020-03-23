// explicitly opt in to make debug trait available
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
}

fn main() {
    let rect1 = Rectangle { width: 100, height: 125 };
    println!(
        "the area of the rectangle is {}",
        rect1.area()
    )
}
