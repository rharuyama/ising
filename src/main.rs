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
	state: (0..10000).map(|_| rng.gen_range(0..2)).collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update view per 50 times loop
    for _t in 0..50 {
	let mut rng = rand::thread_rng();
	let n = rng.gen_range(0..10000);
	let d_ham = d_hamiltonian(model, n);
	let p: f64 = rng.gen();
	
	if d_ham <= -2 { // then flip
	    model.state[n] = if model.state[n] == 0 { 1 } else { 0 };
	} else if -2 < d_ham && (-d_ham as f64).exp() <= p {
	    println!("botz: {}", (-d_ham as f64).exp());
	    println!("p: {}", p);
	    model.state[n] = if model.state[n] == 0 { 1 } else { 0 };
	} else {
	    //	println!("else");
	}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();
    draw.background().color(WHITE);
    for i in 0..100 {
	for j in 0..100 {
	    if i < 75 && j < 25 {
		model.state[j + i * 100] = 1;
	    }
	}
    }

    grid::grid(&draw, &model);
    
    draw.to_frame(app, &frame).unwrap();
}

