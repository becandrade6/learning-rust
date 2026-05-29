fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let s = String::from("hello");
    // this below wont work, because we cannot modify something we're borrowing
    // change_not_mutable(&s);

    let mut mutable_s = String::from("hello");

    change_mutable(&mut mutable_s);

    // We cannot combine mutable and immutable references
    let mut s = String::from("hello");
    let r1 = &s;    // no problem
    let r2 = &s;    // no problem
    //let r3 = &mut s;    // BIG PROBLEM

    
}

// define and use a calculate_length function that
// has a reference to an object as a parameter
// instead of taking ownership of the value
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn change_not_mutable(some_string: &String) {
    some_string.push_str(", world");
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

