#![allow(dead_code)]

use std::time::SystemTime;
use macroquad::prelude::*;
use macroquad::rand::gen_range;


#[macroquad::main(window_conf)]
async fn main() {
    let win = window_conf();
    let mut system = ParticleSystem::new()
        .position(vec2(win.window_width as f32 / 2., win.window_height as f32 / 2.))
        .initial_velocity(vec2(0., -200.));

    let time = SystemTime::now();
    let mut prev_time = time.elapsed().unwrap().as_secs_f32();
    let mut mouse_ctrl = false;

    // App loop
    loop {
        clear_background(BLACK);

        // Delta time for use in 'ticking' methods
        let delta = time.elapsed().unwrap().as_secs_f32() - prev_time;

        // Handle Input
        if is_mouse_button_pressed(MouseButton::Left) { mouse_ctrl = !mouse_ctrl }

        // Move ParticleSystem
        let mut position = if mouse_ctrl {
            let mouse_pos = mouse_position();
            vec2(
                mouse_pos.0,
                mouse_pos.1
            )
        } else {
            vec2(
                (prev_time * 5.).sin() * 200. + win.window_width as f32 / 2.,
                (prev_time * 10.).sin() * 100. + win.window_height as f32 / 2.
            )
        };
        system.position = position;

        // Particle handling
        system.tick(delta);
        system.draw();

        prev_time = time.elapsed().unwrap().as_secs_f32();
        next_frame().await;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("title"),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}


/// System with the sole purpose of handling Particles
struct ParticleSystem {
    position: Vec2,
    gravity: Vec2,
    emit_interval: f32,
    initial_velocity: Vec2,
    randomize: bool,
    rand_amount: f32,
    particles: Vec<Particle>,
    _interval_timer: f32,
}

impl ParticleSystem {
    /// Return a newly created `ParticleSystem`
    fn new() -> Self {
        ParticleSystem {
            position: vec2(0., 0.),
            gravity: vec2(0., 100.),
            emit_interval: 0.001,
            initial_velocity: vec2(0., 0.),
            randomize: true,
            rand_amount: 50.,
            particles: vec![],
            _interval_timer: 0.001,
        }
    }

    /// Tick through every particle in `particles`
    fn tick(self: &mut Self, delta: f32) {
        self._interval_timer -= delta;
        if self._interval_timer < 0. {
            self._interval_timer = self.emit_interval;

            let mut x_vel = 0.;
            let mut y_vel = 0.;
            if self.randomize {
                x_vel = gen_range(-self.rand_amount, self.rand_amount);
                y_vel = gen_range(-self.rand_amount * 2., self.rand_amount * 2.);
            }

            self.particles.push(Particle::new()
                .position(self.position)
                .velocity(vec2(
                    x_vel + self.initial_velocity.x,
                    y_vel + self.initial_velocity.y
                )));
        }

        // Create a buffer for removed particles
        // (to avoid modifying the length of `particles` while iterating)
        let mut rem_buffer: Vec<usize> = vec![];
        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];

            particle.velocity += self.gravity * 5. * delta;
            particle.position += particle.velocity * delta;

            particle.radius -= particle.decay_rate * delta;
            if particle.radius <= 0. {
                rem_buffer.push(i);
            }
        }
        for idx in rem_buffer {
            self.particles.remove(idx);
        }
    }

    /// Draw every particle in `particles`
    fn draw(self: &mut Self) {
        for particle in &self.particles {
            draw_circle(
                particle.position.x,
                particle.position.y,
                particle.radius,
                WHITE
            );
        }
    }

    /// Set `position` to `value`.
    fn position(mut self: Self, value: Vec2) -> Self {
        self.position = value;
        return self;
    }
    
    /// Set `emit_interval` to `value`.
    fn emit_interval(mut self: Self, value: f32) -> Self {
        self.emit_interval = value;
        return self;
    }

    /// Set `initial_velocity` to `value`.
    fn initial_velocity(mut self: Self, value: Vec2) -> Self {
        self.initial_velocity = value;
        return self;
    }
    
    /// Set `particles` to `value`.
    fn particles(mut self: Self, value: Vec<Particle>) -> Self {
        self.particles = value;
        return self;
    }
}


/// Simple `Particle` for use in a `ParticleSystem`
struct Particle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    decay_rate: f32,
}

impl Particle {
    /// Return a newly created `Particle`
    fn new() -> Self {
        Particle {
            position: vec2(0., 0.),
            velocity: vec2(0., 0.),
            radius: 8.,
            decay_rate: 8.,
        }
    }

    /// Set `position` to `value`.
    fn position(mut self: Self, value: Vec2) -> Self {
        self.position = value;
        return self;
    }

    /// Set `velocity` to `value`.
    fn velocity(mut self: Self, value: Vec2) -> Self {
        self.velocity = value;
        return self;
    }
    
    /// Set `radius` to `value`.
    fn radius(mut self: Self, value: f32) -> Self {
        self.radius = value;
        return self;
    }

    /// Set `decay_rate` to `value`.
    fn decay_rate(mut self: Self, value: f32) -> Self {
        self.decay_rate = value;
        return self;
    }
}
