extern crate seawarfare_rs;
use seawarfare_rs::location::*;
extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    println!("Hello, world!");
    let mut a = Location::new(0, 0, 0);
    a.print();
    a.x = 1;
    a.print();

    let local = Local::now().naive_local();
    println!("Time format: {}", local.format("%m-%d-%Y %H:%M:%S"));

    let mut b = NaiveDateTime::parse_from_str("10/21/2015 17:02:00", "%m/%d/%Y %H:%M:%S")?;
    println!("b: {}", b);
    b = b + Duration::seconds(100);
    println!("b: {}", b);

    let diff = local - b;
    println!("diff: {}", diff.num_seconds());

    Ok(())
}
