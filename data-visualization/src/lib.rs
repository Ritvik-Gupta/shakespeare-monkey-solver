use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shakespeare_monkey_solver::{biased_scale::BiasedScale, population::Population};

fn total_generations_taken_to_simulate(
    target_term: &str,
    mutation_rate: usize,
    population_size: usize,
    biased_scale: BiasedScale,
) -> usize {
    let mut population = Population::new(
        target_term.to_owned(),
        mutation_rate,
        population_size,
        biased_scale,
    );
    let mut generation_counter = 0;

    loop {
        population.compute_biased_fitness();
        if population
            .population
            .par_iter()
            .any(|candidate| candidate.fitness == target_term.len())
        {
            return generation_counter;
        }
        population.update_generation();
        generation_counter += 1;
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub enum BiasedScaleType {
    Multiplicative,
    Order,
    Exponential,
}

impl BiasedScaleType {
    fn build_with_factor(&self, factor: f64) -> BiasedScale {
        match self {
            BiasedScaleType::Multiplicative => BiasedScale::Multiplicative(factor),
            BiasedScaleType::Order => BiasedScale::Order(factor),
            BiasedScaleType::Exponential => BiasedScale::Exponential(factor),
        }
    }
}

pub type BiasedScaleStore = (BiasedScaleType, f64);

#[pyclass]
#[derive(Debug)]
pub struct SimulationFrame {
    pub mutation_rate: usize,
    pub population_size: usize,
    pub biased_scale: BiasedScaleStore,
    pub generations_taken: usize,
}

#[pymethods]
impl SimulationFrame {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pyfunction]
fn compute_generations_for_dataset(
    target_term: &str,
    mutation_range: (usize, usize),
    population_range: (usize, usize),
    biased_scale: BiasedScaleStore,
) -> Vec<SimulationFrame> {
    let dataset = (mutation_range.0..mutation_range.1)
        .flat_map(|mutation_rate| {
            (population_range.0..population_range.1)
                .map(move |population_size| (mutation_rate, population_size))
        })
        .collect::<Vec<_>>();

    dataset
        .par_iter()
        .map(|&(mutation_rate, population_size)| SimulationFrame {
            population_size,
            mutation_rate,
            biased_scale: biased_scale.clone(),
            generations_taken: total_generations_taken_to_simulate(
                target_term,
                mutation_rate,
                population_size,
                biased_scale.0.build_with_factor(biased_scale.1),
            ),
        })
        .collect()
}

#[pymodule]
fn data_visualization(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_generations_for_dataset, m)?)?;
    m.add_class::<BiasedScaleType>()?;
    m.add_class::<SimulationFrame>()?;
    Ok(())
}
