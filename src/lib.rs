mod simulated_annealing;
use std::fmt::Display;

use num_traits::Float;
use simulated_annealing::{GetEnergy, SimulatedAnnealing};

impl<T: Float + Display> GetEnergy<T> for SimulatedAnnealing<T> {
    fn get_energy(&self, x: T) -> T {
        (x - T::from(0.3).unwrap()).powi(3) - T::from(5).unwrap() * x + x.powi(2)
            - T::from(2).unwrap()
    }
    // fn get_energy(&self, x: T) -> T {
    //     -(x - T::from(2).unwrap()).powi(2)
    // }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut problem = SimulatedAnnealing::new(-2.0, 2.0, 0.01, 100.0, 0.01, 0.0);
        problem.optimize();
    }
}
