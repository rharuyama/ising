use ising::model::Model;

pub fn d_hamiltonian(model: &Model, n: usize) -> i32 {
    if n == 9999 { // upper right corner
	- interaction(
	    model.state[n], 
	    model.state[n - 1] // left edge
	) - interaction( 
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    } else if n == 99 { // down right corner
	- interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n],
	    model.state[n - 1] // left edge
	)
    } else if n == 0 {
	- interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else if n == 9900 { // upper left corner
	- interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else if n % 100 == 99 { // right side
	- interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    } else if n / 100 == 99 { // upper side
	- interaction(
	    model.state[n + 1],
	    model.state[n] // left edge
	) - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else if n % 100 == 0 { // left side
	- interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	) - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    } else if n / 100 == 0 { // down side
	- interaction(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else {
	- interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	) - interaction(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    }
}

// energy
fn interaction(xi: u8, xj: u8) -> i32 {
    if xi == 0 && xj == 0 {
	0
    } else if xi == 0 && xj == 1 {
	1
    } else if xi == 1 && xj == 0 {
	1
    } else {
	0
    }
}
