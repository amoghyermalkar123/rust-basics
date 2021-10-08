/*
A trait tells the Rust compiler about functionality a particular type has 
and can share with other types. We can use traits to define shared behavior 
in an abstract way. We can use trait bounds to specify that a generic type 
can be any type that has certain behavior.
*/
pub mod default;
pub mod trait_bound;

pub trait Summary {
    // just mention method signatures
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implement the Summary trait for NewsArticle type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
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
}

pub fn use_summary() {
    let tweet = Tweet{
        username: String::from("amogh"),
        content: String::from("this is content"),
        reply: false, 
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    let s = tweet.summarize();
    let a = article.summarize();
    println!("{}, {}", s, a)
}

pub fn take_any_type_that_has_summary_trait(data : &impl Summary) -> &impl Summary {
    data
}


