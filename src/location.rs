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
    // atime
}

impl Location {
    pub fn new(x: i64, y: i64, z: i64) -> Location {
        Location {x, y, z}
    }
    pub fn new2(x: i64, y: i64) -> Location {
        Location::new(x, y, 0)
    }
    pub fn print(&self) {
        println!("({}, {}, {})", self.x, self.y, self.z);
    }
    pub fn get_xyz(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}

impl Clone for Location {
    fn clone(&self) -> Location {
        Location::new(self.x, self.y, self.z)
    }
}
