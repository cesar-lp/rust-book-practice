enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Coordinates {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move(Coordinates),
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn execute() {
    build_addresses();
    build_message();
    some_none();
    coins_example(Coin::Quarter(UsState::Alabama));
    plus_one(None);
    plus_one(Some(5));
    if_let(Coin::Dime);
}

fn build_addresses() {
    let _home = IpAddress::V4(127, 0, 0, 1);
    let _loopback = IpAddress::V6(String::from("::1"));
}

fn build_message() {
    let message = Message::Write(String::from("This is a message"));
    message.call();

    match message {
        Message::Quit => println!("Quiting application"),
        Message::Move(coordinates) => {
            println!("Moving to x: {}, y: {}", coordinates.x, coordinates.y)
        }
        _ => println!("Doing something else"),
    }
}

fn some_none() {
    let _some = Some(5);
    let _none: Option<i32> = None;
}

fn coins_example(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn if_let(coin: Coin) {
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
