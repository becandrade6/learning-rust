# The Slice Type

*Slices* let you reference a consecutive sequence of elements in a collection. A slice is a kind of reference, so it does not have ownership.

Here’s a small programming problem: Write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

## String Slices

A *string slice* is a reference to a consecutive sequence of the elements of a `String`.

Now when we call first_word, we get back a single value that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

## String Literals as Slices

String literals are stored inside the binary. 

```Rust
let s = "Hello, world!";
```

The type of s here is &str: It’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

## String Slices as Parameters

Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing any functionality.

# Other Slices

String slices are specific to strings, but there's a more general slice type too.

we can create slices of other types just as we did with strings.

