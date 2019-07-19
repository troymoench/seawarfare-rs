use std::collections::HashMap;


pub type LocationMap = HashMap<String, Location>;

#[derive(Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    t: chrono::NaiveDateTime
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64, t: chrono::NaiveDateTime) -> Location {
        Location {x, y, z, t}
    }
    pub fn new2(x: f64, y: f64, t: chrono::NaiveDateTime) -> Location {
        Location::new(x, y, 0.0, t)
    }
    pub fn print(&self) {
        println!("({:.2}, {:.2}, {:.2}) t: {}", self.x, self.y, self.z, self.t);
    }
    pub fn get_xyz(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Calculate the 2D distance between two locations
    /// using the Pythagorean Theorem
    pub fn distance(&self, other: &Location) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return (dx.powi(2) + dy.powi(2)).sqrt();
    }
}

impl Clone for Location {
    fn clone(&self) -> Location {
        Location::new(self.x, self.y, self.z, self.t)
    }
}

impl Default for Location {
    fn default () -> Location {
        Location {x: 0.0, y: 0.0, z: 0.0, t: chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_new() {
        let t = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let l = Location::new(1.0, 2.0, 3.0, t);
        l.print()
    }

    #[test]
    fn test_location_new2() {
        let t = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let l = Location::new2(1.0, 2.0, t);
        l.print()
    }
}
