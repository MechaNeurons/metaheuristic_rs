use num_traits::Float;
use rand::Rng;
use std::fmt::Display;

pub trait GetEnergy<T: Float> {
    fn get_energy(&self, x: T) -> T;
}
#[allow(dead_code)]
pub struct SimulatedAnnealing<T: Float + Display> {
    min_cor: T,
    max_cor: T,
    min_temp: T,
    max_temp: T,
    cooling_rate: T,
    state: T,
    best_state: T,
}

// impl<T: Float + Display> GetEnergy<T> for SimulatedAnnealing<T> {
//     // fn get_energy(&self, x: T) -> T {
//     //     (x - T::from(0.3).unwrap()).powi(3) - T::from(5).unwrap() * x + x.powi(2)
//     //         - T::from(2).unwrap()
//     // }
//     fn get_energy(&self, x: T) -> T {
//         -(x - T::from(2).unwrap()).powi(2)
//     }
// }
#[allow(dead_code)]
impl<T: Float + Display> SimulatedAnnealing<T> {
    pub fn new(
        min_cor: T,
        max_cor: T,
        min_temp: T,
        max_temp: T,
        cooling_rate: T,
        initial_state: T,
    ) -> Self {
        Self {
            min_cor,
            max_cor,
            min_temp,
            max_temp,
            cooling_rate,
            state: initial_state,
            best_state: initial_state,
        }
    }

    fn generate_new_state(&self) -> T {
        // let uniform = Uniform::new(self.min_cor,self.max_cor);
        let random = rand::thread_rng().gen::<f64>();
        self.min_cor + (self.max_cor - self.min_cor) * T::from(random).unwrap()
    }

    fn acceptance_probability(&self, energy: T, next_energy: T, temp: T) -> T {
        if next_energy > energy {
            return T::from(1).unwrap();
        }
        ((energy - next_energy) / temp).exp()
    }

    pub fn optimize(&mut self) {
        let mut temp = self.max_temp;
        while temp > self.min_temp {
            let next_state = self.generate_new_state();
            let energy = self.get_energy(self.state);
            let next_energy = self.get_energy(next_state);

            let random = T::from(rand::thread_rng().gen::<f64>()).unwrap();

            if random < self.acceptance_probability(energy, next_energy, temp) {
                self.state = next_state;
            }

            if self.get_energy(self.state) > self.get_energy(self.best_state) {
                self.best_state = self.state;
            }
            temp = temp * (T::from(1).unwrap() - self.cooling_rate);
        }
        println!(
            "optimization sol f({}) = {}",
            self.best_state,
            self.get_energy(self.best_state),
        )
    }
}