fn main() {
    // ----------------- BASICS ----------------------
    // the String has 3 'fields'
    // ptr (points to the heap where the string literal is)
    // len (how much memory, in bytes, the contents of the String are currently using)
    // capacity (the total amount of memory, in bytes, the String has received from the allocator)
    let s1 = String::from("hello");

    // when we do this below, we create another stack, copying the 3 fields of s1 to s2
    // but now, we would have two variables with the field 'ptr' pointing to the same heap space
    // so. when both vars go out of scope, we would have the double free error
    // we would try to free memory twice by freeing the ptr
    let s2 = s1;

    // to prevent this, Rust considers s1 as no longer valid after new assignment
    // So, Rust does not need to free anything when s1 goes out of scope

    //if we try the line belo, it wont work
    //println!("{s1}, world");

    // ----------------- SCOPE AND ASSIGNMENT ----------------------

    // creates new data in heap and binds to s in stack
    let mut s = String::from("hello");

    // the original string immediately goes out of scope, Rust will run the drop function on it
    // and its memory will be freed right away.
    // now, a new data is created in heap and binds to s in stack
    s = String::from("ahoy");

    // this prints 'ahoy, world'
    println!("{s}, world");

    // ----------------- INTERACTING WITH CLONE ----------------------

    // this clones the first data on heap to another heap space and binds to s2_clone on stack
    let s2_clone = s2.clone();

    println!("s2 = s1 = {s2}), s2_clone = {s2_clone}");

    // ----------------- OWNERSHIP AND FUNCTIONS ----------------------

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here

    let x = 5; // x comes into scope

    // Because i32 implements the Copy trait, x does NOT move into the function, so it's okay to use x afterward
    makes_copy(x);
}
// Here, x goes out of scope, then s. However, because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and 'drop' is called. The baking memory is freed.

fn makes_copy(some_integer: u32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
