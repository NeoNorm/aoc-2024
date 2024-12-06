


fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    
    let mut lines = input.lines();

    let mut rules: Vec<(usize,usize)> = vec![];

    loop {
        let line = lines.next().unwrap();

        if let Some(rule_input) = line.split_once('|') {
            rules.push(parse_rule(rule_input));
        } else { break; }
    }

    let mut updates: Vec<Vec<usize>> = vec![];

    for line in lines {
        updates.push(line.split(',').map(|page| page.parse::<usize>().unwrap()).collect::<Vec<usize>>());
    }

    (rules, updates)
}

fn parse_rule(input: (&str,&str)) -> (usize,usize) {
    (input.0.parse().unwrap(), input.1.parse().unwrap())
}

pub fn go_1(input: &str) -> usize {
    let mut result: usize = 0;
    let (rules, updates) = parse(input);

    for update in updates {
        let applied_rules: Vec<&(usize,usize)> = rules.iter().filter(|rule| update.contains(&rule.0) && update.contains(&rule.1)).collect();

        //println!("Update");
        //for page in update {
        //    println!("Page({}): {}", page, applied_rules.iter().filter(|rule| rule.0 == page).count());
        //}

        let mut page_suffix_count: Vec<(usize, usize)> = update
            .iter()
            .map(|page| {
                (*page, applied_rules.iter().filter(|rule| rule.0 == *page).count())
            })
            .collect();
        page_suffix_count.sort_by_key(|item| item.1);
        page_suffix_count.reverse();
        if page_suffix_count.clone().into_iter().unzip::<usize,usize,Vec<usize>,Vec<usize>>().0 != update {
            result += page_suffix_count.get(page_suffix_count.len() / 2).unwrap().0;
        }
        
    }

    result
}

mod tests {

    #[test]
    fn aoc_test() {
        let input = 
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(super::go_1(input), 123);
    }
}
