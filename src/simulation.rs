pub struct Simulation {
    field: Vec<f64>,
    field_dot: Vec<f64>,
    c: f64,
}

const DELTA_T: f64 = 0.0001;
pub const X: u32 = 500;
pub const Y: u32 = 500;
const H: f64 = 1.0 / X as f64;
const C: f64 = 0.01;
const INIT: f64 = 0.1;

impl Simulation {
    pub fn new() -> Self {
        let mut field = vec![0.0; (X * Y) as usize];
        field[(X * (Y / 2) + X / 2) as usize] = INIT;
        field[(X * (Y / 2) + X / 2) as usize - 1] = INIT;
        field[(X * (Y / 2) + X / 2) as usize + 1] = INIT;
        field[(X * (Y / 2) + X / 2) as usize - X as usize] = INIT;
        field[(X * (Y / 2) + X / 2) as usize + X as usize] = INIT;
        let mut field_dot = vec![0.0; (X * Y) as usize];
        //field_dot[(X * (Y / 2) + X / 2) as usize] = 1.0;

        Self {
            field,
            field_dot,
            c: C,
        }
    }

    pub fn multi_step(&mut self, n: u32) -> &Vec<f64> {
        for _ in 0..n - 1 {
            let _ = self.step();
        }
        self.step()
    }

    pub fn step(&mut self) -> &Vec<f64> {
        let field_dot = self.field_dot.clone();
        for (n, node) in self.field.clone().iter().enumerate() {
            let left;
            if n as u32 % X == 0 {
                left = 0.0;
            } else {
                left = self.field[n - 1];
            }

            let right;
            if n as u32 % X == X - 1 {
                right = 0.0;
            } else {
                right = self.field[n + 1];
            }

            let u_ddx = (left - 2.0 * *node + right) / H.powi(2);

            let top;
            if n as u32 / X == 0 {
                top = 0.0;
            } else {
                top = self.field[n - X as usize];
            }

            let bottom;
            if n as u32 / X == Y - 1 {
                bottom = 0.0;
            } else {
                bottom = self.field[n + X as usize];
            }

            let u_ddy = (top - 2.0 * *node + bottom) / H.powi(2);

            let u_ddot = self.c.powi(2) * (u_ddx + u_ddy);
            self.field_dot[n] = field_dot[n] + u_ddot * DELTA_T;
            self.field[n] = node + field_dot[n] * DELTA_T;
        }
        &self.field
    }

    pub fn energy(&self) -> f64 {
        self.field.iter().map(|x| x.abs()).sum::<f64>() / (X * Y) as f64
    }
}
