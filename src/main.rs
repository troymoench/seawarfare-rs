extern crate seawarfare_rs;
use seawarfare_rs::sim_manager::*;

extern crate chrono;


fn main() {
    let mut sim = SimManager::new();
    sim.init("orders01.txt");
    sim.print();
    sim.print_navy();
    sim.print_orders();
    sim.execute();
    sim.print_history();
}
