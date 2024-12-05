#[derive(Debug)]
struct OrderingRule {
    pre: i32,
    post: i32,
}
#[derive(Debug)]
struct PageUpdates {
    values: Vec<i32>,
}

impl PageUpdates {
    fn middle(&self) -> i32 {
        self.values[((self.values.len() - 1) as f64 / 2.0).round() as usize]
    }
    fn abides(&self, rule: &OrderingRule) -> bool {
        // Rule only applies if both numbers are present
        if self.values.contains(&rule.pre) && self.values.contains(&rule.post) {
            let mut pre_found = false;
            let mut post_found = false;
            for v in &self.values {
                if *v == rule.post && !pre_found {
                    return false;
                }
                if *v == rule.pre && post_found {
                    return false;
                }
                if *v == rule.post && pre_found {
                    post_found = true;
                    continue;
                }
                if *v == rule.pre && !post_found {
                    pre_found = true;
                    continue;
                }
            }
        }
        true
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
    let mut rules: Vec<OrderingRule> = Vec::new();
    let mut updates: Vec<PageUpdates> = Vec::new();
    for ln in _input.lines() {
        let layout: Vec<&str> = ln.split("|").collect();
        if layout.len() == 2 {
            let (start, end) = (
                layout[0].parse::<i32>().unwrap(),
                layout[1].parse::<i32>().unwrap(),
            );
            rules.push(OrderingRule {
                pre: start,
                post: end,
            });
            continue;
        }
        if ln.len() > 1 {
            updates.push(PageUpdates {
                values: ln.split(",").map(|x| x.parse::<i32>().unwrap()).collect(),
            });
        }
    }
    let mut sum = 0;
    for update in updates {
        let mut failed = false;
        for r in &rules {
            if !update.abides(r) {
                failed = true;
                break;
            }
        }
        if !failed {
            sum += update.middle();
        }
    }
    Ok(sum)
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
