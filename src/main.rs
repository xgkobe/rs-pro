use std::fmt::Result;
use std::io::Result as IoResult;
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 6,
        Coin::Dime => 25,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            23
        }
    }
}
fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));
    let_if();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// _下划线通配符
fn match_() -> i8{
    let v = 0u8;
    match v {
        0 => 0,
        1 => 1,
        _ => 2,
    }
}

// if let 处理一种匹配而忽略其它匹配的情况， 可以把if let看作是match的语法糖
fn let_if() {
    let v = Some(0);
    if let Some(0) = v {
        println!("{:#?}", Some(3));
    } else {
        println!("{:#?}", Some(0u8));
    }
}