use ising::Model;

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
