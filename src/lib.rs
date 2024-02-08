pub mod tuning_systems {
    pub fn equal_temperament(step: u32, base: u32) -> f64 {
        2f64.powf(step as f64 / base as f64)
    }
}
