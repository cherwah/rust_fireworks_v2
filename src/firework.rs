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
    coords: Vec<[f32; 2]>,
    // populate initial coordinate collection with the current coordinates
    angle: f32,
    speed: f32,
    acceleration: f32,
    _hue: f32,
    brightness: f32,
    // circle target indicator radius
    target_radius: f32 
}

impl Firework {
    // create firework
    pub fn new(sx: f32, sy: f32, tx: f32, ty: f32, trail_len: i32, 
        hue: f32, rng: &mut ThreadRng) -> Self {
        // distance for firework to reach target point
        let dist_to_target:f32 = compute_dist(sx, sy, tx, ty);
        
        // determine the angle to shoot firework to target point
        let x_diff = tx - sx;
        let y_diff = ty - sy;
        
        // store last 3 coordinates to simulate a trail
        // to hit the pulsing circle
        let mut coords:Vec<[f32; 2]> = Vec::new();

        for _ in 0..trail_len {
            coords.push([sx, sy]);
        }

        let hue_swing = 0.15;

        return Firework {
            x: sx, 
            y: sy, 
            sx: sx, 
            sy: sy, 
            tx: tx, 
            ty: ty,
            dist_to_target: dist_to_target,
            dist_traveled: 0.0,
            coords: coords,            
            angle: y_diff.atan2(x_diff),
            speed: 2.0,
            acceleration: 1.05,
            _hue: rng.gen_range((hue - hue_swing)..(hue + hue_swing)),
            brightness: rng.gen_range(0.5..0.7),
            target_radius: 1.0
        }
    }

    fn update(&mut self, &mut model:Model) 
    {
        let firework = &mut model.fireworks[i];

        // remove last item in coordinates array
        firework.coords.pop();
        
        // add current coordinates to the start of the array
        firework.coords.insert(0, [firework.x, firework.y]);
        
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
        firework.dist_traveled = compute_dist( firework.sx, firework.sy,
            firework.x + vx, firework.y + vy);

        // if the distance traveled is greater than the initial distance to target, 
        // then target has been reached
        if firework.dist_traveled >= firework.dist_to_target {
            create_particles(firework.tx, firework.ty, 100, model);
            model.fireworks.remove(i);
        } else {
            // target not reached, keep traveling
            firework.x += vx;
            firework.y += vy;
        }
    }
}

