pub fn part1(input: &str) -> i32 {
    input.chars()
        .map(|step| {
            match step {
                '(' => 1,
                ')' => -1,
                _ => panic!("Invalid step: {}", step)
            }
        }).fold(0, |acc, step| acc + step)
}

pub fn part2(input: &str) -> i32 {
    let mut floor = 0;
    for (idx, step) in input.chars().enumerate() {
        match step {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid step: {}", step)
        }
        if floor == -1 {
            let idx = idx as i32 + 1;
            return idx;
        }
    }
    input.len() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    #[test]
    fn test_b() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    #[test]
    fn test_c() {
        assert_eq!(part1("))((((("), 3);
    }

    #[test]
    fn test_d() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    #[test]
    fn test_e() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3)
    }

    #[test]
    fn test_f() {
        assert_eq!(part2(")"), 1);
    }

    #[test]
    fn test_g() {
        assert_eq!(part2("()()))"), 5);
    }

    #[cfg(feature = "inputfile")]
    #[test]
    fn part1_inputfile() {
        use crate::utils::print_solution;
        print_solution(2015, 1, 1, part1);
    }

    #[cfg(feature = "inputfile")]
    #[test]
    fn part2_inputfile() {
        use crate::utils::print_solution;
        print_solution(2015, 1, 2, part2);
    }
}