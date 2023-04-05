use crate::Model;
use rand::Rng;
use rand::rngs::ThreadRng;
use nannou::prelude::*;


pub struct Particle {
    x: f32,
    y: f32,
    // track the past coordinates of each particle to create a trail effect, increase 
    // the coordinate count to create more prominent trails
    coords: Vec<[f32; 2]>,
    // set a random angle in all possible directions, in radians
    angle: f32,
    speed: f32,
	// friction will slow the particle down
	friction: f32,
	// gravity will be applied and pull the particle down
	gravity: f32,
    // set the hue to a random number +-50 of the overall hue variable
    hue: f32,
    brightness: f32,
    alpha: f32,
    // set how fast the particle fades out
    decay: f32    
}

impl Particle {
    pub fn new(x: f32, y: f32, hue: f32, trail_len: i32, rng:&mut ThreadRng) -> Particle {
        // track the past coordinates of each particle to create a trail effect, 
        // increase the coordinate count to create more prominent trails
        let mut coords:Vec<[f32; 2]> = Vec::new();
        for _ in 0..trail_len {
            coords.push([x, y])
        }

        // hue ranges from 0 (0 degree) to 1.0 (360 degree)
        let hue_swing = 0.15;

        return Particle {
            x: x,
            y: y,
            coords: coords,
            angle: rng.gen_range(0.0..std::f32::consts::PI * 2.0),
            speed: rng.gen_range(1.0..10.0),
            friction: 0.95,
            gravity: 1.0,
            hue: rng.gen_range((hue - hue_swing)..(hue + hue_swing)),
            brightness: rng.gen_range(0.5..0.8),
            alpha: 1.0,
            decay: rng.gen_range(0.015..0.03)
        }
    }

    pub fn draw(draw:&Draw, i:usize, model:&Model) {
        let particle = &model.particles[i];
    
        let last = particle.coords.len() - 1;
        draw.line()
            .start(pt2(particle.coords[last][0], particle.coords[last][1]))
            .end(pt2(particle.x, particle.y))
            .weight(1.0)
            .hsla(particle.hue, 1.0, particle.brightness, particle.alpha);
    }

    pub fn update(model: &mut Model) {
        let mut i = model.particles.len();

        while i > 0 {      
            i -= 1;  

            let particle = &mut model.particles[i];
    
            // remove last item in coordinates array
            particle.coords.pop();
            // add current coordinates to the start of the array
            particle.coords.insert(0, [particle.x, particle.y]);
            // slow down the particle
            particle.speed *= particle.friction;    
            // // apply velocity
            particle.x += particle.angle.cos() * particle.speed;
            particle.y += particle.angle.sin() * particle.speed - particle.gravity;
    
            // fade out the particle
            particle.alpha -= particle.decay;
            
            // remove the particle once the alpha is low enough
            if particle.alpha <= particle.decay {
                model.particles.remove(i);
            }
        }
    }

    pub fn create(x: f32, y: f32, n_particles: i32, model: &mut Model) {
        for _ in 0..n_particles {
            model.particles.push(Particle::new(x, y, model.hue, 5, &mut model.rng));
        }
    }
}
