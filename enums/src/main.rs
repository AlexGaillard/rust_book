#[derive(Debug)]

enum UsState {
    Alabama,
    Arkansas,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alabama));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("A shiny new penny!");
            1
        }
        Coin::Nickel => {
            println!("A nickel for your thoughts!");
            5
        }
        Coin::Dime => {
            println!("One dime out of a dozen!");
            10
        }
        Coin::Quarter(state) => {
            println!("Oh good a quarter from {:?}!", state);
            25
        }
    }
}
