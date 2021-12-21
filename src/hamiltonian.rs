use ising::model::Model;

pub fn _hamiltonian(model: &Model) -> i32 {
    let mut ham: i32 = 0;
    for n in 0..10000 {
	if n % 100 == 99 && n / 100 == 99 {
	    ham += 0;
	} else if n % 100 == 99 {
	    ham += - interaction(
		model.state[n],
		model.state[n + 100]
	    );
	} else if n / 100 == 99 {
	    ham += - interaction(
		model.state[n],
		model.state[n + 1]
	    );
	} else {
	    ham += -interaction(
		model.state[n],
		model.state[n + 100]
	    ) - interaction(
		model.state[n],
		model.state[n + 1]
	    )
	}	
    }
    ham
}

pub fn d_hamiltonian(model: &Model, n: usize) -> i32 {
    let mut d_ham: i32 = 0;
    
    if n == 9999 { // upper right corner
	d_ham += - interaction(
	    model.state[n], 
	    model.state[n - 1] // left edge
	) - interaction( 
	    model.state[n],
	    model.state[n - 100] // down edge
	);
    } else if n == 99 { // down right corner
	d_ham += - interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n],
	    model.state[n - 1] // left edge
	);
    } else if n == 0 {
	d_ham += - interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	);
    } else if n == 9900 { // upper left corner
	d_ham += - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	);
    } else if n % 100 == 99 { // right side
	d_ham += - interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	);
    } else if n / 100 == 99 { // upper side
	d_ham += - interaction(
	    model.state[n + 1],
	    model.state[n] // left edge
	) - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	);
    } else if n % 100 == 0 { // left side
	d_ham += - interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	) - interaction(
	    model.state[n],
	    model.state[n - 100] // down edge
	);
    } else if n / 100 == 0 { // down side
	d_ham += - interaction(
	    model.state[n],
	    model.state[n - 1] // left edge
	) - interaction(
	    model.state[n + 100],
	    model.state[n] // upper edge
	) - interaction(
	    model.state[n + 1],
	    model.state[n] // right edge
	);
    } else {
	d_ham += - interaction(
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
	);
    }
    d_ham
}

// energy
fn interaction(xi: u8, xj: u8) -> i32 {
    if xi == 0 && xj == 0 {
	0
    } else if xi == 0 && xj == 1 {
	1
    } else if xi == 1 && xj == 0 {
	0
    } else {
	0
    }
}
