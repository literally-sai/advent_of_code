pub fn part1(input: &str) -> i32 {
    input.split_whitespace()
        .into_iter()
        .map(|freq| freq.replace(",", ""))
        .fold(0, |acc, freq| freq.parse::<i32>().unwrap() + acc)
}

pub fn part2(input: &str) -> i32 {
    0
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part1("+1, +1, +1"), 3);
    }

    #[test]
    fn test_b() {
        assert_eq!(part1("+1, +1, -2"), 0);
    }

    #[test]
    fn test_c() {
        assert_eq!(part1("-1, -2, -3"), -6);
    }

    #[test]
    fn test_d() {
        assert_eq!(part2("+1, -1"), 0);
    }

    #[test]
    fn test_e() {
        assert_eq!(part2("+3, +3, +4, -2, -4"), 10);
    }

    #[test]
    fn test_f() {
        assert_eq!(part2("-6, +3, +8, +5, -6"), 5);
    }

    #[cfg(feature = "inputfile")]
    #[test]
    fn part1_inputfile() {
        use crate::utils::print_solution;
        print_solution(2018, 1, 1, part1);
    }
}