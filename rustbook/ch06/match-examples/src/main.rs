#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("A Penny is {} cents", value_in_cents(Coin::Penny));
    println!("A Nickel is {} cents", value_in_cents(Coin::Nickel));
    println!("A Dime is {} cents", value_in_cents(Coin::Dime));
    println!("A Quarter is {} cents", value_in_cents(Coin::Quarter(UsState::Alaska)));

    // Option enum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch-All Patterns and the _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    }

    // changing the rules, if you roll anything other than 3 or 7 you must reroll
    // now we use the placeholder '_' because we no longer need to use the catch-all value
    let dice_roll2 = 9;
    match dice_roll2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    //changing the rule again, if is different than 3 or 7 nothing else happens on the turn
    // we can express that using the unit value (the empty tuple) that goes with the catch-all
    let dice_roll3 = 9;
    match dice_roll3 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=> 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i)=> Some(i+1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}
