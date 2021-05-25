#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}
// 枚举可以实现struct的组合
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u32, y:u32 },
    Write(String),
    ChangeColor(u8, u8, u8)
}

impl Message {
    fn call(&self) {
        println!("self is {:#?}", self)
    }
}
// enum Option<T> {
//     Some(T),
//     None
// }
#[derive(Debug)]
enum usState {
    Alabama,
    Alaska
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("the state is {:?}", state);
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}
// 其他场景使用 _ 通配符
fn some_u8_value(val: u8) {
    match val {
        1 => println!("one"),
        _ => (),
    }
}
// 只有一个pattern时 可以使用if let
// Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
fn some_u8_value_if_let(val: u8) {
    if let 1 = val {
        println!("one")
    } else {

    }
}

fn main() {
    let six = IPAddrKind::V6(String::from("::1"));
    let four = IPAddrKind::V4(127, 0, 0, 1);
    println!("Hello, {:?}", six);
    println!("Hello, {:?}", four);
    let msg = Message::Write(String::from("你好"));
    // we’ve created a variable m that has the value Message::Write(String::from("hello")),
    // and that is what self will be in the body of the call method when m.call() runs.
    msg.call();
    let some_one = Some("liuwei");
    println!("some_one, {:?}", some_one);
    // let absent_number: Option<i32> = None;
    let coin = value_in_cents(Coin::Quarter(usState::Alabama));
    println!("coin, {:?}", coin);
    let five = Some(5);
    let six = plus_one(five);
    let y: Option<i32> = None;
    assert_eq!(y.xor(six), Some(6));
    let none = plus_one(None);
}
