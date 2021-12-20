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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();
    draw.background().color(WHITE);
    let t = app.time;

    grid::grid(&draw, t, &model);
    
    draw.to_frame(app, &frame).unwrap();
}

