mod utils;

use crate::utils::color_from_hsv;
use noise::{NoiseFn, Perlin};
use rand::{Rng, rng};
use raylib::prelude::*;
use std::f32::consts::PI;

pub const SCRN_WIDTH: usize = 900;
pub const SCRN_HEIGHT: usize = 900;
pub const ROW_COUNT: usize = 50;
pub const COL_COUNT: usize = 50;
pub const TILE_WIDTH: usize = SCRN_WIDTH / ROW_COUNT;
pub const TILE_HEIGHT: usize = SCRN_HEIGHT / COL_COUNT;

pub const FIELD_LEN: usize = ROW_COUNT * COL_COUNT;
pub const PARTICLES_COUNT: usize = 100;

pub struct ParticlesSOA {
    positions: [Vector2; PARTICLES_COUNT],
    velocities: [Vector2; PARTICLES_COUNT],
    accelerations: [Vector2; PARTICLES_COUNT],
}

impl ParticlesSOA {
    pub fn new() -> Self {
        let mut positions = [Vector2::zero(); PARTICLES_COUNT];
        let velocities = [Vector2::zero(); PARTICLES_COUNT];
        let accelerations = [Vector2::zero(); PARTICLES_COUNT];

        let mut r = rng();

        for i in 0..PARTICLES_COUNT {
            let frac_x: f32 = r.random();
            let frac_y: f32 = r.random();
            positions[i] = Vector2 {
                x: frac_x * (SCRN_WIDTH as f32),
                y: frac_y * (SCRN_HEIGHT as f32),
            };
        }

        Self {
            positions: positions,
            velocities: velocities,
            accelerations: accelerations,
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCRN_WIDTH as i32, SCRN_HEIGHT as i32)
        .msaa_4x()
        .title("Kesh")
        .build();

    let mut force_field: [Vector2; FIELD_LEN] = [Vector2::zero(); FIELD_LEN];
    let seed: u32 = rng().random();
    let perlin = Perlin::new(seed);
    let mut zoff: f64 = 0.0;
    update_force_field(&mut force_field, &perlin, zoff);

    let mut particles = ParticlesSOA::new();
    let mut hue = 0.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // NOTE: Uncomment if you want to see the vector field
        //
        // d.clear_background(Color::RAYWHITE);
        // for y in 0..ROW_COUNT {
        //     for x in 0..COL_COUNT {
        //         let start = Vector2 {
        //             x: (x * TILE_WIDTH) as f32,
        //             y: (y * TILE_HEIGHT) as f32,
        //         };

        //         let index = x + y * ROW_COUNT;
        //         let end = force_field[index] * 0.1;
        //         let end = Vector2 {
        //             x: end.x * TILE_WIDTH as f32,
        //             y: end.y * TILE_HEIGHT as f32,
        //         };
        //         let end = start + end;

        //         d.draw_line_v(start, end, Color::BLACK);
        //         d.draw_circle_v(end, 2.0, Color::BLACK);
        //     }
        // }

        if zoff == 0.0 {
            d.clear_background(Color::BLACK);
        }

        // let color = color_from_hsv(hue, 1.0, 1.0);
        for i in 0..PARTICLES_COUNT {
            d.draw_circle_v(particles.positions[i], 0.5, Color::RED.alpha(0.2));
            // d.draw_circle_v(particles.positions[i], 0.5, color);
        }

        // let fps = d.get_fps();
        // println!("{fps}");

        update_force_field(&mut force_field, &perlin, zoff);
        update_particles(&mut particles, &force_field);

        const DZ: f64 = 0.01;
        zoff += DZ;
        hue += 0.1;
    }
}

fn update_particles(particles: &mut ParticlesSOA, force_field: &[Vector2; FIELD_LEN]) {
    let dt = 0.1;

    for i in 0..PARTICLES_COUNT {
        let pos = particles.positions[i];
        let x = (pos.x / SCRN_WIDTH as f32) * ((ROW_COUNT - 1) as f32);
        let y = (pos.y / SCRN_HEIGHT as f32) * ((COL_COUNT - 1) as f32);
        let index = (x + y * ROW_COUNT as f32) as usize;
        particles.accelerations[i] += force_field[index] * dt;

        let acc = particles.accelerations[i];
        particles.velocities[i] += acc * dt;
        if particles.velocities[i].length() >= 10.0 {
            particles.velocities[i] = particles.velocities[i].normalized() * 10.0;
        }

        let vel = particles.velocities[i];
        particles.positions[i] += vel * dt;
        particles.positions[i] = Vector2 {
            x: if particles.positions[i].x < 0.0 {
                SCRN_WIDTH as f32
            } else if particles.positions[i].x >= SCRN_WIDTH as f32 {
                0.0
            } else {
                particles.positions[i].x
            },
            y: if particles.positions[i].y < 0.0 {
                SCRN_HEIGHT as f32
            } else if particles.positions[i].y >= SCRN_HEIGHT as f32 {
                0.0
            } else {
                particles.positions[i].y
            },
        };

        particles.accelerations[i] = Vector2::zero();
    }
}

fn update_force_field(field: &mut [Vector2; FIELD_LEN], perlin: &Perlin, z_off: f64) {
    const DX: f64 = 0.01;
    const DY: f64 = 0.01;

    let mut y_off = 0.0;
    for y in 0..ROW_COUNT {
        let mut x_off = 0.0;
        for x in 0..COL_COUNT {
            let noise_val = perlin.get([x_off, y_off, z_off]) as f32;
            let noise_angle = noise_val * 2.0 * PI;
            let vector = Vector2::one().rotated(noise_angle) * 5.0;

            let index = x + y * ROW_COUNT;
            field[index] = vector;
            x_off += DX;
        }
        y_off += DY;
    }
}
