use std::fmt::{Debug, Display};
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl<T: Display> Summary for T {
    fn summarize_author(&self) -> String {
        format!("{}", self)
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_books"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

pub fn notify<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(t: T, u: U) -> T
where
    T: Display + Clone,
    U: Clone + Debug,
{
    t
}
