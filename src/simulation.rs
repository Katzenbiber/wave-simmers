pub struct Simulation {
    field: Vec<f64>,
    field_dot: Vec<f64>,
    timestep: f64,
    pub x: u32,
    pub y: u32,
    c: f64,
    h: f64,
}

impl Simulation {
    pub fn new(args: crate::Args) -> Self {
        let timestep = args.timestep;
        let x = args.x;
        let y = args.y;
        let init = args.init;
        let c = args.c;

        let mut field = vec![0.0; (x * y) as usize];
        let center = (x as i32 / 2, y as i32 / 2);
        log::debug!("center: {center:?}");
        for (n, node) in field.iter_mut().enumerate() {
            let row = n as i32 / x as i32;
            let col = n as i32 % x as i32;
            let offset = ((col - center.0) as f64, (row - center.1) as f64);

            *node = Simulation::init_value_gaus(offset, init)
        }
        let mut field_dot = vec![0.0; (x * y) as usize];
        //field_dot[(x * (y / 2) + x / 2) as usize] = 1.0;

        Self {
            field,
            field_dot,
            timestep,
            x,
            y,
            c,
            h: 1.0 / x as f64,
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
            if n as u32 % self.x == 0 {
                left = 0.0;
            } else {
                left = self.field[n - 1];
            }

            let right;
            if n as u32 % self.x == self.x - 1 {
                right = 0.0;
            } else {
                right = self.field[n + 1];
            }

            let u_ddx = (left - 2.0 * *node + right) / self.h.powi(2);

            let top;
            if n as u32 / self.x == 0 {
                top = 0.0;
            } else {
                top = self.field[n - self.x as usize];
            }

            let bottom;
            if n as u32 / self.x == self.y - 1 {
                bottom = 0.0;
            } else {
                bottom = self.field[n + self.x as usize];
            }

            let u_ddy = (top - 2.0 * *node + bottom) / self.h.powi(2);

            let u_ddot = self.c.powi(2) * (u_ddx + u_ddy);
            self.field_dot[n] = field_dot[n] + u_ddot * self.timestep;
            self.field[n] = node + field_dot[n] * self.timestep;
        }
        &self.field
    }

    pub fn energy(&self) -> f64 {
        let pot = self.field.iter().map(|x| x.powi(2)).sum::<f64>() / (self.x * self.y) as f64;

        let kin = self.field_dot.iter().map(|x| x.powi(2)).sum::<f64>() / (self.x * self.y) as f64;
        pot + kin
    }

    fn init_value_gaus(offset: (f64, f64), init: f64) -> f64 {
        let mu = 0.0;
        let sigma = 4.0;

        let dist = (offset.0.powi(2) + offset.1.powi(2)).sqrt();
        let exp = -0.5 * ((dist - mu) / sigma).powi(2);
        exp.exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt()) * init
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaus() {
        let left = (-1.0, 0.0);
        let right = (1.0, 0.0);
        let init = 1.0;
        assert_eq!(
            Simulation::init_value_gaus(left, init),
            Simulation::init_value_gaus(right, init)
        );
    }
}
