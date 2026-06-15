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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
