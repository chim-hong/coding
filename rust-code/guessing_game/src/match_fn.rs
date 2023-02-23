#[derive(Debug)]
enum State {
    Zhejiang,
    Beijing,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}
fn match_fn(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => {
            println!("Penny");
            5
        }
        Coin::Dime => {
            println!("Penny");
            10
        }
        Coin::Quarter(state) => {
            println!("Quarter {:#?}", state);
            20
        }
        _ => {
            println!("==========");
            0
        }
    }
}

pub fn main() {
    println!("{}", match_fn(Coin::Quarter(State::Zhejiang)));
    println!("{}", match_fn(Coin::Nickel));

    let v = 2u8;

    let result = match v {
        1=>1,
        2=>3,
        3=>5,
        _=>0
    };

    println!("result: {},{}",v,result);
}
