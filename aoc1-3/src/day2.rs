use std::collections::HashMap;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[String]) -> usize {
    let counts: Vec<LetterCounts> = input.iter().map(|s| count(s)).collect();

    let mut twos = 0;
    let mut threes = 0;
    for c in counts {
        if c.two {
            twos += 1;
        }
        if c.three {
            threes += 1;
        }
    }

    twos * threes
}

#[derive(Debug, PartialEq, Eq)]
struct LetterCounts {
    two: bool,
    three: bool,
}

fn count(input: &str) -> LetterCounts {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in input.chars() {
        let mut c = counts.entry(c).or_insert(0);
        *c += 1;
    }

    let mut ret = LetterCounts {
        two: false,
        three: false,
    };
    for &c in counts.values() {
        if c == 2 {
            ret.two = true;
        } else if c == 3 {
            ret.three = true;
        }
    }

    ret
}

#[aoc(day2, part2)]
fn solve_part2(input: &[String]) -> String {
    let vec: Vec<LetterSequence> = input.iter().map(|i| LetterSequence::new(i)).collect();

    for (i, v) in vec.iter().enumerate() {
        for w in vec.iter().skip(i) {
            if let Some(idx) = v.diff1(w) {
                let mut s: String = v.0.clone().into_iter().collect();
                s.remove(idx);
                return s;
            }
        }
    }

    String::new()
}

#[derive(Debug)]
struct LetterSequence(Vec<char>);

impl LetterSequence {
    fn new(s: &str) -> Self {
        LetterSequence(s.chars().collect())
    }

    fn diff1(&self, other: &LetterSequence) -> Option<usize> {
        let mut diff_count = 0;
        let mut diff_index = 0;

        for (idx, (&x, &y)) in self.0.iter().zip(other.0.iter()).enumerate() {
            if x != y {
                diff_count += 1;
                diff_index = idx;
            }

            if diff_count >= 2 {
                return None;
            }
        }

        if diff_count == 1 {
            Some(diff_index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let inputs = vec![
            ("abcdef", false, false),
            ("bababc", true, true),
            ("abbcde", true, false),
            ("abcccd", false, true),
            ("aabcdd", true, false),
            ("abcdee", true, false),
            ("ababab", false, true),
        ];

        for i in inputs {
            assert_eq!(
                count(i.0),
                LetterCounts {
                    two: i.1,
                    three: i.2
                }
            );
        }
    }

    #[test]
    fn test_solve1() {
        let input = [
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab"),
        ];
        assert_eq!(solve_part1(&input), 12);
    }

    #[test]
    fn test_solve2() {
        let input: Vec<String> = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]
        .into_iter()
        .map(|&s| String::from(s))
        .collect();

        assert_eq!(solve_part2(&input), String::from("fgij"));
    }
}
