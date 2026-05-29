fn main() {
    // This is what a string slice is
    let s = String::from("hello world");

    // hello is a reference to a portion of the String, specified in the extra [0..5] bit.
    // We create slices using a range within square brackets [starting_index..ending_index]
    // where starting_index is the first position in the slice and ending_index is one more than the last position in the slice
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}, {world}");

    // both below are equivalent
    // let slice = &s[0..2];
    // let slice = &s[..2];

    // also, you can drop both values to take a slice of the entire string, so these are equal:
    // let len = s.len();
    // let slice = &s[0..len];
    // let slice = &s[..];

    let my_string = String::from("hello world");

    // 'first_word' works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    println!("word 1 = {word}");
    let word = first_word(&my_string[..]);
    println!("word 2 = {word}");
    // 'first_word also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);
    println!("word 3 = {word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    println!("word 1 = {word}");
    let word = first_word(&my_string_literal[..]);
    println!("word 2 = {word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word 3 = {word}");
}

// Create a function that takes a string of words separated by spaces
// and returns the first word it finds in that string
// if the function doesn't find a space in the string, the whole string
// must be one word, so the entire string should be returned
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            // when we find a space, return a string slice using the start
            // of the string and the index of the space as starting and ending indices
            return &s[0..i];
        }
    }

    // otherwise, if we do not find a space, return a slice of the entire string
    &s[..]
}

