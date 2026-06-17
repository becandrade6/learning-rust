pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())      // default implementation
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implements the trait Summary for the struct NewsArticle
// uses the default implementation of summarize
// implements the summarize author
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// implements both functions for the trait
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// instead of a concrete type for the item parameter
// we specify the impl keyword and the trait name
// this parameter accepts any type that implements the specified trait
// In the body of notify, we can call any methods on item that come from
// Summary trait, such as summarize. We can call notify and pass in any
// instance of NewsArticle or SocialPost
// Code that calls the function with any other type such as String or i32 wont compile
// because they don't implement Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// now implemented via trait bound
pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// via trait bound, having two parameters becomes cleaner
pub fn notify_3<T: Summary>(item1: &T, item2: &T){
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

use::std::fmt::Display;

// we can specify more than one trait bound also
// the item here must implement both Display and Summary traits
pub fn notify_4<T: Summary + Display>(item1: &T, item2: &T){
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// we can also use a where clause, to keep syntax clearer
pub fn notify_5<T>(item1: &T, item2: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// we can also use the impl Trait syntax in the return position to return a value of some type that implements a trait
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{ x, y}
    }
}

// here we implement the cmp_display only if its inner
// type T implements the PartialOrd trait that enables comparison
// and the Display trait that enables printing
impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

