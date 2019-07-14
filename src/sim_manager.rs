use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use chrono::Duration;
use crate::order::*;
use crate::movable::*;

// typedef std::map<std::string, Movable*> NavyMap;
// typedef std::deque<Order*> OrderQueue;
//
// class SimulationMgr {
// public:
// 	bool simInit(std::string orderFile);
// 	void simDoUpdate(ATime);
// 	ATime getStart() const { return start; }
// 	ATime getStop() const { return stop; }
// 	NavyMap* getNavy() { return &nm; }
// 	void printOrders();
// 	void printNavy();
// 	void printHistList();
// private:
// 	ATime start;
// 	ATime stop;
// 	NavyMap nm;
// 	OrderQueue oq;
// };

type OrderQueue = Vec<Box<Order>>;
type NavyMap = HashMap<String, Box<Movable>>;

#[derive(Debug)]
pub enum Opcode {
	StartSim,
	StopSim,
	EndSim,
	CreateCruiser,
	CreateAircraftCarrier,
	CreateFighter,
	DeployShip,
	DeployAircraft,
	ChangeShipOrders,
	ChangeAircraftOrders,
	LandAircraft,
	Invalid,
}

impl Opcode {
	pub fn new(op: &str) -> Self {
		match op {
			 "StartSim" => Opcode::StartSim,
			 "StopSim" => Opcode::StopSim,
			 "EndSim" => Opcode::EndSim,
			 "CreateCruiser" => Opcode::CreateCruiser,
			 "CreateAircraftCarrier" => Opcode::CreateAircraftCarrier,
			 "CreateFighter" => Opcode::CreateFighter,
			 "DeployShip" => Opcode::DeployShip,
			 "DeployAircraft" => Opcode::DeployAircraft,
			 "ChangeShipOrders" => Opcode::ChangeShipOrders,
			 "ChangeAircraftOrders" => Opcode::ChangeAircraftOrders,
			 "LandAircraft" => Opcode::LandAircraft,
			 _ => Opcode::Invalid
		}
	}
}

pub struct SimManager {
	start: chrono::NaiveDateTime,
	stop: chrono::NaiveDateTime,
	navy_map: NavyMap,
	order_q: OrderQueue
}

impl SimManager {
	pub fn new() -> Self {
		SimManager {
			start: chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
			stop: chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
			navy_map: NavyMap::new(),
			order_q: OrderQueue::new()
		}
	}

	pub fn get_start(&self) -> chrono::NaiveDateTime {
		return self.start.clone()
	}

	pub fn get_stop(&self) -> chrono::NaiveDateTime {
		return self.stop.clone()
	}

	pub fn print(&self) {
		println!("start: {} stop: {}", self.start, self.stop);
	}

	pub fn print_orders(&self) {
		println!(" Order Queue");
		println!("=============");
		for o in self.order_q.iter() {
			o.print();
		}
		println!("=============");
	}

	pub fn print_navy(&self) {
		println!(" Navy Map");
		println!("==========");
		for n in self.navy_map.values() {
			n.print();
		}
		println!("==========");
	}

	/// Print the history of each movable in the navy map
	pub fn print_history(&self) {
		for (_, val) in self.navy_map.iter() {
			val.print();
			val.print_hl();
		}
	}

	/// Search for a movable in the navy map
	fn find_movable(&self, id: String) -> &Box<Movable> {
		return self.navy_map.get(id.as_str()).unwrap();
	}

	/// Initialize the simulation manager
	pub fn init(&mut self, filename: &str) -> bool {
		// open the file and read line by line
		// if the line starts with a # or is blank, skip

		// open the file. If it fails to open, complain and return false
		let f = File::open(filename);
		let f = match f {
			Ok(file) => file,
			Err(_error) => {
				panic!("There was a problem opening the file: {}", filename);
			},
		};


	    let buffered = BufReader::new(f);

	    for line in buffered.lines().map(|l| l.unwrap()) {
			if line.len() == 0 || line.starts_with("#") {
				continue;
			}
			// println!("{:?}", line);

			// split line on whitespace
			let mut tokens: Vec<&str> = line.split_whitespace().collect();
			let opcode = Opcode::new(&tokens.remove(0));
			// for s in &tokens {
			// 	println!("s: {}", s);
			// }
			// println!("opcode: {:?}", &opcode);

			match opcode {
				Opcode::StartSim => {
					let date_time_str = format!("{} {}", tokens[0], tokens[1]);
					let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																	   "%m/%d/%Y %H:%M:%S");
					match parsed {
						Ok(dt) => self.start = dt,
						Err(_error) => return false
					};
				},
				Opcode::StopSim | Opcode::EndSim => {
					let date_time_str = format!("{} {}", tokens[0], tokens[1]);
					let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																	   "%m/%d/%Y %H:%M:%S");
					match parsed {
						Ok(dt) => self.stop = dt,
						Err(_error) => return false
					};
				},
				Opcode::CreateCruiser => {
					let name = String::from(tokens[0]);
					let id = String::from(tokens[1]);
					let max_speed = tokens[2].parse::<f64>().unwrap();
					let missiles = tokens[3].parse::<i64>().unwrap();

					let mp = Box::new(Cruiser::new(name, id.clone(), max_speed, missiles));
					self.navy_map.insert(id, mp);
				},
				Opcode::CreateAircraftCarrier => {
					let name = String::from(tokens[0]);
					let id = String::from(tokens[1]);
					let max_speed = tokens[2].parse::<f64>().unwrap();
					let max_aircraft = tokens[3].parse::<i64>().unwrap();

					let mp = Box::new(Carrier::new(name, id.clone(), max_speed, max_aircraft));
					self.navy_map.insert(id, mp);
				},
				Opcode::CreateFighter => {
					let name = String::from(tokens[0]);
					let id = String::from(tokens[1]);
					let ship_id = String::from(tokens[2]);
					let max_speed = tokens[3].parse::<f64>().unwrap();
					let max_ceiling = tokens[4].parse::<f64>().unwrap();
					let max_bombs = tokens[5].parse::<i64>().unwrap();

					let mp = Box::new(Fighter::new(name, id.clone(), max_speed, ship_id.clone(), max_ceiling, max_bombs));
					self.navy_map.insert(id, mp);
				},
				Opcode::DeployShip => {
					let date_time_str = format!("{} {}", tokens[0], tokens[1]);
					let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																	   "%m/%d/%Y %H:%M:%S");
					let atm = match parsed {
						Ok(dt) => dt,
						Err(_error) => return false
					};

					let id = String::from(tokens[2]);
					let x = tokens[3].parse::<f64>().unwrap();
					let y = tokens[4].parse::<f64>().unwrap();
					let head = tokens[5].parse::<f64>().unwrap();
					let spd = tokens[6].parse::<f64>().unwrap();
					let op = DeployShip::new(atm, id, x, y, head, spd);
					self.order_q.push(Box::new(Order::DeployShipOrder(op)));
				},
				Opcode::DeployAircraft => {
					let date_time_str = format!("{} {}", tokens[0], tokens[1]);
					let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																	   "%m/%d/%Y %H:%M:%S");
					let atm = match parsed {
						Ok(dt) => dt,
						Err(_error) => return false
					};

