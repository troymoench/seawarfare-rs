extern crate seawarfare_rs;
use seawarfare_rs::location::*;

fn main() {
    println!("Hello, world!");
    let mut a = Location::new(0, 0, 0);
    a.print();
    a.x = 1;
    a.print();
}
