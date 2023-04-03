use nannou::prelude::*;
use nannou::event::WindowEvent;

use rand::Rng;        // RNG traits
use rand::thread_rng; // to return an instance of a RNG

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

struct Model {
    win_id: window::Id,
    win_width: u32,
    win_height: u32,
    fireworks: Vec<Firework>,
    particles: Vec<Particle>,
    timer_total: u8,
    timer_tick: u8
}

fn model(app: &App) -> Model {
    let win_width: u32 = 1024;
    let win_height: u32 = 768;

    let win_id = app.new_window()
        .size(win_width, win_height)
        .title("Fireworks")
        .build()
        .unwrap();

    // our model stores program states as such the
    // current particles and fireworks
    return Model {
        win_id,
        win_width,   
        win_height,     
        fireworks: Vec::new(),
        particles: Vec::new(),
        timer_total: 5,
        timer_tick: 0
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {  
}

fn event(app: &App, model: &mut Model, event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

}
