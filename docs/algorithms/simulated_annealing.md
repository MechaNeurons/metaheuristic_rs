# Simulated Annealing

## How to use

1. First import `Float` trait from `num_trait` and `SimulatedAnnealing` struct and `Energy` trait from `simulated_annealing`

   ```rust
   use num_traits::Float;
   use simulated_annealing::{GetEnergy, SimulatedAnnealing};
   ```
2. Now you must implement `get_energy` function for the `SimulatedAnnealing` for the objective function you want to maximize. For example if your objective function is $-(x-2)^2$ then you can use the code below.

   ```rust
   impl<T: Float + Display> GetEnergy<T> for SimulatedAnnealing<T> {
        fn get_energy(&self, x: T) -> T {
           -(x - T::from(2).unwrap()).powi(2)
        }
    }

   ```
3. In your main function create an instance of `SimulatedAnnealing` struct using `new()` and give it the right parameters.
and then run optimize method on your problem.

```rust
let mut problem = SimulatedAnnealing::new(min_cor: T,max_cor: T,min_temp: T,max_temp: T,cooling_rate: T,initial_state: T);
problem.optimize();
```
The `SimulatedAnnealing` is a generic type so you must use only one type.
