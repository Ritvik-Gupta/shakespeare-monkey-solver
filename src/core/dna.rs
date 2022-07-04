use crate::utils::random::Random;

const fn generate_charset() -> [char; 53] {
    let mut charset = [' '; 53];

    let mut i = 0;
    while i < 26 {
        charset[i] = (i as u8 + b'a') as char;
        i += 1;
    }

    let mut i = 0;
    while i < 26 {
        charset[i + 27] = (i as u8 + b'A') as char;
        i += 1;
    }

    charset
}

static CHARSET: [char; 53] = generate_charset();

fn gen_random_char() -> char {
    CHARSET[Random::new().gen_range_usize(0..CHARSET.len())]
}

#[derive(Clone, Debug)]
pub struct Dna {
    pub genes: Vec<char>,
    pub fitness: usize,
    pub biased_fitness: f64,
}

impl Dna {
    pub fn crate_random_genes(num_genes: usize) -> Self {
        Self {
            genes: std::iter::repeat_with(gen_random_char)
                .take(num_genes)
                .collect(),
            fitness: 0,
            biased_fitness: 0.0,
        }
    }

    pub fn compute_fitness(&mut self, target_term: &String) -> usize {
        self.fitness = target_term
            .char_indices()
            .filter(|&(idx, token)| self.genes[idx] == token)
            .count();
        self.fitness
    }

    pub fn crossover(partner_a: &Self, partner_b: &Self) -> Self {
        let mut child = Self {
            genes: Vec::with_capacity(partner_a.genes.len()),
            fitness: 0,
            biased_fitness: 0.0,
        };

        let midpoint = Random::new().gen_range_usize(0..partner_a.genes.len());

        for i in 0..partner_a.genes.len() {
            child.genes.push(if i > midpoint {
                partner_a.genes[i]
            } else {
                partner_b.genes[i]
            });
        }

        child
    }

    pub fn mutate(&mut self, mutation_rate: usize) {
        let mut rng = Random::new();
        self.genes.iter_mut().for_each(|gene| {
            if rng.gen_range_usize(0..101) < mutation_rate {
                *gene = gen_random_char();
            }
        });
    }
}
