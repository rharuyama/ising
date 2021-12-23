pub struct Model {
    pub weight_0_0: i32,
    pub weight_0_1: i32,
    pub weight_1_0: i32,
    pub weight_1_1: i32,
    pub state: Vec<u8>,
    pub temperature: f64,
    pub update_speed: i32,
}

impl Model {
    pub fn new(weight_0_0: i32,
	       weight_0_1: i32,
	       weight_1_0: i32,
	       weight_1_1: i32,
	       state: Vec<u8>,
	       temperature: f64,
	       update_speed: i32) -> Model {
	
	Model { weight_0_0: weight_0_0,
		weight_0_1: weight_0_1,
		weight_1_0: weight_1_0,
		weight_1_1: weight_1_1,
		state: state,
		temperature: temperature,
		update_speed: update_speed }
    }

    pub fn interaction_energy(&self, xi: usize, xj: usize) -> i32 {
	if self.state[xi] == 0 && self.state[xj] == 0 {
	    self.weight_0_0
	} else if self.state[xi] == 0 && self.state[xj] == 1 {
	    self.weight_0_1
	} else if self.state[xi] == 1 && self.state[xj] == 0 {
	    self.weight_1_0
	} else {
	    self.weight_1_1
	}
    }

    pub fn energy_average(&self) -> i32 {
	self.weight_0_0 + self.weight_0_1 + self.weight_1_0 + self.weight_1_1
    }

    pub fn up(&self, n: usize) -> i32 {
	- self.interaction_energy(
	    n + 100,
	    n
	)
    }
	
    pub fn down(&self, n: usize) -> i32 {
	return 0;
	- self.interaction_energy(
	    n,
	    n - 100
	)
    }
	
    pub fn left(&self, n: usize) -> i32 {
	return 0;
	- self.interaction_energy(
	    n,
	    n - 1
	)
    }
	
    pub fn right(&self, n: usize) -> i32 {
	- self.interaction_energy(
	    n + 1,
	    n
	)
    }
	

    pub fn d_hamiltonian(&self, n: usize) -> i32 {
	if n == 9999 { // upper right corner
	    self.down(n) + self.left(n)
		
	} else if n == 99 { // down right corner
	    self.up(n) + self.left(n)
		
	} else if n == 0 {
	    self.up(n) + self.right(n)
		
	} else if n == 9900 { // upper left corner
	    self.down(n) + self.right(n)
	    
	} else if n % 100 == 99 { // right side
	    self.up(n) + self.left(n) + self.down(n)
		
	} else if n / 100 == 99 { // upper side
	    self.left(n) + self.down(n) + self.right(n)
		
	} else if n % 100 == 0 { // left side
	    self.up(n) + self.right(n) + self.down(n)
		
	} else if n / 100 == 0 { // down side
	    self.left(n) + self.up(n) + self.right(n)
		
	} else {
	    self.up(n) + self.down(n) + self.left(n) + self.right(n)
	}
    }
}
