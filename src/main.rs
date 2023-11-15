use std::fs;

fn read_exercise_input(exercise: u32) -> String {
    let filename = format!("data/exercise_{:02}.txt", exercise);
    fs::read_to_string(&filename)
        .expect(&format!("Unable to read exercise input {}", filename))
        .trim_end()
        .to_string()
}

fn main() {
    let input = read_exercise_input(1);
    println!("ex01a_purist: {}", ex01a_purist(&input));
    println!("ex01a_readable: {}", ex01a_readable(&input));
    println!("ex01b: {}", ex01b(&input));

    let input = read_exercise_input(2);
    println!("ex02a: {}", ex02a(&input));
    println!("ex02b: {}", ex02b(&input));

    let input = read_exercise_input(3);
    println!("ex03a: {}", ex03a(&input));
    println!("ex03b: {}", ex03b(&input));

    // println!("ex04a: {}", ex04a("iwrupvqb"));
    // println!("ex04b: {}", ex04b("iwrupvqb"));

    let input = read_exercise_input(5);
    println!("ex05a: {}", ex05a(&input));
    println!("ex05b: {}", ex05b(&input));

}

fn ex01a_purist(input: &str) -> i32 {
    input.chars().fold(0, |acc, code| acc + ex01_decoder(code))
}

fn ex01a_readable(input: &str) -> i32 {
    let mut floor = 0;
    for code in input.chars() {
        floor += ex01_decoder(code);
    }
    floor
}

fn ex01_decoder(code: char) -> i32 {
    match code {
        '(' => 1,
        ')' => -1,
        _ => unreachable!("Invalid input {code}"),
    }
}

fn ex01b(input: &str) -> usize {
    const TARGET: i32 = -1;
    let mut floor = 0;
    for (index, code) in input.chars().enumerate() {
        floor += ex01_decoder(code);
        if floor == TARGET {
            return index + 1;
        }
    }
    panic!("Not doable!");
}

fn ex02_parse_line(line: &str) -> [u32; 3] {
    let dimensions: Vec<u32> = line.splitn(3, 'x')
        .map(|d| d.parse().expect("Bad input"))
        .collect();
    [dimensions[0], dimensions[1], dimensions[2]]
}

fn ex02a(input: &str) -> u32 {
    fn wrapping_paper(line: &str) -> u32 {
        let [w, h, l] = ex02_parse_line(line);
        let mut areas = [w * h, w * l, h * l];
        areas.sort();
        areas[0] * 3 + areas[1] * 2 + areas[2] * 2
    }
    input.lines().map(|line| wrapping_paper(line)).sum()
}

fn ex02b(input: &str) -> u32 {
    fn ribbon(line: &str) -> u32 {
        let [w, h, l] = ex02_parse_line(line);
        let mut half_perimeters = [w + h, w + l, h + l];
        half_perimeters.sort();
        2 * half_perimeters[0] + w * h * l
    }
    input.lines().map(|line| ribbon(line)).sum()
}

fn ex03a(input: &str) -> u32 {
    _ex03(input, 1)
}

fn ex03b(input: &str) -> u32 {
    _ex03(input, 2)
}

fn _ex03(input: &str, santas: usize) -> u32 {
    let mut houses = vec![vec![true]];
    let mut x = 1;
    let mut y = 1;
    let mut ox = 1;
    let mut oy = 1;
    let mut visited = 1;

    for santa in 0..santas {
        for code in input.chars().skip(santa).step_by(santas) {
            match code {
                '^' => y += 1,
                'v' => y -= 1,
                '<' => x -= 1,
                '>' => x += 1,
                _ => unreachable!("Invalid code {code}"),
            };
            if y == 0 { y = 1; oy += 1; houses.insert(0, vec![]); }
            if x == 0 { x = 1; ox += 1; for row in &mut houses { row.insert(0, false); } }

            if y > houses.len() { houses.push(vec![]); }
            let row = &mut houses[y - 1];
            if x > row.len() { row.resize(x, false); }
            if !row[x - 1] { visited += 1; }
            row[x - 1] = true;
        }
        x = ox; y = oy;
    }
    visited
}

// fn ex04a(input: &str) -> u32 {
//     let mut n = 0;
//     loop {
//         let input = format!("{input}{n}");
//         let hash = md5::compute(input.as_bytes());
//         if hash[0] == 0 && hash[1] == 0 && hash[2] < 0x10 { break n; }
//         n += 1;
//     }
// }
//
// fn ex04b(input: &str) -> u32 {
//     let mut n = 0;
//     loop {
//         let input = format!("{input}{n}");
//         let hash = md5::compute(input.as_bytes());
//         if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 { break n; }
//         n += 1;
//     }
// }
//

const BAD_WORDS: [&str; 4] = ["ab", "cd", "pq", "xy"];
fn ex05a(input: &str) -> u32 {
    fn is_nice(input: &&str) -> bool {
        !BAD_WORDS.iter().any(|word| input.contains(word)) &&
            std::iter::zip(input.chars(), input.chars().skip(1)).any(|(a, b)| a == b) &&
            input.chars().filter(|ch| "aeiou".contains(*ch)).collect::<Vec<_>>().len() >= 3
    }
    input.lines().filter(is_nice).count() as u32
}

fn ex05b(input: &str) -> u32 {
    fn is_nice(line: &&str) -> bool {
        c1(line) && c2(line)
    }

    fn c1(line: &&str) -> bool {
        match line.char_indices().rev().nth(2) {
            None => return false,
            Some((last_index, _)) => {
                for (start_index, _) in line[0..last_index].char_indices() {
                    let end_index = start_index + line[start_index..].char_indices().nth(2).unwrap().0;
                    if line[end_index..].contains(&line[start_index..end_index]) { return true; }
                }
            }
        }
        false
    }

    fn c2(line: &&str) -> bool {
        match line.char_indices().rev().nth(1) {
            None => return false,
            Some((last_index, _)) => {
                for (start_index, ch) in line[0..last_index].char_indices() {
                    if line[start_index..].chars().nth(2).unwrap() == ch { return true; }
                }
            }
        }
        false
    }

    input.lines().filter(is_nice).count() as u32
}
