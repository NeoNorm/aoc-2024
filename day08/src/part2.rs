

use std::collections::{HashMap, HashSet};
use std::iter::successors;
use glam::{IVec2,ivec2};
use itertools::Itertools;

fn parse(input: &str) -> HashMap<char,Vec<IVec2>> {
    let mut result: HashMap<char, Vec<IVec2>> = HashMap::new();

    input.lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, chr)| *chr != '.')
                .for_each(|(x, chr)| {
                    let position: IVec2 = ivec2(x as i32,y as i32);
                    result.entry(chr).or_default().push(position);
            });
    });

    result
}

fn parse_bounds(input: &str) -> IVec2 {
    ivec2(input.lines().next().unwrap().chars().count() as i32, input.lines().count() as i32)
}


pub fn go(input: &str) -> usize {

    let bounds = parse_bounds(&input);
    let in_bounds = |position: &IVec2| { position.x >= 0 && position.x < bounds.x && position.y >= 0 && position.y < bounds.y };

    let all_antinodes: HashSet<IVec2> = parse(input).iter().flat_map(|antenna_group| 
        antenna_group.1.iter()
            .combinations(2)
            .map(|antenna_pair| (*antenna_pair.get(0).unwrap(), *antenna_pair.get(1).unwrap()))
            .map(|antenna_pair| (antenna_pair, *antenna_pair.1 - *antenna_pair.0))
            .flat_map(|antenna_pair| { 
                successors(Some(*antenna_pair.0.1), move |position| Some(*position - antenna_pair.1))
                    .take_while(in_bounds)
                    .chain(
                        successors(Some(*antenna_pair.0.0), move |position| Some(*position + antenna_pair.1))
                            .take_while(in_bounds))
                //asdf.chain(qwer)//.collect::<Vec<IVec2>>()
            })
            .filter(|antinode| antinode.x >= 0 && antinode.x < bounds.x && antinode.y >= 0 && antinode.y < bounds.y)
        ).collect();
    
    all_antinodes.len()
}


#[cfg(test)]
mod tests {

    use crate::INPUT;
    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_input() {
        assert_eq!(super::go(TEST_INPUT), 34);
    }

}


