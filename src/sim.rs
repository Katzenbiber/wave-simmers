pub struct Simulation {
    size: (f64, f64),
    discretization: u32,
    u_n: Vec<f64>,
    u_nm1: Vec<f64>,
    c: f64,
    t: f64,
}

impl Simulation {
    pub fn new(args: &crate::Args) -> Self {
        let size = (args.x, args.y);
        let u_n = vec![0.0; (args.discretization * args.discretization) as usize];
        // let u_n = Self::init_value_gauss(size, args.discretization);
        let u_nm1 = vec![0.0; (args.discretization * args.discretization) as usize];

        Self {
            size,
            discretization: args.discretization,
            u_n,
            u_nm1,
            c: args.c,
            t: 0.0,
        }
    }

    pub fn multi_step(&mut self, n: u32, dt: f64) -> &Vec<f64> {
        for _ in 0..n - 1 {
            let _ = self.step(dt);
        }
        self.step(dt)
    }

    pub fn step(&mut self, dt: f64) -> &Vec<f64> {
        let c = self.c;
        let mut u_np1 = vec![0.0; (self.discretization * self.discretization) as usize];

        for i in 0..self.u_n.len() {
            let (left, right, top, bottom) = self.get_star(i);
            let uxx = (left - 2.0 * self.u_n[i] + right)
                / (self.size.0 / self.discretization as f64).powi(2);
            let uyy = (top - 2.0 * self.u_n[i] + bottom)
                / (self.size.1 / self.discretization as f64).powi(2);
            let laplacian = uxx + uyy;
            u_np1[i] = 2.0 * self.u_n[i] - self.u_nm1[i] + c.powi(2) * dt.powi(2) * laplacian;
        }

        let center = self.discretization as usize * (self.discretization as usize / 2)
            + self.discretization as usize / 2;
        u_np1[center] = (self.t * 5.0).sin();

        self.u_nm1 = self.u_n.clone();
        self.u_n = u_np1;
        self.t += dt;
        &self.u_n
    }

    pub fn energy(&self) -> f64 {
        self.u_n.iter().map(|x| x.abs().powi(2)).sum()
    }

    pub fn time(&self) -> f64 {
        self.t
    }

    fn init_value_gauss(size: (f64, f64), disc: u32) -> Vec<f64> {
        let mu = 0.0;
        let sigma = 5.0;
        let mut u = vec![0.0; (disc * disc) as usize];
        for i in 0..(disc * disc) {
            let x = (i % disc) as f64 * size.0 / disc as f64;
            let y = (i / disc) as f64 * size.1 / disc as f64;
            let dist_from_center = ((x - size.0 / 2.0).powi(2) + (y - size.1 / 2.0).powi(2)).sqrt();
            u[i as usize] = Self::gauss(dist_from_center, mu, sigma) * 0.001;
        }
        u
    }

    fn gauss(x: f64, mu: f64, sigma: f64) -> f64 {
        let exp = -(x - mu).powi(2) / (2.0 * sigma.powi(2));
        exp.exp() / (sigma * (2.0 * std::f64::consts::PI).sqrt())
    }

    fn get_star(&self, n: usize) -> (f64, f64, f64, f64) {
        let left = if n as u32 % self.discretization == 0 {
            0.0
        } else {
            self.u_n[n - 1]
        };

        let right = if n as u32 % self.discretization == self.discretization - 1 {
            0.0
        } else {
            self.u_n[n + 1]
        };

        let top = if n as u32 / self.discretization == 0 {
            0.0
        } else {
            self.u_n[n - self.discretization as usize]
        };

        let bottom = if n as u32 / self.discretization == self.discretization - 1 {
            0.0
        } else {
            self.u_n[n + self.discretization as usize]
        };

        (left, right, top, bottom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss() {
        assert_eq!(Simulation::gauss(0.0, 0.0, 1.0), 0.3989422804014327);
        assert_eq!(Simulation::gauss(1.0, 0.0, 1.0), 0.24197072451914337);
    }

    #[test]
    fn test_get_star() {
        let sim = Simulation {
            size: (1.0, 1.0),
            discretization: 3,
            u_n: vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            u_nm1: vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            c: 1.0,
            t: 0.0,
        };

        assert_eq!(sim.get_star(0), (0.0, 1.0, 0.0, 3.0));
        assert_eq!(sim.get_star(1), (0.0, 2.0, 0.0, 4.0));
        assert_eq!(sim.get_star(2), (1.0, 0.0, 0.0, 5.0));
        assert_eq!(sim.get_star(3), (0.0, 4.0, 0.0, 6.0));
        assert_eq!(sim.get_star(4), (3.0, 5.0, 1.0, 7.0));
        assert_eq!(sim.get_star(5), (4.0, 0.0, 2.0, 8.0));
        assert_eq!(sim.get_star(6), (0.0, 7.0, 3.0, 0.0));
        assert_eq!(sim.get_star(7), (6.0, 8.0, 4.0, 0.0));
        assert_eq!(sim.get_star(8), (7.0, 0.0, 5.0, 0.0));
    }
}
