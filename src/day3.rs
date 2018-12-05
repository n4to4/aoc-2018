use regex::Regex;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

const MAX_ID: usize = 2048;
const BOARD_SIZE: usize = 1024;

#[derive(Debug, PartialEq, Eq)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Claim> {
    input.lines().map(|l| parse_claim(l).unwrap()).collect()
}

fn parse_claim(s: &str) -> Result<Claim> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\A#(\d+) @ (\d+),(\d+): (\d+)x(\d+)\z").unwrap();
    }

    let cap = (*RE).captures(s).unwrap();

    Ok(Claim {
        id: cap[1].parse()?,
        x: cap[2].parse()?,
        y: cap[3].parse()?,
        width: cap[4].parse()?,
        height: cap[5].parse()?,
    })
}

type Board = [[u8; BOARD_SIZE]; BOARD_SIZE];

#[aoc(day3, part1)]
fn solve_part1(input: &[Claim]) -> usize {
    let mut board: Board = [[0u8; BOARD_SIZE]; BOARD_SIZE];

    input.iter().for_each(|c| paint(&mut board, c));
    count(&board)
}

fn paint(board: &mut Board, claim: &Claim) {
    for x in claim.x..(claim.x + claim.width) {
        for y in claim.y..(claim.y + claim.height) {
            board[x][y] += 1;
        }
    }
}

fn count(board: &Board) -> usize {
    let mut count = 0;

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if board[x][y] >= 2 {
                count += 1;
            }
        }
    }

    count
}

type Board2 = Box<[[u16; BOARD_SIZE]; BOARD_SIZE]>;

#[aoc(day3, part2)]
fn solve_part2(input: &[Claim]) -> usize {
    let mut board: Board2 = Box::new([[0u16; BOARD_SIZE]; BOARD_SIZE]);
    let mut survivors: [bool; MAX_ID] = [false; MAX_ID];

    input
        .iter()
        .for_each(|c| paint2(&mut board, &mut survivors, c));

    for (id, &survived) in survivors.iter().enumerate() {
        if survived {
            return id;
        }
    }

    panic!("should not happen");
}

fn paint2(board: &mut Board2, survivors: &mut [bool; MAX_ID], claim: &Claim) {
    let mut clean = true;

    for x in claim.x..(claim.x + claim.width) {
        for y in claim.y..(claim.y + claim.height) {
            if board[x][y] == 0 {
                board[x][y] = claim.id as u16;
            } else {
                clean = false;

                let id = board[x][y];
                survivors[id as usize] = false;
            }
        }
    }

    survivors[claim.id] = clean;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_claim("#1 @ 338,764: 20x24").unwrap(),
            Claim {
                id: 1,
                x: 338,
                y: 764,
                width: 20,
                height: 24,
            }
        );
        assert_eq!(
            parse_claim("#2 @ 80,667: 12x26").unwrap(),
            Claim {
                id: 2,
                x: 80,
                y: 667,
                width: 12,
                height: 26,
            }
        );
    }

    #[test]
    fn solver() {
        let claims = vec![
            Claim {
                id: 1,
                x: 1,
                y: 3,
                width: 4,
                height: 4,
            },
            Claim {
                id: 2,
                x: 3,
                y: 1,
                width: 4,
                height: 4,
            },
            Claim {
                id: 3,
                x: 5,
                y: 5,
                width: 2,
                height: 2,
            },
        ];

        let count = solve_part1(&claims);
        assert_eq!(count, 4);
    }

    #[test]
    fn test() {}
}
