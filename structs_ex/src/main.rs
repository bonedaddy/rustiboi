// explicitly opt in to make debug trait available
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "the area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    println!("");
    let rect1: (i32, i32) = (30, 50);
    println!(
        "the area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
    let rect2 = Rectangle { width: 100, height: 50 };
    println!("");
    println!(
        "the area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );
    println!("rect2 debug display 1 = {:?}", rect2);
    println!("rect2 debug display 2 = {:#?}", rect2);

}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_tuple(dimensions: (i32, i32)) -> i32 {
    return dimensions.0 * dimensions.1;
}


fn area_struct(rectangle: &Rectangle) -> i32 {
    return rectangle.width * rectangle.height;
}