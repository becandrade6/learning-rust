fn main() {
    // ------------------------ Creating a New String ------------------------
    // this line creates a new, empty string called s, into which we can then load data
    let mut s = String::new();

    // often we'll have some initial data with which we want to start the string
    // so we use to_string method, which is available on any type that implements
    // the Display trait, as string literals do.
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly
    let s = "initial contents".to_string();

    // we can also use the function String::from to create a String from a string literal.
    // This is equivalent to using the method to_string
    let s = String::from("initial contents");

    // remember that strings are UTF-8 encoded, so we can include any properly encoded data in them
    // such as all these valid String values:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // ------------------------ Updating a String ------------------------

    // we can grow a String using the push_str method to append a string slice
    // after the lines below s will contain 'foobar'
    let mut s = String::from("foo");
    s.push_str("bar");

    // the push_str method takes a string slice because we don't necessarily want to
    // take ownership of the parameter. Like below we want to be able to use s2 after appending its content to s1
    // if the push_str took ownership of s2 we would not be able to print its value on the last line
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // the push method takes a single character as a parameter and adds it to the String
    let mut s = String::from("lo");
    s.push('l');

    // often you'll want to combine two existing strings. One way to do so is
    // to use the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;      // note s1 has been moved here and can no longer be used

    // if we need to concatenate multiple strings, the behavior of the + operator gets unwieldy
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // we can use format! to combine strings in a more fashion way
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // ------------------------ Slicing Strings ------------------------
    // use [] with a range to create a string slice containing particular bytes
    // here s will be a &str that contains the first 4 bytes of the string.
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    // If we were trying to slice only part of a character's bytes with something
    // like &hello[0..1] Rust would panic at runtime in the same way as if an invalid
    // index were accessed in a vector

    // ------------------------ Iterating over Strings ------------------------
    // iterating specifying about the chars method (i want individual unicode scalar values)
    println!("Iterating in chars through Зд");
    for c in "Зд".chars() {
        println!("{c}");
    }

    // iterating specifying about the bytes method (returns each raw byte)
    println!("Iterating in bytes through Зд");
    for b in "Зд".bytes() {
        println!("{b}");
    }

    
}
