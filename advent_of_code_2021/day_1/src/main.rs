use std::{fs::File, io::Read};

fn main() {
    let measurements = read_input();
    let increases = calculate_increases(0, measurements, 0);
    println!("{}", increases) // Answer is 1529
}

fn read_input() -> Vec<i32> {
    let mut f = match File::open("input.txt") {
        Ok(f) => f,
        Err(_) => File::create("input.txt").unwrap(),
    };

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("An error ocurred while reading the file content");

    let mut input: Vec<i32> = Vec::new();

    for line in content.lines() {
        match line.trim().parse() {
            Ok(n) => input.push(n),
            Err(_) => continue,
        }
    }

    return input;
}

fn calculate_increases(i: usize, m: Vec<i32>, mut sum: i32) -> i32 {
    if (i + 1) == m.len() || i == m.len() {
        return sum;
    }

    if m[i] < m[i + 1] {
        sum += 1;
    }

    return calculate_increases(i + 1, m, sum);
}
