use std::collections::HashSet;
use std::ops::ControlFlow;

pub fn part1(input: &str) -> i32 {
    let mut pos = (0, 0);
    let mut dir = Direction::North;
    input.split(", ").for_each(|ins| {
        match ins.chars().next() {
            Some('R') => dir = turn_right(dir),
            Some('L') => dir = turn_left(dir),
            _ => panic!("Invalid instruction: {}", ins),
        }
        let dist = ins[1..].parse().unwrap_or(0);
        pos = move_forward(pos, dir, dist);
    });
    pos.0.abs() + pos.1.abs()
}

pub fn part2(input: &str) -> i32 {
    let mut pos = (0, 0);
    let mut dir = Direction::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(pos);
    let result = input.split(", ").try_for_each(|ins| {
        match ins.chars().next() {
            Some('R') => dir = turn_right(dir),
            Some('L') => dir = turn_left(dir),
            _ => panic!("Invalid instruction: {}", ins),
        }
        let dist = ins[1..].parse().unwrap_or(0);
        for _ in 0..dist {
            pos = move_forward_step(pos, dir);
            if visited.contains(&pos) {
                return ControlFlow::Break(pos.0.abs() + pos.1.abs());
            } else {
                visited.insert(pos);
            }
        }
        ControlFlow::Continue(())
    });
    match result {
        ControlFlow::Break(dist) => dist,
        ControlFlow::Continue(()) => 0,
    }
}

fn move_forward(pos: (i32, i32), dir: Direction, dist: i32) -> (i32, i32) {
    match dir {
        Direction::North => (pos.0, pos.1 + dist),
        Direction::East => (pos.0 + dist, pos.1),
        Direction::South => (pos.0, pos.1 - dist),
        Direction::West => (pos.0 - dist, pos.1)
    }
}

fn move_forward_step(pos: (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (pos.0, pos.1 + 1),
        Direction::East => (pos.0 + 1, pos.1),
        Direction::South => (pos.0, pos.1 - 1),
        Direction::West => (pos.0 - 1, pos.1)
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    }
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(part1("R2, L3"), 5);
    }

    #[test]
    fn test_b() {
        assert_eq!(part1("R2, R2, R2"), 2);
    }

    #[test]
    fn test_c() {
        assert_eq!(part1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_d() {
        assert_eq!(part2("R8, R4, R4, R8"), 4);
    }

    #[cfg(feature = "inputfile")]
    #[test]
    fn part1_inputfile() {
        use crate::utils::print_solution;
        print_solution(2016, 1, 1, part1);
    }

    #[cfg(feature = "inputfile")]
    #[test]
    fn part2_inputfile() {
        use crate::utils::print_solution;
        print_solution(2016, 1, 2, part2);
    }
}
