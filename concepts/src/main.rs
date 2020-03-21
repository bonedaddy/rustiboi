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
    let val = print_number(1000);
    println!("{}", val);
    if val != 1000 {
        panic!("bad number man");
    }
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("{:?}", number);
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, number];
    for element in arr.iter() {
        println!("element value is {:?}", element);
    }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        } 
    };
    println!("the result is {}", result);
    let mut number = 10;
    while number > 0 {
        println!("number {}", number);
        number -= 1;
    }
    for n in (1..100).rev() {
        println!("{}", n);
    }
}


fn print_number(n: i32) -> i32{
    println!("{}", n);
    // most functions return thel ast value implicitly
    // but explicit is better than implicit
    return n;
}