use advent_of_code_data::registry::{Result, Solver, SolverPart};
use advent_of_code_data::{Answer, Day, Year};
use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(5),
    year: Year(2024),
    part_one: SolverPart {
        func: day_5_1,
        examples: &[(
            Answer::Int(143),
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
97,13,75,29,47",
        )],
    },
    part_two: SolverPart {
        func: day_5_2,
        examples: &[],
    },
};

#[derive(Debug)]
struct PageOrderingRule {
    before: usize,
    after: usize,
}

#[derive(Debug)]
struct Update {
    pages: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<PageOrderingRule>, Vec<Update>) {
    let mut page_ordering_rules: Vec<PageOrderingRule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let (before, after) = line.split_once('|').unwrap();
            page_ordering_rules.push(PageOrderingRule {
                before: before.parse::<usize>().unwrap(),
                after: after.parse::<usize>().unwrap(),
            });
        } else if line.contains(',') {
            updates.push(Update {
                pages: line
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            });
        }
    }

    (page_ordering_rules, updates)
}

fn is_update_ordered(pages_to_update: &[usize], rules: &[PageOrderingRule]) -> bool {
    pages_to_update
        .iter()
        .enumerate()
        .all(|(i, _page)| is_page_ordered(pages_to_update, i, rules))
}

fn is_page_ordered(
    pages_to_update: &[usize],
    page_index: usize,
    rules: &[PageOrderingRule],
) -> bool {
    let page = pages_to_update[page_index];

    rules
        .iter()
        .filter(|r| r.before == page)
        .all(|rule| is_page_following_rule(pages_to_update, page_index, rule))
}

fn is_page_following_rule(
    pages_to_update: &[usize],
    page_index: usize,
    rule: &PageOrderingRule,
) -> bool {
    // Does the "after" page appear before this page?
    !pages_to_update
        .iter()
        .take(page_index)
        .any(|prev_page| *prev_page == rule.after)
}

pub fn day_5_1(input: &str) -> Result<Answer> {
    let (page_ordering_rules, updates) = parse_input(input);
    let mut sum_of_middle_page_numers = 0;

    for update in updates.iter() {
        let is_ordered = is_update_ordered(&update.pages, &page_ordering_rules);
        //tracing::debug!("#{}: {is_ordered} for {:?}", i + 1, update.pages);

        if is_ordered {
            assert!(update.pages.len() % 2 == 1);
            let middle_index = update.pages.len() / 2;
            let middle_page = update.pages[middle_index];
            //tracing::debug!("middle page index {middle_index} is {middle_page}");

            sum_of_middle_page_numers += middle_page;
        }
    }

    Ok(sum_of_middle_page_numers.into())
}

pub fn day_5_2(input: &str) -> Result<Answer> {
    let (page_ordering_rules, mut updates) = parse_input(input);
    let mut sum_of_middle_page_numers = 0;

    for update in updates.iter_mut() {
        let is_ordered = is_update_ordered(&update.pages, &page_ordering_rules);
        let pages = &mut update.pages;

        if !is_ordered {
            // Try to fix all the ordering errors in the update.
            // Yes I know this is bubble sort but worse. O(kn^2) FTW.
            let mut swapped = true;
            let mut max_count = 100;

            while swapped {
                swapped = false;

                assert!(max_count > 0);
                max_count -= 1;

                for i in 0..pages.len() {
                    for j in i..pages.len() {
                        for r in &page_ordering_rules {
                            // page number X must be printed at some point before page number Y
                            if pages[i] == r.after && pages[j] == r.before {
                                //tracing::debug!("SWAP({i}, {j}): {} <=> {}", pages[i], pages[j]);
                                pages.swap(i, j);
                                swapped = true;

                                break;
                            }
                        }
                    }
                }
            }

            assert!(is_update_ordered(&update.pages, &page_ordering_rules));

            assert!(update.pages.len() % 2 == 1);
            let middle_index = update.pages.len() / 2;
            let middle_page = update.pages[middle_index];
            //tracing::debug!("middle page index {middle_index} is {middle_page}");

            sum_of_middle_page_numers += middle_page;
        }
    }

    Ok(sum_of_middle_page_numers.into())
}
