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

#[cfg(target_arch = "wasm32")]
fn gen_random_char() -> char {
    'a'
}

#[cfg(not(target_arch = "wasm32"))]
fn gen_random_char() -> char {
    use rand::Rng;

    CHARSET[rand::thread_rng().gen_range(0..CHARSET.len())]
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
        use rand::Rng;

        let mut child = Self {
            genes: Vec::with_capacity(partner_a.genes.len()),
            fitness: 0,
            biased_fitness: 0.0,
        };

        let mut rng = rand::thread_rng();
        let midpoint = rng.gen_range(0..partner_a.genes.len());

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
        use rand::Rng;

        let mut rng = rand::thread_rng();
        self.genes.iter_mut().for_each(|gene| {
            if rng.gen_range(0..=100) < mutation_rate {
                *gene = gen_random_char();
            }
        });
    }
}
