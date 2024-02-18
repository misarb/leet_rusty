#[allow(dead_code)]
pub struct Solution {
    nums: Vec<i32>,
    target: i32,
}
#[allow(dead_code)]
impl Solution {
    pub fn new(nums: Vec<i32>, target: i32) -> Self {
        Self { nums, target }
    }
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
