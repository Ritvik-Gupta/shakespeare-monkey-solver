pub mod dna;

#[derive(Default)]
pub struct PopulationForm {
    pub target_term: String,
    pub mutation_rate: usize,
    pub population_size: usize,
}

impl PopulationForm {
    pub fn create_simulation(&mut self) -> Population {
        let population: Vec<_> =
            std::iter::repeat_with(|| Dna::crate_random_genes(self.target_term.len()))
                .take(self.population_size)
                .collect();

        Population {
            next_gen_population: population.clone(),
            population,
            target_term: self.target_term.clone(),
            generations: 0,
            mutation_rate: self.mutation_rate,
            best_dna: None,
            total_fitness: None,
        }
    }
}

impl PartialEq<&Population> for PopulationForm {
    fn eq(&self, other: &&Population) -> bool {
        self.population_size == other.population.len()
            && self.target_term == other.target_term
            && self.mutation_rate == other.mutation_rate
    }
}

use dna::Dna;

pub struct Population {
    next_gen_population: Vec<Dna>,
    pub population: Vec<Dna>,
    pub target_term: String,
    pub mutation_rate: usize,
    pub generations: usize,
    pub best_dna: Option<String>,
    pub total_fitness: Option<f64>,
}

impl Population {
    pub fn compute_biased_fitness_if_not_finished(&mut self) -> bool {
        let mut total_fitness = 0.0;
        for dna in self.population.iter_mut() {
            let fitness = dna.compute_fitness(&self.target_term);
            if fitness == self.target_term.len() {
                return true;
            }

            dna.biased_fitness = Some(fitness as f64);
            total_fitness += fitness as f64;
        }

        self.total_fitness = Some(total_fitness);
        false
    }

    pub fn update_generation(&mut self) {
        use rand::distributions::WeightedIndex;

        let weighted_indices = WeightedIndex::new(
            self.population
                .iter()
                .map(|dna| dna.biased_fitness.unwrap()),
        )
        .unwrap();

        for i in 0..self.population.len() {
            let mut dna = Dna::crossover(
                self.pool_selection(&weighted_indices),
                self.pool_selection(&weighted_indices),
            );
            dna.mutate(self.mutation_rate);
            self.next_gen_population[i] = dna;
        }

        std::mem::swap(&mut self.population, &mut self.next_gen_population);
        self.total_fitness = None;
        self.generations += 1;
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
