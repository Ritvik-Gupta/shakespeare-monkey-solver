use super::population_store::PopulationStore;
use crate::core::population::Population;

#[derive(Default)]
pub struct PopulationBuilder {
    pub target_term: String,
    pub mutation_rate: usize,
    pub population_size: usize,
}

impl PopulationBuilder {
    pub fn build_simulation(&mut self) -> PopulationStore {
        PopulationStore {
            store: Population::new(
                self.target_term.clone(),
                self.mutation_rate,
                self.population_size,
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
    }
}
