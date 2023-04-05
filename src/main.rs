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
    _win_id: window::Id,        // window to render to
    win_width: u32,
    win_height: u32,
    fireworks: Vec<Firework>,   // firework collection
    particles: Vec<Particle>,   // particle collection
    timer_total: u8,            // total elasped time before reset
    timer_tick: u8,             // how much time has elasped
    hue: f32,                   // starting hue
    rng: ThreadRng
}

fn model(app: &App) -> Model {
    // dimension for our graphics window
    let win_width: u32 = 1024;
    let win_height: u32 = 768;

    // create a new graphics window
    let _win_id = app.new_window()
        .size(win_width, win_height)
        .title("Fireworks Demo")
        .build()
        .unwrap();

    // random generator
    let mut rng = rand::thread_rng();

    // our model stores program states as such the
    // current particles and fireworks
    return Model {
        _win_id,
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

// update states of the demo
fn update(_app: &App, model: &mut Model, _update: Update) {  
    Firework::update(model);
    Particle::update(model);

    // create new firework once timer_tick reaches limit
    if model.timer_tick >= model.timer_total {
        model.hue = model.rng.gen_range(0.0..=1.0);

        Firework::create(
            model.rng.gen_range(-((model.win_width / 3) as f32)..(model.win_width / 3) as f32),
            model.rng.gen_range(0..model.win_height / 3) as f32,
            model);
        
        // reset ticker
        model.timer_tick = 0;
    } else {
        // increment ticker towards timer_total
        model.timer_tick += 1;
    }    
}

// render view based on updated states (from update())
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgba(0.0, 0.0, 0.0, 0.5));

    let mut i = model.fireworks.len();
    while i > 0 {
        i -= 1;
        Firework::draw(&draw, i, model);
    }

    let mut i = model.particles.len();
    while i > 0 {
        i -= 1;
        Particle::draw(&draw, i, model);
    }

    draw.to_frame(app, &frame).unwrap();
}

// capture window-events like resizing and mouse-clicks
fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(window_event), .. } => {
            match window_event {
                WindowEvent::MousePressed(_) => {
                    Firework::create(app.mouse.x, app.mouse.y, model);
                }
                WindowEvent::Resized(new_size) => {
                    model.win_width = new_size.x as u32;
                    model.win_height = new_size.y as u32;                    
                }
                _ => (),
            }
        }
        _ => (),
    }
}
