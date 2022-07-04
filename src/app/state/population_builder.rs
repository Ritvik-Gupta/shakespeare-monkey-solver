use super::{biased_scale::BiasedScaleStore, population_store::PopulationStore};
use crate::core::population::Population;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub struct PopulationBuilder {
    pub target_term: String,
    pub mutation_rate: usize,
    pub population_size: usize,
    pub biased_scale: BiasedScaleStore,
    pub scale_factor: f64,
}

impl PopulationBuilder {
    pub fn build_simulation(&mut self) -> PopulationStore {
        PopulationStore {
            store: Population::new(
                self.target_term.clone(),
                self.mutation_rate,
                self.population_size,
                (self.biased_scale.clone(), self.scale_factor).into(),
            ),
            generation_counter: 0,
            best_candidate: 0,
            has_finished: false,
        }
    }
}

impl PartialEq<&PopulationStore> for PopulationBuilder {
    fn eq(&self, other: &&PopulationStore) -> bool {
        self.population_size == other.population.len()
            && self.target_term == other.target_term
            && self.mutation_rate == other.mutation_rate
            && (self.biased_scale.clone(), self.scale_factor) == other.biased_scale
    }
}
