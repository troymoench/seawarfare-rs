use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::rc::Rc;
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
type NavyMap = HashMap<String, Rc<Movable>>;

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
	LandAircraft
}

impl Opcode {
	pub fn as_str(&self) -> &'static str {
		match self {
			Opcode::StartSim => "StartSim",
			Opcode::StopSim => "StopSim",
			Opcode::EndSim => "EndSim",
			Opcode::CreateCruiser => "CreateCruiser",
			Opcode::CreateAircraftCarrier => "CreateAircraftCarrier",
			Opcode::CreateFighter => "CreateFighter",
			Opcode::DeployShip => "DeployShip",
			Opcode::DeployAircraft => "DeployAircraft",
			Opcode::ChangeShipOrders => "ChangeShipOrders",
			Opcode::ChangeAircraftOrders => "ChangeShipOrders",
			Opcode::LandAircraft => "LandAircraft"
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
	fn find_movable(&self, id: String) -> &Rc<Movable> {
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
				println!("Skipping line");
				continue;
			}
			// println!("{:?}", line);

			// split line on whitespace
			let mut tokens: Vec<&str> = line.split_whitespace().collect();
			let opcode = &tokens.remove(0);
			// for s in &tokens {
			// 	println!("s: {}", s);
			// }
			println!("opcode: {}", &opcode);

			if *opcode == "StartSim" {
				let date_time_str = format!("{} {}", tokens[0], tokens[1]);
				let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																   "%m/%d/%Y %H:%M:%S");
				match parsed {
					Ok(dt) => self.start = dt,
					Err(_error) => return false
				};
			}
			else if *opcode == "StopSim" || *opcode == "EndSim" {
				let date_time_str = format!("{} {}", tokens[0], tokens[1]);
				let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																   "%m/%d/%Y %H:%M:%S");
				match parsed {
					Ok(dt) => self.stop = dt,
					Err(_error) => return false
				};
			}
			else if *opcode == "CreateCruiser" {
				let name = String::from(tokens[0]);
				let id = String::from(tokens[1]);
				let max_speed = tokens[2].parse::<f64>().unwrap();
				let missiles = tokens[3].parse::<i64>().unwrap();

				let mp = Rc::new(Cruiser::new(name, id.clone(), max_speed, missiles));
				self.navy_map.insert(id, mp);
			}
			else if *opcode == "CreateAircraftCarrier" {
				let name = String::from(tokens[0]);
				let id = String::from(tokens[1]);
				let max_speed = tokens[2].parse::<f64>().unwrap();
				let max_aircraft = tokens[3].parse::<i64>().unwrap();

				let mp = Rc::new(Carrier::new(name, id.clone(), max_speed, max_aircraft));
				self.navy_map.insert(id, mp);
			}
			else if *opcode == "CreateFighter" {
				let name = String::from(tokens[0]);
				let id = String::from(tokens[1]);
				let ship_id = String::from(tokens[2]);
				let max_speed = tokens[3].parse::<f64>().unwrap();
				let max_ceiling = tokens[4].parse::<f64>().unwrap();
				let max_bombs = tokens[5].parse::<i64>().unwrap();

				let ship_ptr = self.find_movable(ship_id);
				let mp = Rc::new(Fighter::new(name, id.clone(), max_speed, Rc::clone(ship_ptr), max_ceiling, max_bombs));
				self.navy_map.insert(id, mp);
			}
			else if *opcode == "DeployShip" {
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
			}
			else if *opcode == "DeployAircraft" {
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

			}
			else if *opcode == "ChangeShipOrders" {
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
			}
			else if *opcode == "ChangeAircraftOrders" {
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
			}
			else if *opcode == "LandAircraft" {
				let date_time_str = format!("{} {}", tokens[0], tokens[1]);
				let parsed = chrono::NaiveDateTime::parse_from_str(date_time_str.as_str(),
																   "%m/%d/%Y %H:%M:%S");
				let atm = match parsed {
					Ok(dt) => dt,
					Err(_error) => return false
				};

				let ship_id = String::from(tokens[2]);
				let id = String::from(tokens[3]);
				let ship_ptr = self.find_movable(ship_id);
				let op = LandAircraft::new(atm, id, Rc::clone(ship_ptr));
				self.order_q.push(Box::new(Order::LandAircraftOrder(op)));
			}
			else {
				println!("Invalid opcode: {}", opcode);
				return false;
			}

	    }
		// sort the order queue
		self.order_q.sort();
		// self.order_q.sort_by(|a, b| a.get_extime().partial_cmp(&b.get_extime()).unwrap());
		return true;
	}
	//
	// /// Execute orders and update the navy map for a given time
	// pub fn do_update(&mut self, now: chrono::NaiveDateTime) {
	// 	// execute any orders that are scheduled to be executed
	// 	while !self.order_q.is_empty() {
	// 		let o = self.order_q.get_mut(0).unwrap();
	// 		if o.get_extime() > now {break;}
	// 		let o = self.order_q.first().unwrap();
	// 		// TODO: add error handling
	// 		// let mov = self.find_movable(o.get_id());
	// 		let mov = self.navy_map.get(o.get_id().as_str()).unwrap();
	// 		o.execute(mov.clone(), now);
	// 	}
	//
	// 	// update the position of all deployed movables
	// 	for (_, val) in self.navy_map.iter_mut() {
	// 		if val.get_is_deployed() {
	// 			val.update_position(now);
	// 		}
	// 	}
	//
	// }
}
