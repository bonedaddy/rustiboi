const NUMBER: u32 = 1000;

fn main() {
    let mut x = "hello";
    print!("{} ", x);
    x = "world";
    println!("{}", x);
    let y = "rust";
    print!("{} ", y);
    let y = "is";
    print!("{} ", y);
    let y = "cool";
    println!("{}", y);
    println!("{}", NUMBER);
    let z = 5;
    let z = z + 1;
    let z = z + 2;
    println!("final value of z is {}", z);
    let c = 'z';
    let zz = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c {}, z {}, cat {}", c, zz, heart_eyed_cat);
    let tup = (c, zz, heart_eyed_cat);
    println!("{:?}", tup);
    let ar = [c, zz, heart_eyed_cat];
    println!("{:?}", ar);
    // a 5 element i32 array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    print_number(1000);
}


fn print_number(n: i32) {
    println!("{}", n);
}