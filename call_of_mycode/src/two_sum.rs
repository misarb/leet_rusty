pub struct Solution {
    pub nums: Vec<i32>,
    pub target: i32,
}
impl Solution {
    pub fn two_sum(&self) -> Vec<i32> {
        for i in 0..self.nums.len() {
            for j in i + 1..self.nums.len() {
                if self.nums[i] + self.nums[j] == self.target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

