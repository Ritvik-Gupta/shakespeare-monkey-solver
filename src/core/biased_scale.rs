use BiasedScale::*;

pub enum BiasedScale {
    Multiplicative(f64),
    Order(f64),
    Exponential(f64),
}

impl BiasedScale {
    pub fn scale(&self, value: f64) -> f64 {
        match self {
            &Multiplicative(factor) => value * factor,
            &Order(factor) => value.powf(factor),
            &Exponential(factor) => factor.powf(value),
        }
    }
}
