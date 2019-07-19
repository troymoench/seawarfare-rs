use crate::location::*;
use crate::order::*;


type HistoryList = Vec<Location>;

pub trait Movable {
    fn get_is_deployed(&self) -> bool;
    fn get_was_deployed(&self) -> bool;
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_location(&self) -> Location;
    fn get_history(&self) -> &HistoryList;
    fn deploy(&mut self, x: f64, y: f64, head: f64, spd: f64, t: chrono::NaiveDateTime) -> bool;
    fn change(&mut self, head: f64, spd: f64, alt: f64, t: chrono::NaiveDateTime) -> bool;
    fn update_position(&mut self, t: chrono::NaiveDateTime, loc_map: &LocationMap);
    fn execute(&mut self, order: &Order);
    fn print(&self) {
        println!("Name: {} ID: {}", self.get_name(), self.get_id());
    }
    fn print_hl(&self) {
        for loc in self.get_history() {
            loc.print();
        }
    }
}

/// Calculate a new position using 'dead reckoning'
fn calc_new_position(loc: Location, heading: f64, speed: f64, curr_tm: chrono::NaiveDateTime,
                     prev_tm: chrono::NaiveDateTime) -> Location {
     let time: f64 = ((curr_tm - prev_tm).num_seconds() as f64) / (60.0 * 60.0);
     let distance = speed * time;
     let dx = distance * heading.to_radians().sin();
     let dy = distance * heading.to_radians().cos();
     return Location::new(loc.x + dx, loc.y + dy, loc.z, curr_tm);
}

pub trait Ship {
    fn change(&self, head: f64, spd: f64, alt: f64, t: chrono::NaiveDateTime) -> bool;
    fn update_position(&self, t: chrono::NaiveDateTime);
}


#[derive(Debug)]
pub struct Cruiser {
    name: String,
    id: String,
    at: chrono::NaiveDateTime,
    loc: Location,
    is_deployed: bool,
    was_deployed: bool,
    heading: f64,
    speed: f64,
    max_speed: f64,
    hl: HistoryList,
    max_missles: i64
}

impl Cruiser {
    pub fn new(name: String, id: String, max_speed: f64, max_missles: i64) -> Self {
        Cruiser {
            name: name,
            id: id,
            at: chrono::NaiveDate::from_ymd(2019, 1, 1).and_hms(0, 0, 0),
            loc: Location::default(),
            is_deployed: false,
            was_deployed: false,
            heading: 0.0,
            speed: 0.0,
            max_speed: max_speed,
            hl: HistoryList::new(),
            max_missles: 0
        }
    }
}

impl Movable for Cruiser {
    fn get_is_deployed(&self) -> bool {
        return self.is_deployed;
    }
    fn get_was_deployed(&self) -> bool {
        return self.was_deployed;
    }
    fn get_id(&self) -> String {
        return self.id.clone();
    }
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_location(&self) -> Location {
        return self.loc.clone();
    }
    fn get_history(&self) -> &HistoryList {
        return &self.hl;
    }
    /// determine which order we have received
    /// and call the function associated with it
    fn execute(&mut self, order: &Order) {
        let result = match order {
            Order::DeployShipOrder(o) => self.deploy(o.start_x, o.start_y, o.heading, o.speed, o.extime),
            Order::ChangeShipOrder(o) => self.change(o.heading, o.speed, 0.0, o.extime),
            _ => false
        };
    }
    fn deploy(&mut self, x: f64, y: f64, head: f64, spd: f64, t: chrono::NaiveDateTime) -> bool {
        self.is_deployed = true;
        self.was_deployed = true;
        self.loc = Location::new(x, y, 0.0, t);
        self.hl.push(self.loc.clone());
        self.heading = head;
        self.speed = spd;
        self.at = t;
        return true;
    }
    fn change(&mut self, head: f64, spd: f64, alt: f64, t: chrono::NaiveDateTime) -> bool {
        // self.update_position(t);
        if head != -1.0 {
            self.heading = head;
        }
        if spd != -1.0 {
            self.speed = spd;
        }
        return true;
    }
    fn update_position(&mut self, t: chrono::NaiveDateTime, loc_map: &LocationMap) {
        if self.at == t {
            return;
        }
        self.loc = calc_new_position(self.loc.clone(), self.heading, self.speed, t, self.at);
        self.hl.push(self.loc.clone());
        self.at = t;
    }
}

