

fn parse(input: &str) -> Vec<(isize, Vec<isize>)> {
    
    input
        .lines()
        .filter_map(|line| line.split_once(": ") )
        .map(|line2| (
                line2.0.parse::<isize>().unwrap(), 
                line2.1.split_whitespace().map(|k| k.parse().unwrap()).collect::<Vec<isize>>()))
        .collect()
}

pub fn go(input: &str) -> isize {

    let result: isize = parse(input)
        .iter()
        .filter(|eq| { solver(&eq.1, eq.0) })
        .map(|eq| eq.0)
        .sum();

    result
}

fn solver(operands: &[isize], result: isize) -> bool {
    let operand = operands.last().unwrap();
    
    let mut found_solution = false;
    if operands.len() <= 2 {
        found_solution = ( result / operand == operands[0] && result % operand == 0 ) || result - operand == operands[0];
    } else {
        if result % operand == 0 {
            found_solution = solver(&operands[0..&operands.len() - 1], result / operand);
        }
        if !found_solution {
            found_solution = solver(&operands[0..&operands.len() - 1], result - operand);
        }
    }
    found_solution
}

#[cfg(test)]
mod tests {

    use crate::INPUT;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";


#[test]
    fn wtf() {
        super::go("97776: 6 3 9 6 57 679");

    }

    #[test]
    fn test_test_input() {
        assert_eq!(super::go(TEST_INPUT), 3749);
    }

    #[test]
    fn test_input() {
        assert_eq!(super::go(INPUT), 1260333054159);
    }

    #[test]
    fn solver_simple() {
        assert!(super::solver(&vec![10,19], 190));
    }
    #[test]
    fn solver_not_valid() {
        assert!(!super::solver(&vec![17,5], 83));
    }
    #[test]
    fn solver_valid_recurse() {
        assert!(super::solver(&vec![81,40,27], 3267));
    }
    #[test]
    fn solver_valid_recurse2() {
        assert!(super::solver(&vec![11,6,16,20], 292));
    }

    #[test]
    fn parse() {
        assert_eq!(super::parse(TEST_INPUT),vec![
(190, vec![10,19]),
(3267, vec![81,40,27]),
(83, vec![17,5]),
(156, vec![15,6]),
(7290, vec![6,8,6,15]),
(161011, vec![16,10,13]),
(192, vec![17,8,14]),
(21037, vec![9,7,18,13]),
(292, vec![11,6,16,20]),
        ])
    }

}

