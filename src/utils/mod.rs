#[cfg(not(target_arch = "wasm32"))]
pub mod random {
    use rand::{distributions::WeightedIndex, prelude::ThreadRng, Rng};
    use std::ops::Range;

    const WEIGHTED_OFFSET: f64 = 0.1;

    pub struct Random(ThreadRng);

    impl Random {
        pub fn new() -> Self {
            Random(rand::thread_rng())
        }

        pub fn gen_range_usize(&mut self, range: Range<usize>) -> usize {
            self.0.gen_range(range)
        }

        pub fn gen_range_f64(&mut self, range: Range<f64>) -> f64 {
            self.0.gen_range(range)
        }
    }

    pub struct WeightedIndices(WeightedIndex<f64>);

    impl WeightedIndices {
        pub fn create(iter: impl Iterator<Item = f64>) -> Self {
            Self(
                WeightedIndex::new(iter.map(|val| val + WEIGHTED_OFFSET))
                    .expect("Could not create Weighted Index"),
            )
        }

        pub fn sample(&self, rng: &mut Random) -> usize {
            use rand::prelude::Distribution;

            self.0.sample(&mut rng.0)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod random {
    use std::ops::Range;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Math, js_name = random)]
        fn gen_random() -> f64;
    }

    pub struct Random;

    impl Random {
        pub fn new() -> Self {
            Self
        }

        pub fn gen_range_usize(&mut self, range: Range<usize>) -> usize {
            let (start, end) = (range.start as f64, range.end as f64);
            (start + gen_random() * (end - start)).floor() as usize
        }

        pub fn gen_range_f64(&mut self, range: Range<f64>) -> f64 {
            range.start + gen_random() * (range.end - range.start)
        }
    }

    pub struct WeightedIndices(Vec<f64>);

    impl WeightedIndices {
        pub fn create(iter: impl Iterator<Item = f64>) -> Self {
            let mut cumulative = 0.0;
            let mut weighted_indices = Self(Vec::new());

            for elm in iter {
                cumulative += elm;
                weighted_indices.0.push(cumulative);
            }

            weighted_indices
        }

        pub fn sample(&self, rng: &mut Random) -> usize {
            use std::cmp::Ordering::*;

            let value = rng.gen_range_f64(0.0..*self.0.last().unwrap());

            let (mut low, mut high) = (0, self.0.len() - 1);
            while low <= high {
                let mid = low + (high - low) / 2;

                match self.0[mid]
                    .partial_cmp(&value)
                    .expect("Floating Point comparison problem")
                {
                    Less => low = mid + 1,
                    Greater if mid > 0 => high = mid - 1,
                    _ => return mid,
                }
            }
            low
        }
    }
}
