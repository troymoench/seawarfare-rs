extern crate seawarfare_rs;
use seawarfare_rs::sim_manager::*;

extern crate chrono;


fn main() {
    let mut sim = SimManager::new();
    sim.init("orders05.txt");
    sim.print();
    sim.print_navy();
    sim.print_orders();
    sim.execute();
}
