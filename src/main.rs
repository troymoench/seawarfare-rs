extern crate seawarfare_rs;
use seawarfare_rs::sim_manager::*;
use std::env;
extern crate chrono;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let mut sim = SimManager::new();
    sim.init(filename);
    sim.print();
    sim.print_navy();
    sim.print_orders();
    sim.execute();
    sim.print_history();
}
