use std::collections::HashSet;

pub struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    pub fn contains_duplicate(&self) -> bool {
        let mut non_duplicate_set = HashSet::new();
        for elem in self.nums.iter() {
            if non_duplicate_set.contains(elem) {
                return true;
            }
            non_duplicate_set.insert(*elem);
        }

        false
    }
}
