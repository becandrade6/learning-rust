# Defining Shared Behavior with Traits

A *trait* defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use *trait bounds* to specify that a generic type can be any type that has certain behavior.

> Note: Traits are similar to a feature often called *interfaces* in other languages, although with some differences.

# Defining a Trait

Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

We declare a trait using the `trait` keyword and the trait's name.

Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait.

After the method signature, instead of providing an implementation within curly brackets, we use a semicolon. Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.

A trait can have multiple methods in its body: The method signatures are listed one per line, and each line ends in a semicolon.

# Implementin a Trait on a Type

Implementing a trait on a type is similar to implementing regular methods. The difference is that after `impl`, we put the trait name we want to implement, the use the `for` keyword, and then specify the name of the type we want to implement the trait for.  Within the `impl` block, we put the method signatures that the trait definition has defined. Instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

# Using Default Implementations

Sometimes it's useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method's default behavior.

# Using Traits as Parameters

Now that you know how to define and implement traits, we can explore how to use traits to define functions that accept many different types.

# Multiple Trait Bounds with the + Syntax

We can also specify more than one trait bound. Say we wanted `notify` to use display formatting as well as `summarize` on `item`: We specify in the `notify` definition that `item` must implement both `Display` and `Summary`. We can do using the `+` syntax:

```Rust
pub fn notify(item: &(impl Summary + Display)) {
```

or with trait bounds on generic types:

```Rust
pub fn notify<T: Summary + Display>(item: &T) {
```

With the two trait bounds specified, the body of `notify` can call `summarize` and use `{}` to format `item`.

# Clearer Trait Bounds with where Clauses

