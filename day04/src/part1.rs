#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum XMAS {
    X, M, A, S
}

fn load_grid(input: &str) -> (Vec<XMAS>, (usize, usize)) {
    let (mut x, mut y) = (0,1);

    let mut result: Vec<XMAS> = Vec::with_capacity(input.len());

    let mut input_lines = input.lines();
    let first_line = input_lines.next().unwrap();

    for input_char in first_line.chars() {
        x += 1;
        result.push(parse_XMAS_char(input_char));
    }
    for input_line in input_lines {
        for input_char in input_line.chars() {
            result.push(parse_XMAS_char(input_char));
        }
        y += 1;
    }

    (result, (x, y))
}

fn testable_directions(position: (usize,usize), bounds: (usize,usize)) -> Vec<(isize,isize)> {
    let mut result: Vec<(isize,isize)> = Vec::with_capacity(8);

    let diff = (bounds.0 - position.0, bounds.1 - position.1);
    if diff.0 >= 4 { 
        result.push((1,0));
        if diff.1 >= 4 { result.push((1, 1)); }
        if position.1 >= 3 { result.push((1, -1)); }
    }
    if position.0 >= 3 {
        result.push((-1, 0));
        if diff.1 >= 4 { result.push((-1, 1)); }
        if position.1 >= 3 { result.push((-1, -1)); }
    }
    if diff.1 >= 4 { result.push((0, 1)); }
    if position.1 >= 3 { result.push((0, -1)); }

    result
}

fn testable_indice_arrays(position: (usize,usize), bounds: (usize, usize)) -> Vec<[(usize, usize); 3]> {
    let mut result: Vec<[(usize, usize); 3]> = Vec::with_capacity(8);

    let diff = (bounds.0 - position.0, bounds.1 - position.1);
    if diff.0 >= 4 { 
        result.push([(position.0 + 1, position.1), (position.0 + 2, position.1), (position.0 + 3, position.1)]);
        if diff.1 >= 4 { result.push([(position.0 + 1, position.1 + 1), (position.0 + 2, position.1 + 2), (position.0 + 3, position.1 + 3)]); }
        if position.1 >= 3 { result.push([(position.0 + 1, position.1 - 1), (position.0 + 2, position.1 - 2),(position.0 + 3, position.1 - 3)]); }
    }
    if position.0 >= 3 {
        result.push([(position.0 - 1, position.1),(position.0 - 2, position.1),(position.0 - 3, position.1)]);
        if diff.1 >= 4 { result.push([(position.0 - 1, position.1 + 1),(position.0 - 2, position.1 + 2),(position.0 - 3, position.1 + 3)]); }
        if position.1 >= 3 { result.push([(position.0 - 1, position.1 - 1), (position.0 - 2, position.1 - 2),(position.0 - 3, position.1 - 3)]); }
    }
    if diff.1 >= 4 { result.push([(position.0, position.1 + 1),(position.0, position.1 + 2), (position.0, position.1 + 3)]); }
    if position.1 >= 3 { result.push([(position.0, position.1 - 1),(position.0, position.1 - 2),(position.0, position.1 - 3)]); }

    result
}

pub fn go_1(input: &str) -> usize {
    let (grid, bounds) = load_grid(&input);
    let mut result: usize = 0;

    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            let grid_value_at = |x1: isize, y1: isize| grid.get(y1 as usize * bounds.0 + x1 as usize).unwrap();
            let (x, y) = (x as isize, y as isize);
            if grid_value_at(x,y) == &XMAS::X {
                let directions = testable_directions((x as usize, y as usize), bounds);
                for direction in directions {
                    if grid_value_at(x + direction.0, y + direction.1) == &XMAS::M &&
                        grid_value_at(x + (2 * direction.0), y + (2 * direction.1)) == &XMAS::A &&
                        grid_value_at(x + (3 * direction.0), y + (3 * direction.1)) == &XMAS::S {
                        
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

pub fn go_2(input: &str) -> usize {
    let (grid, bounds) = load_grid(&input);
    let mut result: usize = 0;

    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            let grid_value_at = |x1: isize, y1: isize| grid.get(y1 as usize * bounds.0 + x1 as usize).unwrap();
            let (x, y) = (x as isize, y as isize);
            if grid_value_at(x,y) == &XMAS::X {
                let directions = testable_indice_arrays((x as usize, y as usize), bounds);
                for direction in directions {
                    if grid_value_at(direction[0].0 as isize, direction[0].1 as isize) == &XMAS::M &&
                        grid_value_at(direction[1].0 as isize, direction[1].1 as isize) == &XMAS::A &&
                        grid_value_at(direction[2].0 as isize, direction[2].1 as isize) == &XMAS::S {
                        
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

pub fn go_3(input: &str) -> usize {
    let (grid, bounds) = load_grid(&input);
    let mut result: usize = 0;

    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            let grid_value_at = |pos: (usize, usize)| grid.get(pos.1 * bounds.0 + pos.0).unwrap();
            if grid_value_at((x,y)) == &XMAS::X {
                let directions = testable_indice_arrays((x, y), bounds);
                for direction in directions {
                    if grid_value_at(direction[0]) == &XMAS::M &&
                        grid_value_at(direction[1]) == &XMAS::A &&
                        grid_value_at(direction[2]) == &XMAS::S {
                        
                        result += 1;
                    }
                }
            }
        }
    }

    result
}


#[allow(non_snake_case)]
fn parse_XMAS_char(c: char) -> XMAS {
    match c {
        'X' => XMAS::X,
        'M' => XMAS::M,
        'A' => XMAS::A,
        'S' => XMAS::S,
        _ => panic!("Not an XMAS char! : ({})", c),
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_parse() {

        assert_eq!(parse_XMAS_char('X'), XMAS::X);
        assert_eq!(parse_XMAS_char('M'), XMAS::M);
        assert_eq!(parse_XMAS_char('A'), XMAS::A);
        assert_eq!(parse_XMAS_char('S'), XMAS::S);

        for l in INPUT.lines() {
            for c in l.chars() {
                _ = parse_XMAS_char(c);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_panic() {
        parse_XMAS_char('Z');
    }

    #[test]
    fn test_load_grid_bounds() {
        let (_, bounds) = load_grid(INPUT);
        assert_eq!(bounds, (10, 10));
    }

    #[test]
    fn test_testable_directions() {
        assert_eq!(testable_directions((0,0), (3,3)).len(), 0);
        assert_eq!(testable_directions((0,0), (4,4)), vec![(1, 0), (1, 1), (0, 1)]);
        assert_eq!(testable_directions((3,3), (4,4)), vec![(-1, 0), (-1, -1), (0, -1)]);
        assert_eq!(testable_directions((3,3), (7,7)), vec![(1, 0), (1, 1), (1, -1), (-1, 0), (-1, 1), (-1, -1), (0, 1), (0, -1)]);
    }

    #[test]
    fn test_go_1() {
        assert_eq!(go_1(INPUT), 18)
    }

    #[test]
    fn run_go_1() {
        assert_eq!(go_1(include_str!("../input.txt")), 2547);
    }

    #[test]
    fn run_go_2() {
        assert_eq!(go_2(include_str!("../input.txt")), 2547);
    }

    #[test]
    fn run_go_3() {
        assert_eq!(go_3(include_str!("../input.txt")), 2547);
    }
}
