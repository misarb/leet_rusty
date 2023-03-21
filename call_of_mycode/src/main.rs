struct Solution {
    nums: Vec<i32>,
    target: i32,
}
impl Solution {
    fn two_sum(&self) -> Vec<i32> {
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
fn main() {
    let sol1 = Solution {
        nums: vec![4, 2, 8, 6, 3],
        target: 5,
    };
    let result = sol1.two_sum();
    println!("Result = {:?}", result);
}
