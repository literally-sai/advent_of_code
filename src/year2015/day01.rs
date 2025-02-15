pub fn part1(input: &str) -> i32 {
    input.chars()
        .map(|c| {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0
            }
        }).fold(0, |acc, step| acc + step)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);

        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);

        assert_eq!(part1("))((((("), 3);

        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);

        assert_eq!(part1(")))"), -3);
    }

    #[cfg(feature = "inputfile")]
    #[test]
    fn part1_inputfile() {
        use crate::utils::read_input::read_input;
        let input = read_input(2015, 1).expect("Failed to read input file");
        println!("The output for 2015 day01 is {}", part1(&input));
    }
}