fn main() {
    let s = "world";
    {
        let s = "hello";
        print!("{} ", s);
    }
    println!("{}", s);
    let mut s = String::from("hello world");
    println!("{}", s);
    s.push_str(" this is a test");
    println!("{}", s);
    s = "lolwut".to_string();
    println!("{}", s);
    drop(s);
    let mut x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);
    x = 1;
    println!("{}", x);
    println!("{}", y);
    let s1 = String::from("hello world");
    let mut s2 = s1;
    s2.push_str("this is a test");
    println!("{}", s2);
    let mut s3 = String::from("hello world");
    let s4 = s3.clone();
    s3.push_str(" this isa test");
    println!("s3 = {}, s4 = {}", s3, s4);
    let s10 = gives_ownership();
    let s11 = String::from("hello world");
    let s10OwnerChanges = takes_and_gives_back(s10);
    println!("s10 {}", s10OwnerChanges);
    let (s, size) = get_length(String::from("hello world"));
    println!("string: {}, size: {}", s, size);
}


fn gives_ownership() -> String {
    let some_string = String::from("hello world");
    return some_string;
}

fn takes_and_gives_back(st: String) -> String {
    return st;
} 

fn get_length(st: String) -> (String, usize) {
    let length = st.len();
    return (st, length);
}