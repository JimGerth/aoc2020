use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use regex::Regex;

use super::input;

pub fn part_1() -> i32 {
    let input = input::read_lines(7);
    let mut can_contain_rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut contained_in_rules: HashMap<&str, Vec<&str>> = HashMap::new();

    let bag_color_regex = Regex::new(r"^\w* \w*").unwrap();
    let bag_rules_regex = Regex::new(r"bags contain (.*).").unwrap();
    let bag_rule_regex = Regex::new(r"\d+ (\w+ \w+) \w+").unwrap();

    // Parse rules.
    for line in &input {
        let bag_color = bag_color_regex.captures(line).unwrap().get(0).unwrap().as_str();
        let bag_rules = bag_rules_regex.captures(line).unwrap().get(1).unwrap().as_str();

        let bag_rules: Vec<&str> = if bag_rules != "no other bags" {
            bag_rules.split(", ").map(|rule| bag_rule_regex.captures(rule).unwrap().get(1).unwrap().as_str()).collect()
        } else {
            Vec::new()
        };

        can_contain_rules.insert(bag_color, bag_rules);
        contained_in_rules.insert(bag_color, Vec::new());
    }

    // println!("{:#?}", can_contain_rules);

    // Invert rules.
    for (color, contained_colors) in can_contain_rules.iter() {
        for contained_color in contained_colors {
            contained_in_rules.get_mut(contained_color).unwrap().push(color);
        }
    }

    // println!("{:#?}", contained_in_rules);

    let containers: HashSet<&str> = HashSet::from_iter(get_containers("shiny gold", &contained_in_rules));

    // println!("{:#?}", containers);

    containers.len() as i32
}

fn get_containers<'a>(color: &'a str, contained_in_rules: &'a HashMap<&str, Vec<&str>>) -> Vec<&'a str> {
    let directly_contained_in = contained_in_rules.get(color).expect("Color not listed in rules.");
    let mut contained_in = directly_contained_in.clone();
    for color in directly_contained_in {
        contained_in.append(&mut get_containers(color, contained_in_rules));
    }
    contained_in
}

pub fn part_2() -> i32 {
    5
}