					let id = String::from(tokens[2]);
					let head = tokens[3].parse::<f64>().unwrap();
					let spd = tokens[4].parse::<f64>().unwrap();
					let z = tokens[5].parse::<f64>().unwrap();
					let op = DeployAircraft::new(atm, id, head, spd, z);
					self.order_q.push(Box::new(Order::DeployAircraftOrder(op)));
				},
				Opcode::ChangeShipOrders => {
					let date_time_str = format!("{} {}", tokens[0], tokens[1]);
					let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																	   "%m/%d/%Y %H:%M:%S");
					let atm = match parsed {
						Ok(dt) => dt,
						Err(_error) => return false
					};

					let id = String::from(tokens[2]);
					let head = tokens[3].parse::<f64>().unwrap();
					let spd = tokens[4].parse::<f64>().unwrap();
					let op = ChangeShip::new(atm, id, head, spd);
					self.order_q.push(Box::new(Order::ChangeShipOrder(op)));
				},
				Opcode::ChangeAircraftOrders => {
					let date_time_str = format!("{} {}", tokens[0], tokens[1]);
					let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																	   "%m/%d/%Y %H:%M:%S");
					let atm = match parsed {
						Ok(dt) => dt,
						Err(_error) => return false
					};

					let id = String::from(tokens[2]);
					let head = tokens[3].parse::<f64>().unwrap();
					let spd = tokens[4].parse::<f64>().unwrap();
					let z = tokens[5].parse::<f64>().unwrap();
					let op = ChangeAircraft::new(atm, id, head, spd, z);
					self.order_q.push(Box::new(Order::ChangeAircraftOrder(op)));
				},
				Opcode::LandAircraft => {
					let date_time_str = format!("{} {}", tokens[0], tokens[1]);
					let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																	   "%m/%d/%Y %H:%M:%S");
					let atm = match parsed {
						Ok(dt) => dt,
						Err(_error) => return false
					};

					let ship_id = String::from(tokens[2]);
					let id = String::from(tokens[3]);
					let op = LandAircraft::new(atm, id, ship_id.clone());
					self.order_q.push(Box::new(Order::LandAircraftOrder(op)));
				},
				Opcode::Invalid => {
					println!("Invalid opcode: {:?}", opcode);
					return false;
				},
			}

	    }
		// sort the order queue
		self.order_q.sort();
		// self.order_q.sort_by(|a, b| a.get_extime().partial_cmp(&b.get_extime()).unwrap());
		return true;
	}

	/// Execute orders and update the navy map for a given time
	pub fn do_update(&mut self, now: chrono::NaiveDateTime) {
		// execute any orders that are scheduled to be executed
		while !self.order_q.is_empty() {
			let o = self.order_q.first().unwrap();
			if o.get_extime() > now {break;}
			let o = &self.order_q.remove(0);
			let mov = match self.navy_map.get_mut(o.get_id().as_str()) {
				Some(id) => id,
				None => {println!("Unable to find id: {}, skipping order", o.get_id().as_str()); continue;}
			};
			mov.execute(o);
		}
		// update the position of all deployed movables
		for (_, val) in self.navy_map.iter_mut() {
			if val.get_is_deployed() {
				val.update_position(now);
			}
		}
	}

	/// Execute the simulation
	pub fn execute(&mut self) {
		println!("starting sim");
		let mut t = self.start;
		while t < self.stop {
			self.do_update(t);
			t += Duration::seconds(60);
		}
		println!("sim completed");
	}
}
