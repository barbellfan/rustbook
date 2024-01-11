use std::fmt::{Display, Debug               };

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

// returning a type that implements traits
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
/* This won't work though. Only one type may be returned.
fn returns_summarizable_bad(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/

// using trait bounds to conditionally implement methods
// _cmp_display can only be called if T has both traits.
struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementations
/*
impl<T: Display> ToString for T {
    // snip
}
*/