#[derive(Debug)]
pub struct Carrier {
    name: String,
    id: String,
    at: chrono::NaiveDateTime,
    loc: Location,
    is_deployed: bool,
    was_deployed: bool,
    heading: f64,
    speed: f64,
    max_speed: f64,
    hl: HistoryList,
    max_aircraft: i64
}

impl Carrier {
    pub fn new(name: String, id: String, max_speed: f64, max_aircraft: i64) -> Self {
        Carrier {
            name: name,
            id: id,
            at: chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
            loc: Location::default(),
            is_deployed: false,
            was_deployed: false,
            heading: 0.0,
            speed: 0.0,
            max_speed: max_speed,
            hl: HistoryList::new(),
            max_aircraft: max_aircraft
        }
    }
}

impl Movable for Carrier {
    fn get_is_deployed(&self) -> bool {
        return self.is_deployed;
    }
    fn get_was_deployed(&self) -> bool {
        return self.was_deployed;
    }
    fn get_id(&self) -> String {
        return self.id.clone();
    }
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_location(&self) -> Location {
        return self.loc.clone();
    }
    fn get_history(&self) -> &HistoryList {
        return &self.hl;
    }
    /// determine which order we have received
    /// and call the function associated with it
    fn execute(&mut self, order: &Order) {
        let result = match order {
            Order::DeployShipOrder(o) => self.deploy(o.start_x, o.start_y, o.heading, o.speed, o.extime),
            Order::ChangeShipOrder(o) => self.change(o.heading, o.speed, 0.0, o.extime),
            _ => false
        };
    }
    fn deploy(&mut self, x: f64, y: f64, head: f64, spd: f64, t: chrono::NaiveDateTime) -> bool {
        self.is_deployed = true;
        self.was_deployed = true;
        self.loc = Location::new(x, y, 0.0, t);
        self.hl.push(self.loc.clone());
        self.heading = head;
        self.speed = spd;
        self.at = t;
        return true;
    }
    fn change(&mut self, head: f64, spd: f64, alt: f64, t: chrono::NaiveDateTime) -> bool {
        // self.update_position(t);
        if head != -1.0 {
            self.heading = head;
        }
        if spd != -1.0 {
            self.speed = spd;
        }
        return true;
    }
    fn update_position(&mut self, t: chrono::NaiveDateTime, loc_map: &LocationMap) {
        if self.at == t {
            return;
        }
        self.loc = calc_new_position(self.loc.clone(), self.heading, self.speed, t, self.at);
        self.hl.push(self.loc.clone());
        self.at = t;
    }
}

// #[derive(Debug)]
pub struct Fighter {
    name: String,
    id: String,
    at: chrono::NaiveDateTime,
    loc: Location,
    is_deployed: bool,
    was_deployed: bool,
    heading: f64,
    speed: f64,
    max_speed: f64,
    hl: HistoryList,
    is_landing: bool,
	ship_id: String,
    ship_loc: Location,
	max_ceiling: f64,
	altitude: f64,
	max_bombs: i64
}

impl Fighter {
    pub fn new(name: String, id: String, max_speed: f64, ship_id: String, max_ceiling: f64, max_bombs: i64) -> Self {
        Fighter {
            name: name,
            id: id,
            at: chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
            loc: Location::default(),
            is_deployed: false,
            was_deployed: false,
            heading: 0.0,
            speed: 0.0,
            max_speed: max_speed,
            hl: HistoryList::new(),
            is_landing: false,
            ship_id: ship_id,
            ship_loc: Location::default(),
            max_ceiling: max_ceiling,
            altitude: 0.0,
            max_bombs: max_bombs
        }
    }

