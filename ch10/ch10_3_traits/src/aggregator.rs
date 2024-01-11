use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("about the author: {}", self.author)
    }
    //fn summarize(&self) -> String {
    //    format!("{}, by {} ({})", self.headline, self.author, self.location)
    //}
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Using trait bound syntax

// two parameters that both implement Summary
// params may be different types
pub fn _notify2(_item1: &impl Summary, _item2: &impl Summary) { }

// params must now be the same type
pub fn _notify3<T: Summary>(_item1: &T, _item2: &T) { }

//specifying multiple trait bounds with the + syntax
pub fn _notify4(_item: &(impl Summary + Display)) { }

// trait bounds on generic types
pub fn _notify5<T: Summary + Display>(_item: &T) { }

// trait bound with where clauses
// this is complex
fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) { }
// this is more clear
fn _some_function2<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}
