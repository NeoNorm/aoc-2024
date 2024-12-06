
pub fn go_1(input: &str) -> usize {
    let asdf = input.lines()
        .map(split_on_space_to_uint)
        .map(|items| {
            items.windows(2).map(|item| item[1] as isize - item[0] as isize).collect::<Vec<isize>>() })
        .filter(|items| {
            items.iter().all(|item| *item > 0 && *item <= 3) 
            || items.iter().all(|item| *item < 0 && *item >= -3) })
        .count();
        
    asdf
}

fn split_on_space_to_uint(input: &str) -> Vec<usize> {
    input.split_whitespace()
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
