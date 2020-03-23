fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word = {}", word);
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello = {}, world = {}", hello, world);
    let len = s.len();
    let full = &s[0..len];
    println!("full = {}", full);
}


fn first_word(st: &String) -> usize {
    let bytes = st.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("i = {}, item = {}", i, item);
            return i;
        }
    }
    return st.len();
}