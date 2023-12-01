use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("src/input") {
        let sum: u32 = lines
            .filter_map(Result::ok)
            .filter_map(|line| {
                let number: Option<u32> = match (
                    line.chars()
                        .find(|c| c.is_digit(10))
                        .and_then(|c| c.to_digit(10)),
                    line.chars()
                        .rev()
                        .find(|c| c.is_digit(10))
                        .and_then(|c| c.to_digit(10)),
                ) {
                    (Some(first), Some(last)) => Some(first * 10 + last),
                    _ => None,
                };
                number
            })
            .sum();

        println!("Sum of numbers: {}", sum);
    }
}
