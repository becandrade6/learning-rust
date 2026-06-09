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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // --snip--
        }
    }
}

fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    // `if let` is a one-arm match: run the block if the pattern fits, otherwise run the else.
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        println!("Not a quarter.");
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // if `coin` is a Quarter, unpack its state and describe it; otherwise return None.
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_2(coin: Coin) -> Option<String> {
    // Same as version 1, but `state` is extracted into its own variable so it can be
    // used below the if-let block without extra nesting. The early `return None` bails out
    // of the whole function when the pattern doesn't match.
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_3(coin: Coin) -> Option<String> {
    // `let-else` is the clean version of the pattern above: if the pattern matches,
    // `state` is bound directly in the outer scope (no nesting needed). If it doesn't
    // match, the else arm *must* diverge — here it returns None. After this line,
    // the rest of the function can treat `state` as a plain variable.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
