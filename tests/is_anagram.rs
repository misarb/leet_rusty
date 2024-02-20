#[test]

fn check_is_anagram_solution() {
    use leet_rusty::problems::is_anagram::Solution;

    //Arrange
    let test_cases = [
        ("anagram".to_string(), "nagaram".to_string(), true),
        ("car".to_string(), "tar".to_string(), false),
        ("mama".to_string(), "amam".to_string(), true),
        (" ".to_string(), " ".to_string(), true),
    ];

    // Where
    for (s, t, expected_result) in test_cases {
        let sol = Solution::new(s, t);

        // THEN
        assert_eq!(sol.is_anagram(), expected_result);
    }
}
