#[derive(Debug)]
enum IpAddrKindOpt {
    V4(Option<(u32, u32, u32, u32)>),
    V6(Option<(u32, u32, u32, u32)>),
}

#[derive(Debug)]
struct IpAddrOpt {
    kind: IpAddrKindOpt,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn is_v4(&self) -> bool {
        match self.kind {
            IpAddrKind::V4 => true,
            IpAddrKind::V6 => false,
        }
    }
    fn is_v6(&self) -> bool {
        match self.kind {
            IpAddrKind::V4 => false,
            IpAddrKind::V6 => true,
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("lucky penny");
                return 1;
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => {
                println!("lucky quarter");
                return 25;
            },
        }
    }
}

fn main() {
    let v4 = IpAddrKindOpt::V4(Some((1, 2, 3, 4)));
    let v6 = IpAddrKindOpt::V6(Some((5, 6, 7, 8)));
    let v4_loopback_opt = IpAddrOpt {
        kind: v4,
    };
    let v6_loopback_opt = IpAddrOpt {
        kind: v6,
    };
    println!("v4 = {:#?}, v6 = {:#?}", v4_loopback_opt, v6_loopback_opt);
    let v4_loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let v6_loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("v4 = {:#?}, v6 = {:#?}", v4_loopback, v6_loopback);
    println!("v4_loopback is v4 {}", v4_loopback.is_v4());
    println!("v4_looopback is v6 {}", v4_loopback.is_v6());
    println!("v6_loopback is v4 {}", v6_loopback.is_v4());
    println!("v6_looopback is v6 {}", v6_loopback.is_v6());

    println!(
        "penny value {}, nickel value {}, dime value {}, quarter value {}", 
        Coin::Penny.value_in_cents(), Coin::Nickel.value_in_cents(),
        Coin::Dime.value_in_cents(), Coin::Quarter.value_in_cents(),
    );

}
