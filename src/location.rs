/*
class Location {
public:
	Location();
	Location(double, double, ATime);
	Location(double, double, double, ATime);
	void getXY(double& x_pos, double& y_pos) const;
	void getXYZ(double& x_pos, double& y_pos, double& z_pos) const;
	void setZ(double z_pos) { z = z_pos; }
	void print();
private:
	double x;
	double y;
	double z;
	ATime t;
};
*/

#[derive(Debug)]
pub struct Location {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    t: chrono::NaiveDateTime
}

impl Location {
    pub fn new(x: i64, y: i64, z: i64, t: chrono::NaiveDateTime) -> Location {
        Location {x, y, z, t}
    }
    pub fn new2(x: i64, y: i64, t: chrono::NaiveDateTime) -> Location {
        Location::new(x, y, 0, t)
    }
    pub fn print(&self) {
        println!("({}, {}, {}) t: {}", self.x, self.y, self.z, self.t);
    }
    pub fn get_xyz(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}

impl Clone for Location {
    fn clone(&self) -> Location {
        Location::new(self.x, self.y, self.z, self.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_new() {
        let t = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let l = Location::new(1, 2, 3, t);
        l.print()
    }

    #[test]
    fn test_location_new2() {
        let t = chrono::NaiveDate::from_ymd(2015, 10, 21).and_hms(17, 2, 0);
        let l = Location::new2(1, 2, t);
        l.print()
    }
}
