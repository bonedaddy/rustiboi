fn main() {
    let mut s = String::from("hello world");
    let size = s.len();
    println!("string {}, size {}", s, size);
    let old_size = get_length_and_push(&mut s);
    println!("string {}, size {}", s, old_size);
    let mut ss = String::from("hello world");
    {
        let r1 = &mut ss;
        println!("{}", r1);
    }
    let r2 = &mut ss;
    println!("{}", r2);
}

fn get_length_and_push(st: &mut String) -> usize {
    st.push_str("this is a test!!1");
    return st.len();
}