mod grid;
mod hamiltonian;
use crate::hamiltonian::d_hamiltonian;
use ising::model::Model;
use nannou::prelude::*;
use rand::Rng;
use std::f64;

fn main() {
    nannou::app(model)
	.update(update)
	.simple_window(view)
	.size(900, 600).
	run();
}

fn model(_app: &App) -> Model {
    let mut rng = rand::thread_rng();
    Model {
	state: (0..10000).map(|n| {
	    let i = n % 100;
	    let j = n / 100;
	    if 48 <= i && i < 52 && 48 <= j && j < 52 {
		rng.gen_range(0..2)
	    } else {
		// 0
		rng.gen_range(0..2)
	    }
	}).collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update view per n times loop
    let n = 500;
    for _t in 0..n {
	let mut rng = rand::thread_rng();
	let n = rng.gen_range(0..10000);
	let d_ham = d_hamiltonian(model, n);
	let p: f64 = rng.gen();
	let tempreture: f64 = 20.0;

	let energy_average = -2;
	let e = d_ham - energy_average; 
	if e <= 0 { // then flip
	    model.state[n] = if model.state[n] == 0 { 1 } else { 0 };
	} else if 0 < e
	    && (- e as f64 * (1.0 / tempreture)).exp() <= p {
	    model.state[n] = if model.state[n] == 0 { 1 } else { 0 };
	} else {
	    
	}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();
    draw.background().color(WHITE);   

    grid::grid(&draw, &model);
    
    draw.to_frame(app, &frame).unwrap();
}

