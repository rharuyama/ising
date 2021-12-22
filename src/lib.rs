pub struct Energy {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

impl Energy {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> Energy {
	Energy { a: a, b: b, c:c, d:d }
    }

    pub fn interaction_energy(&self, xi: u8, xj: u8) -> i32 {
	if xi == 0 && xj == 0 {
	    self.a
	} else if xi == 0 && xj == 1 {
	    self.b
	} else if xi == 1 && xj == 0 {
	    self.c
	} else {
	    self.d
	}
    }

    pub fn energy_average(&self) -> i32 {
	self.a + self.b + self.c + self.d
    }
}


pub struct Model {
    pub state: Vec<u8>,
    pub ie: Energy,
    pub temperature: f64,
    pub update_speed: i32,
}
