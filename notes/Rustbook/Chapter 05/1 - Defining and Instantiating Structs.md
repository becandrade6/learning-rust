# Defining and Instantiating Structs

Structs are similar to tuples, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you will name each piece of data so it is clear what the values mean. 

To define a struct, we enter the keyword `struct` and name the entire struct. A struct's name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call *fields*.

To get a specific value from a struct, we use dot notation. For example, to access this user’s email address, we use user1.email. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Listing 5-3 shows how to change the value in the email field of a mutable User instance.

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

## Using the Field Init Shorthand

Because the parameter names and the struct field names are exactly the same, we can use the *field init shorthand* syntax to rewrite the function so that i behaves exactly the same but does not have the repetition of username and email.

## Creating instances with struct update syntax

it is often useful to create a new instance of a struct that includes most of the values from another instance of the same type, but changes some of them. You can do this using struct update syntax.

The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section. In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply. We can also still use user1.email in this example, because its value was not moved out of user1.

## Creating Different Types with Tuple Structs

Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

## Defininf Unit-Like Structs

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section. Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. We’ll discuss traits in Chapter 10. Here’s an example of declaring and instantiating a unit struct named AlwaysEqual:

To define AlwaysEqual, we use the struct keyword, the name we want, and then a semicolon. No need for curly brackets or parentheses! Then, we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses. Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior! You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.

