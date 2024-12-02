use advent_of_code::solutions::year2024::day01::solve;
use advent_of_code::utils::input::Input;

#[test]
fn day01() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    let output = solve(Input::Text(input.into()));
    
    if let Ok(output) = output {
        assert_eq!(output.part_1, 11.to_string());
        assert_eq!(output.part_2, 31.to_string());
    } else {
        panic!("Invalid output");
    }

    //let real_output = solve(Input::Path("inputs/2024/day01.txt".into()));
    //if let Ok(output) = real_output {
    //    output.print();
    //}
}
