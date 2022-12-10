pub fn get_top_stacks<'a>() -> &'a str {
    let stacks = get_stacks();
    let step_lines = STEPS.split('\n').collect::<Vec<&str>>();
    let steps = Vec::new();
    for step_line in step_lines {
        let words = step_line.split(" ");
        let nums = Vec::new();
        for word in words.into_iter() {
            if word.chars().all(|x| x.is_numeric()) {
                nums.push(word.parse());
            }
        }
    }
    println!("Stacks: {:?}", stacks);

    "hi"
}

fn get_stacks() -> Vec<Vec<char>> {
    let data_lines = DATA.split('\n').collect::<Vec<&str>>();
    let mut number_indices = Vec::new();
    let last_line = data_lines
        .last()
        .expect("No last line")
        .chars()
        .collect::<Vec<char>>();
    for (i, c) in last_line.iter().enumerate() {
        if c.is_numeric() {
            number_indices.push((c.to_digit(10).expect("Can't parse digit") as usize, i));
        }
    }

    let mut stacks = Vec::new();
    for _i in 0..data_lines.len() {
        stacks.push(Vec::new());
    }

    for i in (0..=(data_lines.len() - 2)).rev() {
        let line = data_lines[i].chars().collect::<Vec<char>>();
        for (number, number_index) in &number_indices {
            let index = number_index;
            let char = line[*index];

            if char.is_alphabetic() {
                stacks[number - 1].push(char);
            }
        }
    }

    stacks
}

struct Step {
    pub number: u8,
    pub from: u8,
    pub to: u8,
}

const DATA: &str = "[N]             [R]             [C]
[T] [J]         [S] [J]         [N]
[B] [Z]     [H] [M] [Z]         [D]
[S] [P]     [G] [L] [H] [Z]     [T]
[Q] [D]     [F] [D] [V] [L] [S] [M]
[H] [F] [V] [J] [C] [W] [P] [W] [L]
[G] [S] [H] [Z] [Z] [T] [F] [V] [H]
[R] [H] [Z] [M] [T] [M] [T] [Q] [W]
 1   2   3   4   5   6   7   8   9 ";

