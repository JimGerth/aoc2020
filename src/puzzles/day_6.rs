use std::collections::HashMap;

use super::input;

pub fn part_1() -> i32 {
    let groups = input::read_paragraphs(6);

    let mut sum = 0;

    for group in groups {
        let mut answers = HashMap::new();

        for answer in group.chars() {
            if answer.is_alphabetic() {
                answers.insert(answer, 1);
            }
        }

        sum += answers.values().sum::<i32>();
    }

    sum
}

pub fn part_2() -> i32 {
    let groups = input::read_paragraphs(6);

    let mut sum: i32 = 0;

    for group in groups {
        let mut persons = group.lines();
        let mut answers = String::from(persons.next().unwrap());

        for person in persons {
            let mut remaining_answers = String::new();

            for answer in answers.chars() {
                if person.contains(answer) {
                    remaining_answers.push(answer);
                }
            }

            answers = remaining_answers;
        }

        sum += answers.len() as i32;
    }

    sum
}
