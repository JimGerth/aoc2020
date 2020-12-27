use super::input;

pub fn part_1() -> i32 {
    let codes = input::read_lines(5);

    let mut max_id = -1;

    for code in codes {
        let id = get_id(&code);
        if id > max_id {
            max_id = id;
        }
    }

    max_id
}

pub fn part_2() -> Option<i32> {
    let codes = input::read_lines(5);

    let mut ids: Vec<i32> = codes.iter().map(|code| get_id(code)).collect();
    ids.sort();

    for (i, id) in ids[..ids.len() - 1].iter().enumerate() {
        if id + 2 == ids[i + 1] {
            return Some(id + 1);
        }
    }

    None
}

fn get_id(code: &str) -> i32 {
    let mut row = 0;
    let mut col = 0;

    for (i, letter) in code.chars().enumerate() {
        match letter {
            'B' => row += 1 << 6 - i,
            'R' => col += 1 << 2 - (i - 7),
            _ => {}
        }
    }

    row * 8 + col
}
