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
    pub fn new(weight_0_0: i32, weight_0_1: i32, weight_1_0: i32, weight_1_1: i32,
    state: Vec<u8>, temperature: f64, update_speed: i32) -> Model {
	Model { weight_0_0: weight_0_0,
		weight_0_1: weight_0_1,
		weight_1_0: weight_1_0,
		weight_1_1: weight_1_1,
	state: state, temperature: temperature, update_speed: update_speed }
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

    pub fn d_hamiltonian(model: &Model, n: usize) -> i32 {
	if n == 9999 { // upper right corner
	    - model.interaction_energy(
		n,
		n - 1
	    ) - model.interaction_energy( 
		n,
		n - 100
	    )
	} else if n == 99 { // down right corner
	    - model.interaction_energy(
		n + 100,
		n
	    ) - model.interaction_energy(
		n,
		n - 1
	    )
	} else if n == 0 {
	    - model.interaction_energy(
		n + 100,
		n
	    ) - model.interaction_energy(
		n + 1,
		n
	    )
	} else if n == 9900 { // upper left corner
	    - model.interaction_energy(
		n,
		n - 100
	    ) - model.interaction_energy(
		n + 1,
		n
	    )
	} else if n % 100 == 99 { // right side
	    - model.interaction_energy(
		n + 100,
		n
	    ) - model.interaction_energy(
		n,
		n - 1
	    ) - model.interaction_energy(
		n,
		n - 100
	    )
	} else if n / 100 == 99 { // upper side
	    - model.interaction_energy(
		n + 1,
		n
	    ) - model.interaction_energy(
		n,
		n - 100
	    ) - model.interaction_energy(
		n + 1,
		n
	    )
	} else if n % 100 == 0 { // left side
	    - model.interaction_energy(
		n + 100,
		n
	    ) - model.interaction_energy(
		n + 1,
		n
	    ) - model.interaction_energy(
		n,
		n - 100
	    )
	} else if n / 100 == 0 { // down side
	    - model.interaction_energy(
		n,
		n - 1
	    ) - model.interaction_energy(
		n + 100,
		n
	    ) - model.interaction_energy(
		n + 1,
		n
	    )
	} else {
	    - model.interaction_energy(
		n + 100,
		n
	    ) - model.interaction_energy(
		n + 1,
		n
	    ) - model.interaction_energy(
		n,
		n - 1
	    ) - model.interaction_energy(
		n,
		n - 100
	    )
	}
}
}
