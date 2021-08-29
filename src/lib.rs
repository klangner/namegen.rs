//! Names generator
//! 

use std::collections::HashMap;
use rand_xoshiro::rand_core::RngCore;


const START_CHAR: char = '^';
const END_CHAR: char = '$';

 
#[derive(Debug)]
struct FreqTable {
    total_count: u32,
    chars: Vec<char>,
    counts: Vec<u32>
}

#[derive(Debug)]
pub struct NameGen {
    model: HashMap<char, FreqTable>,
}


impl FreqTable {
    fn new(counter: &HashMap<char, u32>) -> FreqTable {
        let total_count = counter.values().sum();
        let chars:Vec<char> = counter.keys().map(|k| *k).collect();
        let counts = chars.iter().map(|k| *counter.get(k).unwrap()).collect();
        FreqTable {total_count, chars, counts}
    }

    fn next_char(&self, rng: &mut dyn RngCore) -> char {
        let idx = rng.next_u32() % self.total_count;
        let mut total: u32 = 0;
        for i in 0..self.counts.len() {
            total += self.counts[i];
            if total > idx { 
                return self.chars[i] 
            }
        }
       '$'
    }
}


impl NameGen {
    pub fn new(names: &Vec<&str>) -> NameGen {
        let mut builder: HashMap<char, HashMap<char, u32>> = HashMap::new();

        for name in names {
            if name.len() > 0 {
                // Extends the word with start and end character
                let mut extended_name = String::from(START_CHAR); 
                extended_name.push_str(name);
                extended_name.push(END_CHAR);

                // update counters
                for i in 0..extended_name.len()-1 {
                    let c = extended_name.as_bytes()[i] as char;
                    let next_c = extended_name.as_bytes()[i+1] as char;

                    if let Some(counters) = builder.get_mut(&c) { 
                        let n = if let Some(v) = counters.get(&next_c) { v+1 } else { 1 };
                        counters.insert(next_c, n); 
                    } else { 
                        let mut counters = HashMap::new();
                        counters.insert(next_c, 1);
                        builder.insert(c, counters);
                    }
                }
            }
        }

        let model: HashMap<char, FreqTable> = builder.iter().map(|(k, v)| (*k, FreqTable::new(v))).collect();

        NameGen {model}
    }

    pub fn next_name(&self, rng: &mut dyn RngCore, max_len: usize) -> String {
        if self.model.is_empty() {
            return String::new();
        }
        let mut name = String::new();
        let mut last_char = '^';

        // Max name length = 100
        for _ in 0..max_len {
            last_char = self.model.get(&last_char).map(|ft| ft.next_char(rng)).unwrap();
            if last_char == '$' {
                break;
            } else {
                name.push(last_char);
            }
        }
        name
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use rand_xoshiro::Xoshiro256Plus;
    use rand_xoshiro::rand_core::SeedableRng;


    #[test]
    fn freq_table_next_char() {
        let mut data = HashMap::new();
        data.insert('a', 2);
        data.insert('b', 5);
        data.insert('c', 1);
        let table = FreqTable::new(&data);
        let mut gen = Xoshiro256Plus::seed_from_u64(100);

        assert_eq!(table.next_char(&mut gen), 'b');
    }

    #[test]
    fn empty_training_set() {
        let names = Vec::new();
        let gen = NameGen::new(&names);
        let mut rng = Xoshiro256Plus::seed_from_u64(100);

        assert_eq!(gen.next_name(&mut rng, 10), "");
    }

    #[test]
    fn single_word() {
        let names = vec!["ala"];
        let gen = NameGen::new(&names);
        let mut rng = Xoshiro256Plus::seed_from_u64(5);

        assert_eq!(gen.next_name(&mut rng, 10), "alalalala");
    }
}