    pub fn deploy(&mut self, head: f64, spd: f64, alt: f64, t: chrono::NaiveDateTime) -> bool {
        self.is_deployed = true;
        self.was_deployed = true;
        self.loc = Location::new(self.ship_loc.x, self.ship_loc.y, alt, t);
        self.hl.push(self.loc.clone());
        self.heading = head;
        self.speed = spd;
        self.altitude = alt;
        self.at = t;
        return true;
    }

    pub fn land(&mut self, ship_id: String, t:chrono::NaiveDateTime) -> bool {
        self.ship_id = ship_id;
        self.is_landing = true;
        return true;
    }

    /// Determine if the fighter can land on a carrier
    fn can_land(&self) -> bool {
        let distance = self.loc.distance(&self.ship_loc);
        return distance <= self.speed / 60.0;
    }

    /// Set the heading to the target carrier
    fn goto_carrier(&mut self) {
        let dx = self.ship_loc.x - self.loc.x;
        let dy = self.ship_loc.y - self.loc.y;
        self.heading = dy.atan2(dx).to_degrees();
    }
}

impl Movable for Fighter {
    fn get_is_deployed(&self) -> bool {
        return self.is_deployed;
    }
    fn get_was_deployed(&self) -> bool {
        return self.was_deployed;
    }
    fn get_id(&self) -> String {
        return self.id.clone();
    }
    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_location(&self) -> Location {
        return self.loc.clone();
    }
    fn get_history(&self) -> &HistoryList {
        return &self.hl;
    }
    /// determine which order we have received
    /// and call the function associated with it
    fn execute(&mut self, order: &Order) {
        let result = match order {
            Order::DeployAircraftOrder(o) => self.deploy(o.heading, o.speed, o.altitude, o.extime),
            Order::ChangeAircraftOrder(o) => self.change(o.heading, o.speed, o.altitude, o.extime),
            Order::LandAircraftOrder(o) => self.land(o.ship_id.clone(), o.extime),
            _ => false
        };
    }
    fn deploy(&mut self, x: f64, y: f64, head: f64, spd: f64, t: chrono::NaiveDateTime) -> bool {
        return false;
    }
    fn change(&mut self, head: f64, spd: f64, alt: f64, t: chrono::NaiveDateTime) -> bool {
        // self.update_position(t);
        if spd != -1.0 {
            self.speed = spd;
        }
        if alt != -1.0 {
            // loc.setZ(alt);
            self.altitude = alt;
        }
        if !self.is_landing && head != -1.0 {
            self.heading = head;
        }
        return true;
    }
    fn update_position(&mut self, t: chrono::NaiveDateTime, loc_map: &LocationMap) {
        self.ship_loc = loc_map.get(self.ship_id.as_str()).unwrap().clone();
        self.loc = calc_new_position(self.loc.clone(), self.heading, self.speed, t, self.at);
        self.loc.z = self.altitude;
        self.hl.push(self.loc.clone());
        self.at = t;

        if self.is_landing {
            if self.can_land() {
                self.is_deployed = false;
                self.is_landing = false;
                self.altitude = 0.0;
            }
            else {
                self.goto_carrier();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cruiser_new() {
        let a = Cruiser::new(String::from("Chelsey"), String::from("I264"), 12.0, 30);
        a.print();
    }

    #[test]
    fn test_carrier_new() {
        let a = Carrier::new(String::from("Gertrude"), String::from("P131"), 25.0, 15);
        a.print();
    }

    #[test]
    fn test_fighter_new() {
        let a = Fighter::new(String::from("Brunhilde"), String::from("G264"), 500.0, String::from("P131"), 100000.0, 20);
        a.print();
    }

    #[test]
    fn test_execute_order() {
        let mut a = Box::new(Cruiser::new(String::from("Chelsey"), String::from("I264"), 12.0, 30));
        a.print();
        let atime = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let op = DeployShip::new(atime, String::from("CGN-39"), 0.0, 0.0, 0.0, 0.0);
        let order = Order::DeployShipOrder(op);
        a.execute(&order);
    }
}
