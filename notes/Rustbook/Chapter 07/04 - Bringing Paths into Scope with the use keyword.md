# Bringing Paths into Scope with the use Keyword

Having to write out the paths to call functions can feel inconvenient and repetitive. Fortunately, there's a way to simplify this process: We can create a shortcut to a path with the `use` keyword once and then use the shorter name everywhere else in the scope.

Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the create root, `hosting` is now a valid name in that scope, just as though the `hosting` module had been defined in the crate root. Paths brought into scope with `use` also check privacy, like any other paths.

Note that `use` only creates the shortcut for the particular scope in which the `use` occurs.

# Creating Idiomatic use Paths

Bringing the function's parent module into scope with `use` means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn't locally defined while still minimizing repetition of the full path.

On the other hand, when bringing in structs, enums, and other items with `use`, it's idiomatic to specify the full path. Like the example below shows the idiomatic way to bring the standard library's `HashMap` struct into the scope of a binary crate.

```Rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

There's no strong reason behind this idiom: It's just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

The exception to this idiom is if we're bringing two items with the same name into scope with `use` statements, because Rust doesn't allow that. The example below shows how to bring two `Result` types into scope that have the same name but different path modules, and how to refer to them.

```Rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

# Providing New Names with the `as` Keyword

There's another solution to the problem of bringing two types of the same name into the same scope with `use`: After the path, we can specify `as` and a new local name, or *alias*, for the type. Rewriting the other example:

```Rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

# Re-exporting Names with `pub use`

When we bring a name into scope with the `use` keyword, the name is private to the scope into which we imported it. To enable code outside that scope to refer to that name as if it had been defined in that scope, we can combine `pub` and `use`. This technique is called *re-exporting* because we're bringing an item into scope but also making that item available for others to bring into their scope.

# Using External Packages

Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the `rand` package and any dependencies from crates.io and make `rand` available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a `use` line starting with the name of the crate, `rand`, and listed the items we wanted to bring into scope. Recall that in Chapter 2 we brought the `Rng` trait into scope and called the `rand::thread_rng` function.

Note that the standard `std` library is also a crate that's external to our package. Because the standard library is shipped with the Rust language, we don't need to change *Cargo.toml* to include `std`. But we do need to refer to it with `use` to bring items from there into our package's scope. For example, with `HashMap` we would use this line:

```Rust
use std::collections::HashMap;
```

This is an absolute path starting with `std`, the name of the standard library crate.

# Using Nested Paths to Clean Up `use` Lists

If we're using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. For example, these two `use` statements we had in the guessing game:

```Rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ:

```Rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

We can use a nested path at any level in a path, which is useful when combining two `use` statements that share a subpath. Like replacing this:
```Rust
use std::io;
use std::io::Write;
```

With this:

```Rust
use std::io::{self, Write};
```

# Importing items with the Glob Operator

If we want to bring *all* public items defined in a path into scope, we can specify that path followed by the `*` operator:

```Rust
use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined. Additionally, if the dependency changes its definitions, what you’ve imported changes as well, which may lead to compiler errors when you upgrade the dependency if the dependency adds a definition with the same name as a definition of yours in the same scope, for example.

The glob operator is often used when testing to bring everything under test into the tests module;
