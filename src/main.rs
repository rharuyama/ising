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
	    if 40 <= i && i < 60 && 40 <= j && j < 60 {
		rng.gen_range(0..2)
	    } else {
		// 0
		rng.gen_range(0..2)
	    }
	}).collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update view per 50 times loop
    for _t in 0..500 {
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

    grid::grid(&draw, &model);
    
    draw.to_frame(app, &frame).unwrap();
}

