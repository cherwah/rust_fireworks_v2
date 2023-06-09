use crate::Model;
use crate::particle::Particle;

use nannou::prelude::*;
use rand::Rng;
use core::f32;


pub struct Firework {
    // actual coordinates
    x: f32,
    y: f32,
    // starting coordinates
    sx: f32,
    sy: f32,
    // target coordinates
    tx: f32,
    ty: f32,
    // distance from starting point to target
    dist_to_target: f32,
    dist_traveled: f32,
    // track the past coordinates of each firework to create a trail effect, 
    // increase the coordinate count to create more prominent trails
    trail_path: Vec<[f32; 2]>,
    // populate initial coordinate collection with the current coordinates
    angle: f32,
    speed: f32,
    acceleration: f32,
    brightness: f32,
    hue: f32,
    // circle target indicator radius
    target_radius: f32 
}

impl Firework {
    // create firework
    pub fn new(sx: f32, sy: f32, tx: f32, ty: f32, trail_len: i32) -> Self {
        // distance for firework to reach target point
        let dist_to_target:f32 = Firework::compute_dist(sx, sy, tx, ty);
        
        // determine the angle to shoot firework to target point
        let x_diff = tx - sx;
        let y_diff = ty - sy;
        
        // store last k coordinates to simulate a trail
        // to hit the pulsing circle
        let mut trail_path:Vec<[f32; 2]> = Vec::new();

        for _ in 0..trail_len {
            trail_path.push([sx, sy]);
        }

        let mut rng = rand::thread_rng();

        return Firework {
            x: sx, y:sy, 
            sx, sy, tx, ty,
            dist_to_target,
            dist_traveled: 0.0,
            trail_path,
            angle: y_diff.atan2(x_diff),
            speed: 2.0,
            acceleration: 1.05,
            brightness: rng.gen_range(0.5..0.7),
            hue: rng.gen_range(0.0..1.0),
            target_radius: 1.0
        }
    }

    pub fn update(model:&mut Model) 
    {
        let mut i = model.fireworks.len();
        
        while i > 0 {      
            i -= 1;  

            let firework = &mut model.fireworks[i];

            // remove last item in trail buffer
            firework.trail_path.pop();
            
            // add current coordinates to the start of the trail buffer
            firework.trail_path.insert(0, [firework.x, firework.y]);
            
            // cycle the circle target indicator radius
            if firework.target_radius < 8.0 {
                firework.target_radius += 0.3;
            } else {
                firework.target_radius = 1.0;
            }        
            
            // speed up the firework to reach the pulsing target at each update interval
            firework.speed *= firework.acceleration;

            // get x/y components based on angle and speed
            let vx = firework.angle.cos() * firework.speed;
            let vy = firework.angle.sin() * firework.speed;

            // determine how far have the firework has traveled with velocities applied
            firework.dist_traveled = Firework::compute_dist( firework.sx, firework.sy,
                firework.x + vx, firework.y + vy);

            // if the distance traveled is greater than the initial distance to target, 
            // then target has been reached
            if firework.dist_traveled >= firework.dist_to_target {
                Particle::create(firework.tx, firework.ty, 100, firework.hue, model);
                model.fireworks.remove(i);
            } else {
                // target not reached, keep traveling
                firework.x += vx;
                firework.y += vy;
            }
        }
    }

    pub fn draw(draw:&Draw, i: usize, model: &Model) {
        let firework = &model.fireworks[i];
        
        // move to last tracked coordinate in the set, then draw a line  to current 
        // (x, y) of firework to simulate a trail path. trail_path[0] contains the 
        // earliest unprocessed (x, y) for the firework.
        let last = firework.trail_path.len() - 1;
        draw.line()
            .start(pt2(firework.trail_path[last][0], 
                       firework.trail_path[last][1]))
            .end(pt2(firework.x, firework.y))
            .weight(1.0)
            .hsla(firework.hue, 1.0, firework.brightness, 0.5);
    
        // draw the target for this firework with a pulsing circle
        draw.ellipse()
            .x_y(firework.tx, firework.ty)
            .no_fill()
            .radius(firework.target_radius)
            .stroke_weight(1.0)
            .stroke(hsla(firework.hue, 1.0, firework.brightness, 0.7));
    }

    pub fn spawn(model: &mut Model) {
        let mut rng = rand::thread_rng();

        let left_min = -(model.win_width as f32 / 3.0);
        let right_max = model.win_width as f32 / 3.0;

        // firework to explode at (x, y) position
        Firework::create(
            rng.gen_range(left_min..right_max), // random x position within given limits
            rng.gen_range(0.0..model.win_height as f32/ 3.0),  // top one-third of the screen
            model);        
    }

    pub fn create(x: f32, y: f32, model: &mut Model) {
        // nannou's origin (0, 0) is at center of screen
        // firework shoots out at (sx, sy) position
        model.fireworks.push(Firework::new(
            0.0,                                // center of screen
            -((model.win_height >> 1) as f32),  // bottom of screen
            x,
            y,  // upper part of the screen  
            3  // trail_len
        ));
    }

    // calculate euclidean distance
    pub fn compute_dist(p1x: f32, p1y: f32, p2x: f32, p2y: f32) -> f32 {
        let x_dist = p1x - p2x;
        let y_dist = p1y - p2y;

        let sq_dist = f32::powf(x_dist, 2.0) + f32::powf(y_dist, 2.0);
        return (sq_dist as f32).sqrt();
    }

}


