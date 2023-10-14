enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn sub_main() {
    println!("=========== Enums =========================");

    let coin_value = value_in_cents(Coin::Penny);
    println!("Value is {coin_value}");

    let some_coins: Option<Coin> = Some(Coin::Nickel);
    match some_coins {
        Some(Coin::Nickel) => println!("It is Nickel"),
        Some(Coin::Dime) => println!("It is Dime"),
        _ => print!("Something else"),
    };

    println!("================================================");
}
