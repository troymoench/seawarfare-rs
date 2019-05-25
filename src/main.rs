extern crate seawarfare_rs;
use seawarfare_rs::location::*;
use seawarfare_rs::order::*;
use seawarfare_rs::movable::*;
use seawarfare_rs::sim_manager::*;

extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    println!("Hello, world!");

    let local = Local::now().naive_local();
    println!("Time format: {}", local.format("%m-%d-%Y %H:%M:%S"));

    let mut b = NaiveDateTime::parse_from_str("10/21/2015 17:02:00", "%m/%d/%Y %H:%M:%S")?;
    println!("b: {}", b);
    b = b + Duration::seconds(100);
    println!("b: {}", b);

    let diff = local - b;
    println!("diff: {}", diff.num_seconds());

    let mut sim = SimManager::new();
    sim.init("orders05.txt");
    sim.print();
    sim.print_navy();
    sim.print_orders();
    Ok(())
}
