use advent_of_code::solutions::year2024::day02::solve;
use advent_of_code::utils::input::Input;

#[test]
fn day02() {
    let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let output = solve(Input::Text(input.into()));

    if let Ok(output) = output {
        assert_eq!(output.part_1, 2.to_string());
        assert_eq!(output.part_2, 4.to_string());
    } else {
        panic!("Invalid output");
    }

    
    //let real_output = solve(Input::Path("inputs/2024/day02.txt".into()));
    //if let Ok(output) = real_output {
    //    output.print();
    //}
}
