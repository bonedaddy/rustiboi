fn main() {
    let s = "world";
    {
        let s = "hello";
        println!("{}", s);
    }
    println!("{}", s);
}
