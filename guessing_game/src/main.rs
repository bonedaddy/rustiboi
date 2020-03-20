use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("input your guess");
        // creates a mutable variable
        let mut guess = String::new();
        // like variables references are immutable by default
        // thus we need to make ita Mutable reference (&mut guess)
        // as opposed to &guess which would be an immutable reference
        stdin().read_line(&mut guess).expect("failed to read line");
        // this is called shadowing, and allows to convert a previous value to a new type
        // trim method on string removes any whitespace
        // parse converts string into any type of number, inferred from the
        // left hand side of the assignment
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("invalid input {}", err);
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("guess too small"),
            Ordering::Equal => {
                println!("you won");
                return;
            },
            Ordering::Greater => println!("guess to large"),
        }
    }
}
