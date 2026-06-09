# Concise Control Flow with `if let` and `let...else`

## The problem `if let` solves

Sometimes you only care about **one specific variant** of an enum and want to ignore everything else. Using a full `match` for this feels like a lot of ceremony:

```rust
let config_max = Some(3u8);

match config_max {
    Some(max) => println!("The maximum is {max}"),
    _ => (), // do nothing — but we still have to write this line
}
```

That `_ => ()` line is just noise. We don't want to do anything if it's `None`, so why do we have to say so?

## `if let`: match on one pattern, skip the rest

`if let` lets you write the same thing more cleanly:

```rust
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("The maximum is {max}");
}
```

Read it as: **"if `config_max` matches `Some(max)`, then run this block (and bind the inner value to `max`)"**. If it doesn't match, nothing happens — no need to say so.

Think of `if let` as a `match` that only has one arm and silently ignores everything else.

> **Trade-off:** `match` forces you to handle every possible case, which helps you avoid bugs. `if let` is shorter, but you give up that safety net. Use `if let` when you genuinely don't care about the other cases.

## Adding an `else` branch

You can add an `else` to handle the "didn't match" case, just like `match`'s `_` catch-all:

```rust
let coin = Coin::Quarter(UsState::Alaska);

if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    println!("Not a quarter.");
}
```

This is exactly equivalent to:

```rust
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => println!("Not a quarter."),
}
```

Both do the same thing — it's just a matter of what reads more clearly for your situation.

---

## `let...else`: getting a value out or bailing early

Now consider a function that takes a `Coin` and returns a description — but only if it's a `Quarter`. For anything else, it returns `None`.

A naive approach nests the logic inside the `if let` block:

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
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
```

This works, but the "happy path" (the interesting logic) is buried inside indentation. The pattern **"extract a value or bail out early"** is common enough that Rust has dedicated syntax for it: `let...else`.

### `let...else` flips the structure

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None; // bail out if it's not a Quarter
    };

    // `state` is available here in the outer scope — no extra nesting!
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

Read `let...else` as: **"bind `state` from `coin` if the pattern matches — otherwise run the `else` block"**.

Two important rules about the `else` block:
1. It **must diverge** — meaning it has to `return`, `break`, `continue`, or `panic!`. It can't just fall through.
2. The bound variable (`state` here) is available in the **outer scope**, after the `let...else` line — not inside a nested block.

### Why this is better

The "bail out" case is handled right at the top, upfront. The rest of the function can then work with `state` directly, without any extra indentation or wrapping. The happy path stays flat and easy to read.

---

## Quick summary

| Syntax | Use when... |
|---|---|
| `match` | You need to handle multiple variants, or want exhaustive checking |
| `if let` | You only care about one variant and want shorter code |
| `if let` + `else` | One variant does something, all others do something else |
| `let...else` | You want to extract a value or bail out early, keeping the happy path flat |
