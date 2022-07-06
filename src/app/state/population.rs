use std::ops::Deref;

use crate::core::population::Population;

pub struct PopulationStore {
    pub store: Population,
    pub generation_counter: usize,
    pub best_candidate: usize,
    pub best_generation_fitness: Vec<usize>,
    pub has_finished: bool,
}

impl Deref for PopulationStore {
    type Target = Population;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl PopulationStore {
    fn compute_biased_fitness_and_best_candidate(&mut self) {
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

    pub fn simulate_generation(&mut self) {
        if self.has_finished {
            return;
        }

        self.compute_biased_fitness_and_best_candidate();
        self.best_generation_fitness
            .push(self.store.population[self.best_candidate].fitness);

        if !self.has_finished {
            self.store.update_generation();
            self.generation_counter += 1;
        }
    }
}
