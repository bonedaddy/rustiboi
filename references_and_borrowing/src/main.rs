fn main() {
    let mut s = String::from("hello world");
    let size = get_length(&mut s);
    println!("string {}, size {}", s, size);
}

fn get_length(st: &mut String) -> usize {
    st.push_str("this is a test!!1");
    return st.len();
}