const STEPS: &str = "move 3 from 9 to 7
move 4 from 4 to 5
move 2 from 4 to 6
move 4 from 7 to 5
move 3 from 7 to 3
move 2 from 5 to 9
move 5 from 6 to 3
move 5 from 9 to 1
move 3 from 8 to 4
move 3 from 4 to 6
move 8 from 1 to 8
move 1 from 8 to 6
move 2 from 8 to 2
move 5 from 8 to 4
move 1 from 8 to 1
move 6 from 6 to 4
move 1 from 7 to 9
move 5 from 1 to 7
move 1 from 1 to 2
move 2 from 9 to 8
move 6 from 4 to 9
move 1 from 6 to 8
move 3 from 2 to 7
move 4 from 2 to 8
move 4 from 9 to 3
move 6 from 5 to 4
move 7 from 8 to 1
move 10 from 4 to 1
move 12 from 1 to 5
move 1 from 4 to 9
move 1 from 2 to 3
move 2 from 9 to 1
move 1 from 9 to 3
move 1 from 6 to 7
move 1 from 9 to 1
move 3 from 1 to 3
move 9 from 5 to 9
move 2 from 2 to 7
move 2 from 7 to 4
move 3 from 9 to 4
move 7 from 5 to 7
move 5 from 1 to 3
move 2 from 4 to 5
move 1 from 4 to 6
move 1 from 6 to 9
move 4 from 9 to 2
move 12 from 7 to 9
move 2 from 4 to 9
move 6 from 5 to 9
move 3 from 7 to 6
move 12 from 9 to 6
move 5 from 9 to 1
move 1 from 7 to 6
move 14 from 6 to 1
move 20 from 3 to 5
move 5 from 9 to 5
move 3 from 2 to 8
move 1 from 6 to 4
move 1 from 9 to 2
move 1 from 4 to 6
move 1 from 2 to 6
move 16 from 1 to 5
move 1 from 2 to 1
move 12 from 5 to 6
move 1 from 8 to 4
move 29 from 5 to 1
move 5 from 6 to 9
move 20 from 1 to 3
move 4 from 1 to 3
move 11 from 3 to 8
move 1 from 4 to 3
move 4 from 9 to 8
move 7 from 1 to 8
move 2 from 3 to 2
move 2 from 6 to 7
move 1 from 9 to 8
move 10 from 3 to 5
move 1 from 6 to 1
move 1 from 7 to 2
move 3 from 1 to 2
move 6 from 2 to 4
move 2 from 6 to 3
move 4 from 6 to 5
move 1 from 6 to 2
move 1 from 2 to 9
move 6 from 5 to 2
move 1 from 9 to 3
move 24 from 8 to 7
move 1 from 4 to 8
move 5 from 5 to 4
move 1 from 4 to 8
move 1 from 8 to 7
move 2 from 8 to 9
move 1 from 9 to 7
move 6 from 2 to 4
move 10 from 3 to 7
move 3 from 5 to 3
move 1 from 9 to 8
move 3 from 3 to 8
move 4 from 8 to 7
move 1 from 4 to 6
move 1 from 6 to 4
move 13 from 4 to 3
move 17 from 7 to 6
move 1 from 6 to 3
move 2 from 4 to 8
move 3 from 7 to 5
move 14 from 6 to 7
move 1 from 5 to 9
move 1 from 5 to 9
move 2 from 6 to 7
move 1 from 5 to 1
move 1 from 1 to 6
move 1 from 9 to 3
move 29 from 7 to 4
move 10 from 4 to 3
move 6 from 7 to 5
move 1 from 6 to 5
move 1 from 9 to 7
move 1 from 7 to 2
move 4 from 3 to 2
move 1 from 2 to 9
move 1 from 8 to 5
move 11 from 3 to 4
move 24 from 4 to 7
move 2 from 2 to 5
move 10 from 3 to 2
move 6 from 2 to 1
move 5 from 4 to 7
move 1 from 9 to 2
move 3 from 5 to 1
move 1 from 4 to 6
move 4 from 2 to 3
move 5 from 5 to 7
move 2 from 5 to 3
move 32 from 7 to 5
move 16 from 5 to 1
move 1 from 1 to 2
move 3 from 2 to 9
move 1 from 8 to 6
move 3 from 7 to 6
move 1 from 2 to 4
move 5 from 6 to 8
move 5 from 8 to 6
move 2 from 9 to 3
move 1 from 7 to 5
move 9 from 5 to 4
move 1 from 9 to 1
move 2 from 3 to 1
move 4 from 3 to 6
move 1 from 3 to 8
move 6 from 4 to 6
move 6 from 5 to 9
move 1 from 9 to 6
move 1 from 5 to 1
move 1 from 5 to 4
move 1 from 3 to 6
move 1 from 8 to 3
move 1 from 4 to 2
move 1 from 2 to 3
move 17 from 6 to 4
move 4 from 1 to 8
move 3 from 9 to 6
move 1 from 8 to 4
move 1 from 9 to 7
move 2 from 6 to 2
move 1 from 7 to 8
move 12 from 1 to 9
move 8 from 9 to 2
move 1 from 6 to 9
move 6 from 2 to 8
move 2 from 8 to 3
move 18 from 4 to 9
move 2 from 1 to 6
move 1 from 6 to 5
move 3 from 4 to 3
move 7 from 3 to 8
move 4 from 2 to 7
move 1 from 4 to 6
move 2 from 6 to 4
move 13 from 9 to 6
move 1 from 5 to 2
move 5 from 9 to 3
move 9 from 1 to 2
move 1 from 1 to 8
move 1 from 2 to 6
move 3 from 7 to 6
move 2 from 2 to 6
move 9 from 8 to 6
move 1 from 7 to 8
move 1 from 8 to 7
move 2 from 4 to 6
move 5 from 3 to 6
move 17 from 6 to 9
move 7 from 8 to 4
move 4 from 2 to 3
move 17 from 6 to 2
move 1 from 6 to 4
move 1 from 7 to 8
move 1 from 8 to 9
move 24 from 9 to 6
move 4 from 3 to 1
move 1 from 1 to 5
move 20 from 6 to 4
move 4 from 6 to 9
move 1 from 5 to 7
move 2 from 4 to 2
move 1 from 9 to 7
move 25 from 4 to 3
move 1 from 4 to 2
move 2 from 1 to 6
move 3 from 9 to 4
move 2 from 4 to 7
move 2 from 7 to 5
move 1 from 4 to 2
move 1 from 6 to 3
move 1 from 1 to 5
move 5 from 3 to 9
move 1 from 5 to 6
move 10 from 2 to 8
move 9 from 2 to 5
move 21 from 3 to 6
move 1 from 7 to 6
move 2 from 6 to 5
move 5 from 9 to 7
move 6 from 7 to 8
move 19 from 6 to 9
move 1 from 6 to 1
move 8 from 8 to 1
move 1 from 6 to 1
move 2 from 8 to 5
move 5 from 9 to 2
move 6 from 8 to 2
move 2 from 9 to 7
move 9 from 9 to 4
move 7 from 2 to 4
move 1 from 6 to 4
move 14 from 5 to 9
move 1 from 1 to 8
move 1 from 7 to 9
move 4 from 2 to 9
move 16 from 4 to 6
move 3 from 2 to 8
move 1 from 6 to 2
move 2 from 8 to 9
move 1 from 8 to 7
move 1 from 8 to 3
move 3 from 2 to 7
move 1 from 3 to 9
move 8 from 9 to 3
move 4 from 7 to 8
move 1 from 5 to 4
move 4 from 6 to 3
move 1 from 4 to 2
move 9 from 3 to 8
move 10 from 9 to 5
move 8 from 6 to 7
move 13 from 8 to 4
move 8 from 5 to 2
move 3 from 6 to 3
move 7 from 9 to 6
move 7 from 7 to 2
move 2 from 4 to 6
move 5 from 6 to 2
move 3 from 1 to 5
move 5 from 5 to 8
move 4 from 6 to 2
move 4 from 1 to 8
move 15 from 2 to 6
move 11 from 4 to 9
move 12 from 6 to 8
move 1 from 6 to 9
move 5 from 3 to 7
move 2 from 2 to 6
move 6 from 7 to 1
move 3 from 1 to 3
move 1 from 4 to 1
move 1 from 3 to 9
move 1 from 3 to 9
move 1 from 7 to 6
move 1 from 3 to 2
move 4 from 2 to 6
move 4 from 2 to 7
move 1 from 2 to 6
move 4 from 1 to 6
move 12 from 6 to 7
move 2 from 6 to 1
move 8 from 9 to 6
move 1 from 7 to 4
move 14 from 8 to 1
move 8 from 1 to 5
move 1 from 3 to 9
move 5 from 9 to 5
move 1 from 8 to 9
move 1 from 9 to 2
move 1 from 9 to 3
move 5 from 8 to 3
move 12 from 5 to 4
move 1 from 9 to 2
move 6 from 7 to 3
move 7 from 3 to 2
move 1 from 5 to 1
move 1 from 8 to 3
move 2 from 1 to 3
move 2 from 6 to 9
move 5 from 6 to 5
move 5 from 1 to 7
move 4 from 4 to 1
move 7 from 2 to 8
move 4 from 3 to 8
move 1 from 9 to 3
move 1 from 9 to 5
move 4 from 1 to 8
move 10 from 7 to 9
move 1 from 6 to 7
move 2 from 8 to 6
move 6 from 4 to 2
move 5 from 3 to 1
move 2 from 6 to 3
move 2 from 7 to 1
move 5 from 2 to 5
move 2 from 7 to 1
move 7 from 5 to 7
move 2 from 5 to 6
move 2 from 5 to 3
move 3 from 2 to 9
move 9 from 9 to 3
move 1 from 6 to 4
move 3 from 3 to 1
move 9 from 8 to 2
move 6 from 3 to 6
move 8 from 7 to 9
move 4 from 9 to 8
move 14 from 1 to 5
move 1 from 9 to 2
move 1 from 1 to 5
move 2 from 3 to 6
move 12 from 5 to 3
move 2 from 2 to 8
move 7 from 6 to 2
move 12 from 2 to 8
move 2 from 6 to 2
move 6 from 9 to 6
move 1 from 1 to 2
move 1 from 9 to 3
move 2 from 5 to 9
move 1 from 9 to 2
move 1 from 9 to 4
move 1 from 3 to 2
move 2 from 6 to 7
move 2 from 6 to 9
move 5 from 4 to 2
move 14 from 3 to 9
move 15 from 9 to 4
move 1 from 7 to 4
move 10 from 8 to 6
move 1 from 5 to 9
move 2 from 9 to 5
move 10 from 8 to 1
move 1 from 7 to 4
move 5 from 1 to 2
move 2 from 1 to 5
move 3 from 4 to 6
move 4 from 5 to 8
move 5 from 8 to 6
move 14 from 2 to 9
move 2 from 6 to 7
move 3 from 2 to 9
move 3 from 1 to 7
move 1 from 7 to 3
move 3 from 7 to 1
move 1 from 3 to 6
move 1 from 7 to 6
move 1 from 8 to 9
move 2 from 1 to 4
move 1 from 1 to 2
move 16 from 9 to 4
move 7 from 4 to 8
move 5 from 8 to 1
move 2 from 8 to 3
move 2 from 1 to 7
move 13 from 6 to 7
move 2 from 2 to 3
move 4 from 7 to 4
move 6 from 4 to 5
move 4 from 7 to 6
move 3 from 1 to 2
move 2 from 2 to 6
move 3 from 3 to 8
move 5 from 5 to 3
move 2 from 9 to 6
move 3 from 3 to 7
move 1 from 8 to 1
move 22 from 4 to 8
move 1 from 4 to 3
move 9 from 6 to 3
move 1 from 2 to 1
move 4 from 3 to 4
move 2 from 4 to 5
move 1 from 1 to 7
move 4 from 3 to 7
move 2 from 6 to 1
move 1 from 6 to 7
move 18 from 8 to 7
move 2 from 6 to 5
move 2 from 3 to 4
move 1 from 5 to 4
move 30 from 7 to 6
move 2 from 1 to 3
move 18 from 6 to 8
move 12 from 6 to 4
move 13 from 4 to 9
move 2 from 3 to 8
move 1 from 6 to 2
move 3 from 7 to 2
move 1 from 1 to 2
move 2 from 5 to 9
move 8 from 8 to 1
move 1 from 7 to 8
move 7 from 1 to 3
move 2 from 4 to 9
move 1 from 1 to 6
move 4 from 2 to 1
move 16 from 8 to 1
move 1 from 2 to 6
move 2 from 4 to 8
move 2 from 5 to 1
move 4 from 3 to 7
move 3 from 7 to 1
move 1 from 6 to 8
move 1 from 8 to 9
move 1 from 7 to 3
move 6 from 3 to 5
move 1 from 3 to 8
move 1 from 6 to 9
move 16 from 9 to 5
move 4 from 5 to 3
move 15 from 5 to 1
move 1 from 5 to 8
move 3 from 9 to 8
move 9 from 8 to 5
move 6 from 5 to 1
move 4 from 5 to 6
move 2 from 6 to 4
move 1 from 6 to 4
move 1 from 8 to 4
move 3 from 3 to 6
move 3 from 6 to 8
move 1 from 6 to 8
move 21 from 1 to 9
move 4 from 8 to 5
move 3 from 5 to 7
move 2 from 5 to 1
move 2 from 4 to 8
move 2 from 8 to 2
move 2 from 7 to 8
move 1 from 7 to 9
move 1 from 8 to 7
move 5 from 1 to 8
move 1 from 7 to 8
move 4 from 8 to 4
move 2 from 4 to 5
move 1 from 2 to 7
move 1 from 2 to 7
move 2 from 7 to 6
move 2 from 6 to 9
move 1 from 4 to 9
move 1 from 3 to 4
move 16 from 1 to 5
move 16 from 5 to 7
move 2 from 5 to 4
move 14 from 9 to 6
move 5 from 4 to 3
move 3 from 3 to 6
move 5 from 1 to 4
move 2 from 4 to 7
move 7 from 9 to 4
move 2 from 9 to 7
move 10 from 6 to 9
move 8 from 4 to 6
move 1 from 8 to 4
move 1 from 1 to 9
move 14 from 6 to 3
move 10 from 3 to 2
move 3 from 7 to 8
move 6 from 3 to 1
move 2 from 7 to 9
move 5 from 7 to 9
move 10 from 9 to 1
move 2 from 4 to 3
move 1 from 2 to 1
move 16 from 1 to 4
move 1 from 6 to 1
move 2 from 3 to 9
move 3 from 8 to 5
move 8 from 7 to 1
move 3 from 5 to 9
move 7 from 4 to 6
move 7 from 1 to 5
move 2 from 8 to 3
move 1 from 7 to 8";
