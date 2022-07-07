use crate::utils::random::Random;

const SPECIAL_SYMBOLS: [char; 14] = [
    ' ', '-', '(', ')', '[', ']', '"', '\'', '/', '.', ',', '_', '!', ':',
];

const fn generate_charset() -> [char; 26 * 2 + SPECIAL_SYMBOLS.len()] {
    let mut charset = [' '; 26 * 2 + SPECIAL_SYMBOLS.len()];

    let mut i = 0;
    let mut j = 0;
    while i < SPECIAL_SYMBOLS.len() {
        charset[j] = SPECIAL_SYMBOLS[i];
        i += 1;
        j += 1;
    }

    let mut i = 0;
    while i < 26 {
        charset[j] = (i as u8 + b'a') as char;
        i += 1;
        j += 1;
    }

    let mut i = 0;
    while i < 26 {
        charset[j] = (i as u8 + b'A') as char;
        i += 1;
        j += 1;
    }

    charset
}

static CHARSET: [char; 26 * 2 + SPECIAL_SYMBOLS.len()] = generate_charset();

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
        self.genes
            .iter_mut()
            .filter(|_| rng.gen_range_usize(0..101) < mutation_rate)
            .for_each(|gene| *gene = gen_random_char());
    }
}
