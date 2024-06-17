use libm::{cosf, sinf};

use crate::{clear, print};

pub struct RotatingCube {
    a: f32,
    b: f32,
    c: f32,
    width: i32,
    height: i32,
    horizontal_offset: i32,
    k1: f32,
    z_buffer: [f32; 80 * 25],
    buffer: [char; 80 * 25],
    cube_width: f32,
    increment_speed: f32,
    distance_from_camera: i32,
}

impl RotatingCube {
    pub fn new() -> RotatingCube {
        RotatingCube {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            k1: 10.0,
            width: 80,
            height: 25,
            z_buffer: [0.0; 80 * 25],
            buffer: [' '; 80 * 25],
            horizontal_offset: 0,
            cube_width: 20.0,
            increment_speed: 9.0,
            distance_from_camera: 100,
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        return j * sinf(self.a) * sinf(self.b) * cosf(self.c) - k * cosf(self.a) * sinf(self.b) * cosf(self.c)
            + j * cosf(self.a) * sinf(self.c) + k * sinf(self.a) * sinf(self.c) + i * cosf(self.b) * cosf(self.c);
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        return j * cosf(self.a) * cosf(self.c) + k * sinf(self.a) * cosf(self.c) - j * sinf(self.a) * sinf(self.b) * sinf(self.c)
            + k * cosf(self.a) * sinf(self.b) * sinf(self.c) - i * cosf(self.b) * sinf(self.c);
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        return k * cosf(self.a) * cosf(self.b) - j * sinf(self.a) * cosf(self.b) + i * sinf(self.b);
    }

    fn calculate_surface(&mut self, cube_x: f32, cube_y: f32, cube_z: f32, ch: char) {
        let x: f32 = self.calculate_x(cube_x, cube_y, cube_z);
        let y: f32 = self.calculate_y(cube_x, cube_y, cube_z);
        let z: f32 = self.calculate_z(cube_x, cube_y, cube_z) + self.distance_from_camera as f32;

        let ooz: f32 = 1.0 / z;

        let xp: i32 = (self.width as f32 / 2.0 + self.horizontal_offset as f32 + self.k1 * ooz * x * 2.0) as i32;
        let yp: i32 = (self.height as f32 / 2.0 + self.k1 * ooz * y) as i32;

        let idx: i32 = xp + yp * self.width;
        if idx >= 0 && idx < self.width * self.height {
            if ooz > self.z_buffer[idx as usize] {
                self.z_buffer[idx as usize] = ooz;
                self.buffer[idx as usize] = ch;
            }
        }
    }

    // I don't know what's wrong but I failed miserably
    pub fn spin(&mut self) {
        clear!();
        loop {
            let mut cube_x = self.cube_width * -1.0;
            while cube_x < self.cube_width {
                let mut cube_y = self.cube_width * -1.0;
                while cube_y < self.cube_width {
                    self.calculate_surface(cube_x, cube_y, self.cube_width * 1.0, '$');
                    cube_y += self.increment_speed;
                }
                cube_x += self.increment_speed;
            }

            clear!();
            let mut k = 0;
            while k < self.width * self.height {
                print!("{}", self.buffer[k as usize]);

                k += 1;
            }

            self.a += 0.005;
            self.b += 0.005;
        };
    }
}