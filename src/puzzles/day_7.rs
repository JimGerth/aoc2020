use std::collections::{HashMap, HashSet};

use regex::Regex;

use super::input;

pub fn part_1() -> i32 {
    let input = input::read_lines(7);
    let contains_rules = parse_rules(input.iter().map(|line| line.as_str()).collect());
    let mut contained_in_rules: HashMap<&str, HashSet<&str>> = HashMap::new();

    // Invert rules.
    for (container_color, contains_rule) in contains_rules {
        // println!("{:?} contains", container_color);
        for (contained_color, _) in contains_rule {
            // println!("\t{:?}", contained_color);
            match contained_in_rules.get_mut(contained_color) {
                Some(contained_in_rule) => {
                    contained_in_rule.insert(container_color);
                }
                None => {
                    let mut contained_in_rule = HashSet::new();
                    contained_in_rule.insert(container_color);
                    contained_in_rules.insert(contained_color, contained_in_rule);
                }
            }
        }
    }

    println!("{:#?}", contained_in_rules);

    let containers = get_containers("shiny gold", &contained_in_rules);

    // println!("{:#?}", containers);

    containers.len() as i32
}

pub fn part_2() -> i32 {
    let input = input::read_lines(7);
    let rules = parse_rules(input.iter().map(|line| line.as_str()).collect());
    get_content_count("shiny gold", &rules)
}

fn get_containers<'a>(
    color: &'a str,
    contained_in_rules: &'a HashMap<&str, HashSet<&str>>,
) -> HashSet<&'a str> {
    let directly_contained_in = match contained_in_rules.get(color) {
        Some(contained_in) => contained_in.clone(),
        None => HashSet::new(),
    };
    let mut contained_in = directly_contained_in.clone();
    for color in directly_contained_in {
        contained_in = contained_in
            .union(&get_containers(color, contained_in_rules))
            .cloned()
            .collect();
    }
    contained_in
}

fn get_content_count(color: &str, rules: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let rule = rules.get(color).expect("Color not listed in rules.");
    let mut total = 0;
    for (color, count) in rule {
        total += count + count * get_content_count(color, &rules);
    }
    total
}

fn parse_rules(input: Vec<&str>) -> HashMap<&str, HashMap<&str, i32>> {
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

    rules
}
