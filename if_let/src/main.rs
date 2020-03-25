#[derive(Debug)]
enum Province {
    BritishColumbia,
    Alberta,
    Sask,
    Manitoba,
    Ontario,
    Quebec,
    NovaScotia,
    NewFoundLand,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Dollar(Province),
}

fn main() {
    let mut count = 0;
    let coin = Coin::Dollar(Province::BritishColumbia);
    match coin {
        Coin::Dollar(province) => println!("province: {:?}", province),
        _ => count += 1,
    }
    let coin = Coin::Dollar(Province::Alberta);
    match coin {
        Coin::Quarter => println!("quarter"),
        _ => count += 1,
    }
    println!("count {}", count);
    let some_u8_value: Option<u8> = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("iflet 3");
    }
    println!("Hello, world!");
}
