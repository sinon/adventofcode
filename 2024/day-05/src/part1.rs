use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct PageNum(i32);
struct Afters(Vec<PageNum>);

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut rules: HashMap<PageNum, Afters> = HashMap::new();
    let mut updates: Vec<Vec<PageNum>> = Vec::new();
    for ln in _input.lines() {
        let layout: Vec<&str> = ln.split("|").collect();
        if layout.len() == 2 {
            let (start, end) = (
                layout[0].parse::<i32>().unwrap(),
                layout[1].parse::<i32>().unwrap(),
            );
            rules
                .entry(PageNum(start))
                .and_modify(|afters| afters.0.push(PageNum(end)))
                .or_insert(Afters(vec![PageNum(end)]));
            continue;
        }
        if ln.len() > 1 {
            updates.push(
                ln.split(",")
                    .map(|x| PageNum(x.parse::<i32>().unwrap()))
                    .collect(),
            );
        }
    }

    let result: i32 = updates
        .iter()
        .filter(|update| {
            update.is_sorted_by(|a, b| rules.get(a).is_some_and(|pages| pages.0.contains(b)))
        })
        .map(|update| {
            let middle = update.len() / 2;
            update[middle].0
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
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
        assert_eq!(143, process(input)?);
        Ok(())
    }
}
