use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut seen: HashSet<i32> = vec![0].into_iter().collect();
    let mut current: i32 = 0;

    for i in input.iter().cycle() {
        current += i;
        if !seen.insert(current) {
            return current;
        }
    }

    panic!("Never reached");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(solve_part1(&[1, 1, 1]), 3);
    }

    #[test]
    fn example1_2() {
        assert_eq!(solve_part1(&[1, 1, -2]), 0);
    }

    #[test]
    fn example1_3() {
        assert_eq!(solve_part1(&[-1, -2, -3]), -6);
    }

    #[test]
    fn example2_1() {
        assert_eq!(solve_part2(&[1, -1]), 0);
    }

    #[test]
    fn example2_2() {
        assert_eq!(solve_part2(&[3, 3, 4, -2, -4]), 10);
    }

    #[test]
    fn example2_3() {
        assert_eq!(solve_part2(&vec![-6, 3, 8, 5, -6]), 5);
    }

    #[test]
    fn example2_4() {
        assert_eq!(solve_part2(&vec![7, 7, -2, -7, -4]), 14);
    }
}
