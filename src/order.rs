use std::rc::Rc;
use crate::movable::*;

// class Order {
// public:
// 	Order();
// 	Order(ATime, std::string);
// 	bool operator==(const Order& o) const;
// 	bool operator<(const Order& o) const;
// 	bool operator>(const Order& o) const;
// 	bool operator!=(const Order& o) const;
// 	ATime exectime() const { return extime; }
// 	std::string get_id() const { return ID; }
// 	virtual bool Execute(Movable*, ATime) = 0;  // make pure virtual
// 	void print();
// protected:
// 	std::string Name;
// 	std::string ID;
// 	ATime extime;
// };


pub trait Order {
    fn get_id(&self) -> String;
    fn get_extime(&self) -> chrono::NaiveDateTime;
    // fn execute(&self, Movable*, chrono::NaiveDateTime) -> bool;
    fn print(&self) {
        println!("ID: {}, extime: {}", self.get_id(), self.get_extime());
    }
    fn equals(&self, other: &Order) -> bool {
        return self.get_extime() == other.get_extime()
    }
    fn lt(&self, other: &Order) -> bool {
        return self.get_extime() < other.get_extime()
    }
}

#[derive(Debug)]
pub struct DeployShip {
    id: String,
    extime: chrono::NaiveDateTime,
    start_x: f64,
    start_y: f64,
    heading: f64,
    speed: f64
}

impl DeployShip {
    pub fn new(a: chrono::NaiveDateTime, id: String, x: f64, y: f64, head: f64, spd: f64) -> Self {
        DeployShip {
            extime: a,
            id: id,
            start_x: x,
            start_y: y,
            heading: head,
            speed: spd
        }
    }
}

impl Order for DeployShip {
    fn get_id(&self) -> String {
        return self.id.clone()
    }

    fn get_extime(&self) -> chrono::NaiveDateTime {
        return self.extime.clone()
    }

    // fn execute(&self, Movable*, chrono::NaiveDateTime) -> bool
}

#[derive(Debug)]
pub struct DeployAircraft {
    id: String,
    extime: chrono::NaiveDateTime,
    heading: f64,
    speed: f64,
    altitude: f64
}

impl DeployAircraft {
    pub fn new(a: chrono::NaiveDateTime, id: String, head: f64, spd: f64, alt: f64) -> Self {
        DeployAircraft {
            extime: a,
            id: id,
            heading: head,
            speed: spd,
            altitude: alt
        }
    }
}

impl Order for DeployAircraft {
    fn get_id(&self) -> String {
        return self.id.clone()
    }

    fn get_extime(&self) -> chrono::NaiveDateTime {
        return self.extime.clone()
    }

    // fn execute(&self, Movable*, chrono::NaiveDateTime) -> bool
}

#[derive(Debug)]
pub struct ChangeShip {
    id: String,
    extime: chrono::NaiveDateTime,
    heading: f64,
    speed: f64,
}

impl ChangeShip {
    pub fn new(a: chrono::NaiveDateTime, id: String, head: f64, spd: f64) -> Self {
        ChangeShip {
            extime: a,
            id: id,
            heading: head,
            speed: spd,
        }
    }
}

impl Order for ChangeShip {
    fn get_id(&self) -> String {
        return self.id.clone()
    }

    fn get_extime(&self) -> chrono::NaiveDateTime {
        return self.extime.clone()
    }

    // fn execute(&self, Movable*, chrono::NaiveDateTime) -> bool
}

#[derive(Debug)]
pub struct ChangeAircraft {
    id: String,
    extime: chrono::NaiveDateTime,
    heading: f64,
    speed: f64,
    altitude: f64
}

impl ChangeAircraft {
    pub fn new(a: chrono::NaiveDateTime, id: String, head: f64, spd: f64, alt: f64) -> Self {
        ChangeAircraft {
            extime: a,
            id: id,
            heading: head,
            speed: spd,
            altitude: alt
        }
    }
}

impl Order for ChangeAircraft {
    fn get_id(&self) -> String {
        return self.id.clone()
    }

    fn get_extime(&self) -> chrono::NaiveDateTime {
        return self.extime.clone()
    }

    // fn execute(&self, Movable*, chrono::NaiveDateTime) -> bool
}

// #[derive(Debug)]
pub struct LandAircraft {
    id: String,
    extime: chrono::NaiveDateTime,
    ship_id: Rc<Movable>
}

impl LandAircraft {
    pub fn new(a: chrono::NaiveDateTime, id: String, ship_id: Rc<Movable>) -> Self {
        LandAircraft {
            extime: a,
            id: id,
            ship_id: ship_id
        }
    }
}

impl Order for LandAircraft {
    fn get_id(&self) -> String {
        return self.id.clone()
    }

    fn get_extime(&self) -> chrono::NaiveDateTime {
        return self.extime.clone()
    }

    // fn execute(&self, Movable*, chrono::NaiveDateTime) -> bool
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_ship_new() {
        let atime = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let a = DeployShip::new(atime, String::from("CGN-39"), 0.0, 0.0, 0.0, 0.0);
        a.print();
    }

    #[test]
    fn test_deploy_aircraft_new() {
        let atime = chrono::NaiveDate::from_ymd(2015, 11, 21).and_hms(17, 10, 0);
        let a = DeployAircraft::new(atime, String::from("FA18C_1"), 0.0, 0.0, 0.0);
        a.print();
    }

    #[test]
    fn test_change_ship_new() {
        let atime = chrono::NaiveDate::from_ymd(2015, 11, 21).and_hms(17, 12, 0);
        let a = ChangeShip::new(atime, String::from("CGN-39"), 0.0, 100.0);
        a.print();
    }

    #[test]
    fn test_change_aircraft_new() {
        let atime = chrono::NaiveDate::from_ymd(2015, 11, 21).and_hms(17, 13, 0);
        let a = ChangeAircraft::new(atime, String::from("FA18C_1"), 0.0, 500.0, -1.0);
        a.print();
    }

    #[test]
    fn test_land_aircraft_new() {
        let atime = chrono::NaiveDate::from_ymd(2015, 11, 21).and_hms(17, 13, 0);
        let ship: Rc<dyn Movable> = Rc::new(Carrier::new(String::from("USS_Nimitz"), String::from("CVN-68"), 85.0, 50));
        let a = LandAircraft::new(atime, String::from("FA18C_1"), ship);
        a.print();
    }

    #[test]
    fn test_order_equals() {
        let atime = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let a = DeployShip::new(atime, String::from("CGN-39"), 0.0, 0.0, 0.0, 0.0);
        let b = DeployShip::new(atime, String::from("CVN-68"), 1.0, 1.0, 0.0, 0.0);
        assert!(a.equals(&b));
    }

    #[test]
    fn test_order_equals_not() {
        let atime = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let a = DeployShip::new(atime, String::from("CGN-39"), 0.0, 0.0, 0.0, 0.0);
        let btime = atime - chrono::Duration::seconds(1);
        let b = DeployShip::new(btime, String::from("CVN-68"), 1.0, 1.0, 0.0, 0.0);
        assert!(!a.equals(&b));
    }
}
