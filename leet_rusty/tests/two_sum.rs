#[test]
fn check_two_sum_solution() {
    use leet_rusty::problems::two_sum;
    // WHERE
    let nums = vec![1, 4, 2, 9, 3];
    let target = 11;
    let sol = two_sum::Solution::new(nums, target);

    // THEN
    assert_eq!(sol.two_sum(), [2, 3]);
}
