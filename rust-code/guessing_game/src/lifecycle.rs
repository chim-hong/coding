use std::fmt::Display;

fn longest_with_an_announcement<'a,T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main() {
    let x = String::from("hello");
    let y = "world";
    let ann = 123;
    longest_with_an_announcement(&x, y, ann);
}
