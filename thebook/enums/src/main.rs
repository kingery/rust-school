enum IpAddress {
    V4(String),
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("yo!");
    }
}

fn main() {
    let ip4 = IpAddress::V4(String::from("foo"));
    let ip6 = IpAddress::V6;

    route(ip4);
    route(ip6);

    let m = Message::Write(String::from("foo"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("foo");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(1);
    
    //let sum = x + y;

    let nickel = Coin::Nickel;
    println!("{}", value_in_cents(nickel));
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));

    //let config_max: Option<u8> = None;
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("the max is {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("(if let) the max is {}", max);
    }
}

fn route(ip: IpAddress) {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
