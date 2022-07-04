use std::ops::{Deref, DerefMut};

use crate::core::population::Population;

pub struct PopulationStore {
    pub store: Population,
    pub generation_counter: usize,
    pub best_candidate: usize,
    pub has_finished: bool,
}

impl Deref for PopulationStore {
    type Target = Population;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl DerefMut for PopulationStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl PopulationStore {
    pub fn compute_biased_fitness(&mut self) {
        self.store.compute_biased_fitness();

        let pop = &self.store.population;

        for (idx, candidate) in pop.iter().enumerate() {
            if candidate.fitness > self.store.population[self.best_candidate].fitness {
                self.best_candidate = idx;
            }

            if candidate.fitness == self.store.target_term.len() {
                self.has_finished = true;
            }
        }
    }

    pub fn update_generation(&mut self) {
        self.store.update_generation();
        self.generation_counter += 1;
    }
}
