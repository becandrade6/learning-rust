# More about Cargo and Crates.io

We'll look into what Cargo can do and how to publish and install things on crates.io

- Customize your build through release profiles.
- Publish libraries on crates.io
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend Cargo using custom commands


# Customizing Builds with Release Profiles

*Release profiles* are predefined, customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.

Cargo has two main profiles:

- `dev` profile Cargo uses when you run `cargo build`. It builds fast but do not generate optimized binaries;
- `release` profile Cargo uses when you run `cargo build --release`. It makes optimizations so the code runs faster, but takes longer in build;

To create a profile or personalize one, we have to change the *Cargo.toml* file. By adding `[profile.*]` sections for any profile you want to customize, you override any subset of the default settings also, like:

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

For the full list of configuration options and defaults for each profile, see [Cargo's documentation](https://doc.rust-lang.org/cargo/reference/profiles.html).

# Making Useful Documentation Comments

Rust also has a particular kind of comment for documentation, known conveniently as a *documentation comment*, that will generate HTML documentation. The HTML displays the contents of documentation comments for public API items intended for programmers interested in knowing how to *use* your crate as opposed to how your crate is *implemented*.

Documentation comments use three slashes, `///`, instead of two and support Markdown notation for formatting the text. Place documentation comments just before the item they're documenting. Like:

```Rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

We can generate the HTML documentation from this documentation comment by running `cargo doc`. This command runs the `rustdoc` tool distributed with Rust and puts the generated HTML documentation in the *target/doc* directory.

For convenience, running `cargo doc --open` will build the HTML for your current crate's documentation (as well as the documentation for all of your crate's dependencies) and open the result in a web browser.

## Commonly Used Sections

- **# Examples** : show examples of usage of that function;
- **# Panics**: these are the scenarios in which the function being documented could panic. Callers of the function who don't want their programs to panic should make sure they don't call the function in these situations;
- **# Errors**: If the function returns a `Result`, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so that they can write code to handle the different kinds of errors in different ways;
- **# Safety**: If the function is `unsafe` to call (we discuss unsafety in Chapter 20) there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

Most documentation comments don't need all of these sections, but this is a good checklist to remind the aspects of the code users will be interested in knowing about.

# Documentation Comments as Tests

Adding example code blocks in the documentation comments can help demonstrate how to use the library and has an additional bonus: Running `cargo test` will run the code examples in the documentation as tests! Nothing is better than documenting with examples. But nothing is worse than examples that don't work because the code has changed since the documentation was written.

# Contained Item Comments

The style of doc comment `//!` adds documentation to the item that *contains* the comments rather than to the items *following* the comments. We tipically use these doc comments inside the crate root file (*src/lib.rs* by convention) or inside a module to document the crate or the module as a whole.

For example, to add documentation that describes the purpose of the `my_crate` crate that contains the `add_one` function, we add documentation comments that start with `//!` to the beginning of the *src/lib.rs* file, such as:

```Rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

Notice there isn't any code after the last line that begins with `//!`. Because we started the comments with `//!` instead of `///`, we're documenting the item that contains this comment rather than an item that follows this comment. In this case, that item is the *src/lib.rs* file, which is the crate root. These comments describe the entire crate.

When we run `cargo doc --open`, these comments will display on the front page of the documentation for `my_crate` above the list of public items in the crate.

Documentation comments within items are useful for describing crates and modules especially. Use them to explain the overall purpose of the container to help your users understand the crate's organization.

# Exporting a Convenient Public API

The structure of your public API is a major consideration when publishing a crate. People who use your crate are less familiar with the structure than you are and might have difficulty finding the pieces they want to use if your crate has a large module hierarchy.

In Chapter 7, we covered how to make items public using the `pub` keyword, and how to bring items into a scope with the `use` keyword. However, the structure that makes sense to you while you're developing a crate might not be very convenient for your users. You might want to organize your structs in a hierarchy containing multiple levels, but then people who want to use a type you've defined deep in the hierarchy might have trouble finding out that type exists. They might also be annoyed at having to enter `use my_crate::some_module::another_module::UsefulType;` rather than `use my_crate::UsefulType;`.

The good news is that if the structure isn’t convenient for others to use from another library, you don’t have to rearrange your internal organization: Instead, you can re-export items to make a public structure that’s different from your private structure by using `pub use`. *Re-exporting* takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.

We exemplify this with the `art` crate in the examples folder.

In cases where there are many nested modules, re-exporting the types at the top level with `pub use` can make a significant difference in the experience of people who use the crate. Another common use of `pub use` is to re-export definitions of a dependency in the current crate to make that crate’s definitions part of your crate’s public API.

