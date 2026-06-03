# Enums and Pattern Matching

We'll look at enumerations, also referred as *enums*. Enums allow you to define a type by enumerating its possible variants. First we'll define and use an enum to show how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing. Then, we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum. Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.

# Defining an Enum

Enums give you a way of saying a value is one of a possible set of values.

## Enum Values

Let's say we created an enum for a kind of Ip Address:

```Rust
enum IpAddrKind {
    V4,
    V6,
}
```

So, we can create instances of each of the two variants like this:

```Rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

We can also put data directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:

```Rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum! Also, standard library types are often not much more complicated than what you might come up with.

There is one more similarity between enums and structs: Just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.

## The Option Enum

The Option type encodes the very common scenario in which a value could be something, or it could be nothing.

Check the section on rustbook because it is very deep about the Option enum: [section](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum)

