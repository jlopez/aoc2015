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
