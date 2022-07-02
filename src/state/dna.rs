#[cfg(target_arch = "wasm32")]
fn gen_random_char() -> char {
    'a'
}

#[cfg(not(target_arch = "wasm32"))]
fn gen_random_char() -> char {
    use rand::Rng;
    rand::thread_rng().sample(rand::distributions::Alphanumeric) as char
}

#[derive(Debug)]
pub struct Dna {
    pub genes: Vec<char>,
    pub fitness: usize,
}

impl Dna {
    pub fn crate_random_genes(num_genes: usize) -> Self {
        Self {
            genes: std::iter::repeat_with(gen_random_char)
                .take(num_genes)
                .collect(),
            fitness: 0,
        }
    }
}
//     // Fitness function (returns floating point % of "correct" characters)
//     calcFitness(target) {
//       let score = 0;
//       for (let i = 0; i < this.genes.length; i++) {
//         if (this.genes[i] == target.charAt(i)) {
//           score++;
//         }
//       }
//       this.fitness = score / target.length;
//     }

//     // Crossover
//     crossover(partner) {
//       // A new child
//       let child = new DNA(this.genes.length);

//       let midpoint = floor(random(this.genes.length)); // Pick a midpoint

//       // Half from one, half from the other
//       for (let i = 0; i < this.genes.length; i++) {
//         if (i > midpoint) child.genes[i] = this.genes[i];
//         else child.genes[i] = partner.genes[i];
//       }
//       return child;
//     }

//     // Based on a mutation probability, picks a new random character
//     mutate(mutationRate) {
//       for (let i = 0; i < this.genes.length; i++) {
//         if (random(1) < mutationRate) {
//           this.genes[i] = newChar();
//         }
//       }
//     }
//   }
