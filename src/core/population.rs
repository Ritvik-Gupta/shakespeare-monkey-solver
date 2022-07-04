use super::dna::Dna;

pub struct Population {
    pub(super) next_gen_population: Vec<Dna>,
    pub population: Vec<Dna>,
    pub target_term: String,
    pub mutation_rate: usize,
}

impl Population {
    pub fn new(target_term: String, mutation_rate: usize, population_size: usize) -> Self {
        let population = std::iter::repeat_with(|| Dna::crate_random_genes(target_term.len()))
            .take(population_size)
            .collect::<Vec<_>>();
        Self {
            next_gen_population: population.clone(),
            population,
            mutation_rate,
            target_term,
        }
    }

    pub fn compute_biased_fitness(&mut self) {
        for candidate in self.population.iter_mut() {
            let fitness = candidate.compute_fitness(&self.target_term);
            candidate.biased_fitness = fitness as f64;
        }
    }

    pub fn update_generation(&mut self) {
        use rand::distributions::WeightedIndex;

        let weighted_indices = WeightedIndex::new(
            self.population
                .iter()
                .map(|candidate| candidate.biased_fitness),
        )
        .expect("weighted index cannot be created");

        for i in 0..self.population.len() {
            let mut child_candidate = Dna::crossover(
                self.pool_selection(&weighted_indices),
                self.pool_selection(&weighted_indices),
            );
            child_candidate.mutate(self.mutation_rate);
            self.next_gen_population[i] = child_candidate;
        }

        std::mem::swap(&mut self.population, &mut self.next_gen_population);
    }

    fn pool_selection(&self, weighted_indices: &rand::distributions::WeightedIndex<f64>) -> &Dna {
        use rand::prelude::Distribution;

        &self.population[weighted_indices.sample(&mut rand::thread_rng())]
    }
}

/*

  // Compute the current "most fit" member of the population
  evaluate() {
    let worldrecord = 0.0;
    let index = 0;
    for (let i = 0; i < this.population.length; i++) {
      if (this.population[i].fitness > worldrecord) {
        index = i;
        worldrecord = this.population[i].fitness;
      }
    }

    this.best = this.population[index].getPhrase();
    if (worldrecord === this.perfectScore) {
      this.finished = true;
    }
  }

  isFinished() {
    return this.finished;
  }

  getGenerations() {
    return this.generations;
  }

  // Compute average fitness for the population
  getAverageFitness() {
    let total = 0;
    for (let i = 0; i < this.population.length; i++) {
      total += this.population[i].fitness;
    }
    return total / this.population.length;
  }

  allPhrases() {
    let everything = "";

    let displayLimit = min(this.population.length, 50);

    for (let i = 0; i < displayLimit; i++) {
      everything += this.population[i].getPhrase() + "<br>";
    }
    return everything;
  }
}


*/
