mod grid;
// mod hamiltonian;
// use hamiltonian::d_hamiltonian;
use ising::Model;
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
	weight_0_0: 0,
	weight_0_1: 1,
	weight_1_0: 1,
	weight_1_1: 0,
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
	temperature: 0.1, // 0.2 ~ 0.5, for example
	update_speed: 5000,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for _a in 0..model.update_speed {
	let mut rng = rand::thread_rng();
	let n = rng.gen_range(0..10000);
	let d_ham = Model::d_hamiltonian(model, n);
	let p: f64 = rng.gen();
	let e = d_ham - (- model.energy_average());
	
	if e <= 0 { // then flip
	    model.state[n] = if model.state[n] == 0 { 1 } else { 0 };
	} else if 0 < e
	    && p <= (- e as f64 * (1.0 / model.temperature)).exp() { // noise
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

