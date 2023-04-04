use nannou::prelude::*;
use nannou::event::WindowEvent;

use rand::{Rng, rngs::ThreadRng};

mod firework;
mod particle;

use firework::*;
use particle::*;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .view(view)
        .run();
}

pub struct Model {
    win_width: u32,
    win_height: u32,
    fireworks: Vec<Firework>,
    particles: Vec<Particle>,
    timer_total: u8,
    timer_tick: u8,
    hue: f32,
    rng: ThreadRng
}

fn model(app: &App) -> Model {
    // dimension for our graphics window
    let win_width: u32 = 1024;
    let win_height: u32 = 768;

    // create a new graphics window
    app.new_window()
        .size(win_width, win_height)
        .title("Fireworks")
        .build()
        .unwrap();

    // random generator
    let mut rng = rand::thread_rng();

    // our model stores program states as such the
    // current particles and fireworks
    return Model {
        win_width,   
        win_height,         
        fireworks: Vec::new(),
        particles: Vec::new(),
        timer_total: 5,
        timer_tick: 0,
        hue: rng.gen_range(0.0..=1.0),
        rng
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {  
}

fn event(app: &App, model: &mut Model, event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

}
