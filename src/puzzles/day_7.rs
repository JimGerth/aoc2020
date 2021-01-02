use std::collections::{HashMap, HashSet};

use regex::Regex;

use super::input;

pub fn part_1() -> i32 {
    let input = input::read_lines(7);
    let mut can_contain_rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut contained_in_rules: HashMap<&str, HashSet<&str>> = HashMap::new();

    let bag_color_regex = Regex::new(r"^\w* \w*").unwrap();
    let bag_rules_regex = Regex::new(r"bags contain (.*).").unwrap();
    let bag_rule_regex = Regex::new(r"\d+ (\w+ \w+) \w+").unwrap();

    // Parse rules.
    for line in &input {
        let bag_color = bag_color_regex
            .captures(line)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        let bag_rules = bag_rules_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let bag_rules: HashSet<&str> = if bag_rules != "no other bags" {
            bag_rules
                .split(", ")
                .map(|rule| {
                    bag_rule_regex
                        .captures(rule)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                })
                .collect()
        } else {
            HashSet::new()
        };

        can_contain_rules.insert(bag_color, bag_rules);
        contained_in_rules.insert(bag_color, HashSet::new());
    }

    // println!("{:#?}", can_contain_rules);

    // Invert rules.
    for (color, contained_colors) in can_contain_rules.iter() {
        for contained_color in contained_colors {
            contained_in_rules
                .get_mut(contained_color)
                .unwrap()
                .insert(color);
        }
    }

    // println!("{:#?}", contained_in_rules);

    let containers = get_containers("shiny gold", &contained_in_rules);

    // println!("{:#?}", containers);

    containers.len() as i32
}

fn get_containers<'a>(
    color: &'a str,
    contained_in_rules: &'a HashMap<&str, HashSet<&str>>,
) -> HashSet<&'a str> {
    let directly_contained_in = contained_in_rules
        .get(color)
        .expect("Color not listed in rules.");
    let mut contained_in = directly_contained_in.clone();
    for color in directly_contained_in {
        contained_in = contained_in
            .union(&get_containers(color, contained_in_rules))
            .cloned()
            .collect();
    }
    contained_in
}

pub fn part_2() -> i32 {
    let input = input::read_lines(7);
    let mut rules: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    let bag_color_regex = Regex::new(r"^\w* \w*").unwrap();
    let bag_rules_regex = Regex::new(r"bags contain (.*).").unwrap();
    let bag_rule_regex = Regex::new(r"(\d+) (\w+ \w+) \w+").unwrap();

    // Parse rules.
    for line in &input {
        let bag_color = bag_color_regex
            .captures(line)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        let bag_rules = bag_rules_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let bag_rules: HashMap<&str, i32> = if bag_rules != "no other bags" {
            bag_rules
                .split(", ")
                .map(|rule| {
                    (
                        bag_rule_regex
                            .captures(rule)
                            .unwrap()
                            .get(2)
                            .unwrap()
                            .as_str(),
                        bag_rule_regex
                            .captures(rule)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse()
                            .unwrap(),
                    )
                })
                .collect()
        } else {
            HashMap::new()
        };

        rules.insert(bag_color, bag_rules);
    }

    // println!("{:#?}", rules);

    get_content_count("shiny gold", &rules)
}

fn get_content_count(color: &str, rules: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let rule = rules.get(color).expect("Color not listed in rules.");
    let mut total = 0;
    for (color, count) in rule {
        total += count + count * get_content_count(color, &rules);
    }
    total
}
