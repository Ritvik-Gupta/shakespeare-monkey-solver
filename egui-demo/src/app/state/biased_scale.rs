use shakespeare_monkey_solver::biased_scale::BiasedScale;
use BiasedScaleStore::*;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, PartialEq, Default)]
pub enum BiasedScaleStore {
    #[default]
    Multiplicative,
    Order,
    Exponential,
}

impl BiasedScaleStore {
    pub fn build_with_factor(&self, factor: f64) -> BiasedScale {
        match self {
            Multiplicative => BiasedScale::Multiplicative(factor),
            Order => BiasedScale::Order(factor),
            Exponential => BiasedScale::Exponential(factor),
        }
    }
}
