use std::iter::zip;

fn split_on_space_to_two_uint(input: &str) -> (usize, usize) {
    let mut split = input.split_once(' ').unwrap();
    split = (split.0.trim(), split.1.trim());

    (split.0.parse::<usize>().unwrap(), split.1.parse::<usize>().unwrap())
}

fn columnize_item(mut collection: (Vec<usize>, Vec<usize>), value: (usize, usize)) -> (Vec<usize>, Vec<usize>) {
    collection.0.push(value.0);
    collection.1.push(value.1);
    collection
}

#[allow(dead_code)]
pub fn go_1(input: &str) -> usize {

    let (mut column1, mut column2) = input.lines()
        .map(split_on_space_to_two_uint)
        .fold((vec![], vec![]), columnize_item);

    column1.sort();
    column2.sort();
    
    let sum_of_difference = column1.iter().zip(column2.iter())
        .map(|pair| pair.0.abs_diff(*pair.1))
        .sum::<usize>();

    sum_of_difference
}

pub fn go_2(input: &str) -> usize {
    let mut result: usize = 0;

    let mut col_1: Vec<usize> = Vec::with_capacity(1000);
    let mut col_2: Vec<usize> = Vec::with_capacity(1000);

    for line in input.lines() {
        let mut splitter = line.split_ascii_whitespace();

        col_1.push(splitter.next().unwrap().parse().unwrap());
        col_2.push(splitter.next().unwrap().parse().unwrap());
    }

    col_1.sort();
    col_2.sort();

    for pair in zip(&col_1, &col_2) {
        //println!("{:?}", pair);
        result += pair.0.abs_diff(*pair.1);
    }
    
    result
}

#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use crate::INPUT;

    #[test]
    fn test_1() {
        assert_eq!(go_1(INPUT), 1666427);
    }
    #[test]
    fn test_2() {
        assert_eq!(go_2(INPUT), 1666427);
    }
}
