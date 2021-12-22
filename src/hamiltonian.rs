use ising::Model;

pub fn d_hamiltonian(model: &Model, n: usize) -> i32 {
    if n == 9999 { // upper right corner
	- model.ie.interaction_energy(
	    model.state[n], 
	    model.state[n - 1] // left edge
	) - model.ie.interaction_energy( 
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    } else if n == 99 { // down right corner
	- model.ie.interaction_energy(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 1] // left edge
	)
    } else if n == 0 {
	- model.ie.interaction_energy(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - model.ie.interaction_energy(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else if n == 9900 { // upper left corner
	- model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 100] // down edge
	) - model.ie.interaction_energy(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else if n % 100 == 99 { // right side
	- model.ie.interaction_energy(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    } else if n / 100 == 99 { // upper side
	- model.ie.interaction_energy(
	    model.state[n + 1],
	    model.state[n] // left edge
	) - model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 100] // down edge
	) - model.ie.interaction_energy(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else if n % 100 == 0 { // left side
	- model.ie.interaction_energy(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - model.ie.interaction_energy(
	    model.state[n + 1],
	    model.state[n] // right edge
	) - model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    } else if n / 100 == 0 { // down side
	- model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - model.ie.interaction_energy(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - model.ie.interaction_energy(
	    model.state[n + 1],
	    model.state[n] // right edge
	)
    } else {
	- model.ie.interaction_energy(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - model.ie.interaction_energy(
	    model.state[n + 1],
	    model.state[n] // right edge
	) - model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - model.ie.interaction_energy(
	    model.state[n],
	    model.state[n - 100] // down edge
	)
    }
}
