use ising::model::Model;
use nannou::prelude::*;

pub fn grid(draw: &Draw, model: &Model) {
    for i in 0..100 {
	for j in 0..100 {
	    draw.rect()
		.x_y(-250.0 + i as f32 * 5.0, -250.0 + j as f32 * 5.0)
		.w_h(5.0, 5.0)
		.color(get_color(model.state[j + i * 100]));
	}
    }    
}

pub fn get_color(bin: u8) -> Rgb8 {
    if bin == 0 {
	WHITE
    } else {
	BLACK
    }
}
