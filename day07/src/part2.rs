
use rayon::prelude::*;

pub fn go(input: &str) -> isize {

    input
        .lines()
        .par_bridge()
        .filter_map(|line| line.split_once(": ") )
        .map(|line2| (
                line2.0.parse::<isize>().unwrap(), 
                line2.1.split_whitespace().map(|k| k.parse().unwrap()).collect::<Vec<isize>>()))
        .filter(|eq| { solver(&eq.1, eq.0) })
        .map(|eq| eq.0)
        .sum()
}

fn solver(operands: &[isize], result: isize) -> bool {
    let operand = operands.last().unwrap();
    
    if operands.len() <= 2 {
        return  
            ( result / operand == operands[0] && result % operand == 0 ) 
            || result - operand == operands[0]
            || format!("{}{}", operands[0], *operand).parse::<isize>().unwrap() == result;
    } 

    solver(&operands[0..&operands.len() - 1], result - operand)
    || if (result - operand) % 10_isize.pow(operand.ilog10() + 1) == 0 {
            solver(&operands[0..&operands.len() - 1], (result - operand) / 10_isize.pow(operand.ilog10() + 1))
        } else { false }
    || if result % operand == 0 {
            solver(&operands[0..&operands.len() - 1], result / operand)
        } else { false }
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
    fn test_test_input() {
        assert_eq!(super::go(TEST_INPUT), 11387);
    }

    #[test]
    fn test_input() {
        assert_eq!(super::go(INPUT), 162042343638683);
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
    fn solver_concat_simple() {
        assert!(super::solver(&vec![15,6], 156));
    }
    #[test]
    fn solver_concat_recurse() {
        assert!(super::solver(&vec![6,8,6,15], 7290));
    }

}

