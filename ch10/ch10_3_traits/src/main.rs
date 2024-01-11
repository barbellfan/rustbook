mod aggregator;
use aggregator::{Summary, NewsArticle, Tweet};

fn main() {
    let na = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content content content"),
    };

    println!("summary of news article: {}", na.summarize());

    let tw = Tweet {
        username: String::from("@tweeter"),
        content: String::from("a tweet"),
        reply: false,
        retweet: false,
    };

    println!("summary of tweet: {}", tw.summarize());
}
