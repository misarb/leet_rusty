use std::collections::HashMap;

pub struct Solution {
    s: String,
    t: String,
}

impl Solution {
    pub fn new(s: String, t: String) -> Self {
        Self { s, t }
    }

    pub fn is_anagram(&self) -> bool {
        if self.s.len() != self.t.len() {
            return false;
        }

        let mut count_char_s: HashMap<char, i32> = HashMap::new();
        let mut count_char_t: HashMap<char, i32> = HashMap::new();

        for (i, j) in self.s.chars().zip(self.t.chars()) {
            *count_char_s.entry(i).or_default() = 1 + *count_char_s.entry(i).or_default();
            *count_char_t.entry(j).or_default() = 1 + *count_char_t.entry(j).or_default();
        }

        // check if values of the two hashMapare Eq
        for (k, v) in count_char_t.iter() {
            match count_char_s.get(&k) {
                Some(t_v) if t_v == v => continue,
                _ => return false,
            }
        }

        return true;
    }
}
