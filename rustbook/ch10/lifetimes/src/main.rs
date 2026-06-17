// an instance of ImportantExcerpt cant outlive
// the reference it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // the lifetime parameter declaration after impl and its use after the
    // type name are required, but because of the first elision rule,
    // we are not required to annotate the lifetime of the reference to self
    fn level (&self) -> i32 {
        3
    }

    // Here the third lifetime elision rule applies:
    // Rust gives both &self and announcement their own lifetimes
    // Because one of the parameters is &self , the return type gets
    // the lifetime of &self and all lifetimes have been accounted for
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

use std::fmt::Display;

// this function gathers all: generic type parameters, trait bounds and lifetimes
// this returns the longer of two strings slices, but with
// an extra parameter 'ann' of generic type T that can be filled in by any
// type that implements the Display trait, as specified in the where clause
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");

    if x.len() > y.len() { x } else { y } 
}


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");


    // the data in novel must exist before the ImportantExcerpt instance is created
    // also, novel doesn't go out of scope until after the ImportantExcerpt goes
    // so the reference in ImportantExcerpt is still valid
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// we need to add generic lifetime parameters
// that define the relationship between the references
// so that the borrow checker can perform its analysis

// so we that for some lifetime 'a , the function takes two parameters,
// both of which are string slices that live at least as long as lifetime 'a
// and also the string slice returned from the function will live at least as long as lifetime 'a
// In practice, it means that the lifetime of the reference returned by the longest function
// is the same as the smaller of the lifetimes of the values referred to by the function arguments
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() { x } else { y }
}
