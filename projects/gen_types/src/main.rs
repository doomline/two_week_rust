fn main() {
  
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);
    println!("The largest number is {}", result)

}

// you can iterate throught types by using .iter

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn largest_char(list: &[char]) -> char {

    let mut largest = list[0]

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

}


// the above functions have similar code that we can 
// reduce to one function to serve for both. We will use generic types
// use "T" for type (naming convention) generic or custom types are typically one letter


fn largest<T>(list: &[T]) -> {
    let mut largest = list[0]

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest 
}

// Structs using generic type

struct Point<T> {
    x: T,
    y: T,
}

fn wont_work() {
    let wont_work = Point { x: 5. y: 4.0}
}

//the above code won't work because X and Y have the same Type defined, so using
// different types or data wouldn't work. You'd have to define them separate
struct Point<T, U> {
    x: T,
    y: U,
}

fn will_work() {
    let will_work = Point { x: 5. y: 4.0}
}


// Traits
// a trait tells rust about a functionally particular type
// that can be shared

// lets say we need to share data between 
// NewsArticle, a tweet, and metadata indicating if it's new or retweet

// use the trait keyword
// we don't use the brace we use a semicolon
// Each type implementing this trait must provide it's own custom behavior for the body of the method

// using the (&self) invokes it as a method
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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

// implementing traits is similar to implementing methods
// once you put impl, state the trait, then use for 
// if you want to use default implementations you can
// you would do it likeso

pub trait Summary {
    // default implementations can call other methods in the same trait
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// you can use traits as parameters 

pub fn notify(item: impl Summary) {
    println!(" Breaking News! {}", item.summarize());
}

// this works but it wont work with Strings or i32 or....
// to do this you can implement generic trait 

pub fn notify<T: Summary>(item: T) {
    println!("Breaking News! {}", item.summarize());
}

// for multiple items

pub fn notify(item1: impl Summary, item2: impl Summary) {}

// for multiple types you'd write

pub fn notify<T: Summary>(item1: T, item2: T)