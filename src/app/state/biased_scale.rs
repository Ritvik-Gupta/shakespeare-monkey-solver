use crate::core::biased_scale::BiasedScale;
use BiasedScaleStore::*;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum BiasedScaleStore {
    Multiplicative,
    Order,
    Exponential,
}

impl Default for BiasedScaleStore {
    fn default() -> Self {
        Multiplicative
    }
}

impl Into<BiasedScale> for (BiasedScaleStore, f64) {
    fn into(self) -> BiasedScale {
        match self.0 {
            Multiplicative => BiasedScale::Multiplicative(self.1),
            Order => BiasedScale::Order(self.1),
            Exponential => BiasedScale::Exponential(self.1),
        }
    }
}

impl PartialEq<BiasedScale> for (BiasedScaleStore, f64) {
    fn eq(&self, other: &BiasedScale) -> bool {
        match (&self.0, other) {
            (Multiplicative, BiasedScale::Multiplicative(factor))
            | (Order, BiasedScale::Order(factor))
            | (Exponential, BiasedScale::Exponential(factor))
                if *factor == self.1 =>
            {
                true
            }
            _ => false,
        }
    }
}
