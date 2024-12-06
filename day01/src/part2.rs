
use std::collections::HashMap;

fn split_on_space_to_two_uint(input: &str) -> (usize, usize) {
    let mut split = input.split_once(' ').unwrap();
    split = (split.0.trim(), split.1.trim());

    (split.0.parse::<usize>().unwrap(), split.1.parse::<usize>().unwrap())
}

pub fn go_1(input: &str) -> usize {
    let (column1, mut column2) = input.lines()
        .map(split_on_space_to_two_uint)
        .fold((vec![], HashMap::new()), |mut acc: (Vec<usize>, HashMap<usize,usize>), item| {
            acc.0.push(item.0);
            acc.1.entry(item.1).and_modify(|value| *value += 1).or_insert(1);
            acc
        });

    column1.iter().fold(0, |acc, item| acc + (*item * *column2.entry(*item).or_default())) 
}
