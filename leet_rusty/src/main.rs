use leet_rusty::problems::two_sum::Solution;
fn main() {
    let nums = vec![4, 2, 8, 6, 1];
    let target = 5;

    // test
    let result = Solution::new(nums, target);
    println!("Result = {:?}", result.two_sum());
}
