use rayon::prelude::*;

#[repr(u8)]
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

pub fn go_1(input: &str) -> usize {
    let (grid, bounds) = load_grid(&input);
    let mut result: usize = 0;
    let tests = ((&XMAS::M, &XMAS::S), (&XMAS::S, &XMAS::M));
    
    for x in 1..(bounds.0 - 1) {
        for y in 1..(bounds.1 - 1) {
            let grid_value_at = |x1, y1| grid.get(y1 * bounds.0 + x1).unwrap();
            if grid_value_at(x,y) == &XMAS::A {
                let diag_1 = (grid_value_at(x - 1, y - 1), grid_value_at(x + 1, y + 1));
                let diag_2 = (grid_value_at(x + 1, y - 1), grid_value_at(x - 1, y + 1));
                if (diag_1 == tests.0 || diag_1 == tests.1) && (diag_2 == tests.0 || diag_2 == tests.1) {
                    result += 1;
                }
            }
        }
    }

    result
}

const M: &u8 = &"M".as_bytes()[0];
const A: &u8 = &"A".as_bytes()[0];
const S: &u8 = &"S".as_bytes()[0];

pub fn go_2(input: &str) -> usize {
    let line_length = input.find('\n').unwrap() + 1;
    let input_bytes = input.as_bytes();
    let mut result: usize = 0;
    
    for (i, c) in input_bytes.iter().enumerate().skip(line_length).take(input_bytes.len() - (2 * line_length)) {
        if i / line_length == 0 || i / line_length > line_length - 2 || c != A {
           continue 
        }
        if match &input_bytes[i - line_length - 1] {
            M => &input_bytes[i + line_length + 1] == S,
            S => &input_bytes[i + line_length + 1] == M,
            _ => continue,
        } {
            match &input_bytes[i - line_length + 1] {
                M if &input_bytes[i + line_length - 1] == S => result += 1,
                S if &input_bytes[i + line_length - 1] == M => result += 1,
                _ => continue
            }
        }
    }

    result
}

pub fn go_3(input: &str) -> usize {
    let line_length = input.find('\n').unwrap() + 1;
    let input_bytes = input.as_bytes();

    let result: usize = input_bytes.par_iter()
        .enumerate()
        .skip(line_length)
        .take(input_bytes.len() - (2 * line_length))
        .filter(|(i, c)| i / line_length != 0 && i / line_length < line_length - 1 && *c == A)
        //.inspect(|(i, c)| println!("{}", i))
        .map(|(i, _)| {
            if match &input_bytes[i - line_length - 1] {
                M => &input_bytes[i + line_length + 1] == S,
                S => &input_bytes[i + line_length + 1] == M,
                _ => false,
            } {
                match &input_bytes[i - line_length + 1] {
                    M if &input_bytes[i + line_length - 1] == S => 1,
                    S if &input_bytes[i + line_length - 1] == M => 1,
                    _ => 0
                }
            } else { 0 }
        })
        .sum();

    result
}

pub fn go_4(input: &str) -> usize {
    let line_length = input.find('\n').unwrap() + 1;
    let input_transformed = input.as_bytes();

    let mut result: usize = 0;

    for i in line_length..(input_transformed.len() - line_length) {
        if input_transformed[i] == 65 &&
           input_transformed[i - line_length - 1] ^ input_transformed[i + line_length + 1] == 30 &&
           input_transformed[i - line_length + 1] ^ input_transformed[i + line_length - 1] == 30 {
               result += 1;
        }
    }
    result
}

pub fn go_5(input: &str) -> usize {
    let line_length = input.find('\n').unwrap() + 1;
    let input_transformed = input.as_bytes();

    let result: usize = input_transformed.par_iter()
        .enumerate()
        .skip(line_length)
        .take(input_transformed.len() - (2 * line_length))
        .filter(|(i, c)| **c == 65 &&
           input_transformed[i - line_length - 1] ^ input_transformed[i + line_length + 1] == 30 &&
           input_transformed[i - line_length + 1] ^ input_transformed[i + line_length - 1] == 30)
        .count();
    //for i in line_length..(input_transformed.len() - line_length) {
    //    if input_transformed[i] == 65 &&
    //       input_transformed[i - line_length - 1] ^ input_transformed[i + line_length + 1] == 30 &&
    //       input_transformed[i - line_length + 1] ^ input_transformed[i + line_length - 1] == 30 {
    //           result += 1;
    //    }
    //}
    result
}

mod tests {
    #[allow(dead_code)]
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_1() {
        assert_eq!(super::go_1(INPUT), 1939);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::go_2(INPUT), 1939);
    }

    #[test]
    fn test_3() {
        assert_eq!(super::go_3(INPUT), 1939);
    }

    #[test]
    fn test_4() {
        assert_eq!(super::go_4(INPUT), 1939);
    }

    #[test]
    fn test_5() {
        assert_eq!(super::go_5(INPUT), 1939);
    }
    
    #[test]
    fn len_of_input() {
        println!("{:?}", INPUT.find('\n'));
        println!("{}", INPUT.len());
    }
}
