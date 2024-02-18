#[test]
fn check_contains_duplicate_solution() {
    use leet_rusty::problems::contains_duplicate;
    //Arange
    let tes_cases = vec![
        (vec![1, 2, 3, 1], true),
        (vec![1, 3, 4], false),
        (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
    ]; // input epected_reusult
       //WHERE
    for (nums, expected_result) in tes_cases {
        let sol = contains_duplicate::Solution::new(nums);

        //THEN
        assert_eq!(sol.contains_duplicate(), expected_result);
    }
}
