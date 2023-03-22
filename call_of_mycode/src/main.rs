mod two_sum;
mod Roman_to_Integer;
use crate::{two_sum::Solution, Roman_to_Integer::SolutionRomanIntegers};
fn main() {
    let sol1 = Solution {
        nums: vec![4, 2, 8, 6, 1],
        target: 5,
    };
    // secand case
    let sol_roman = SolutionRomanIntegers{

        s:"III".to_string(),
    };
    // test
    let result = sol1.two_sum();
    let result_roman = sol_roman.roman_to_int();
    println!("Result = {:?}", result);
    println!("Result = {:?}", result_roman);
}