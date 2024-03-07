#![allow(dead_code)]

use serde::Serialize;
use std::{collections::VecDeque, num::ParseIntError};

fn read_input(input: Option<&str>) -> String {
    let output = match input {
        None => include_str!("../input.txt"),
        Some(x) => x,
    };
    output.to_string()
}

fn get_santa_computer_secret(value: u32) -> String {
    format!("{value:X}")
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct SantaComputer {
    data: String,
    checksum: String,
}

#[allow(clippy::manual_unwrap_or)]
fn part1(input: String) -> u32 {
    input
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| match x.parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => 0,
                })
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

#[allow(clippy::manual_unwrap_or)]
fn part2(input: String) -> u32 {
    let mut calories_per_elf = input
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| match x.parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => 0,
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    calories_per_elf.sort();

    calories_per_elf.iter().rev().take(3).sum()
}

#[allow(clippy::manual_unwrap_or)]
fn part6(input: String) -> u32 {
    let mut calories_per_elf = input
        .trim()
        .split("\n\n")
        .filter_map(|x| {
            match x
                .split('\n')
                .map(|x| x.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()
            {
                Ok(s) => Some(s.iter().sum()),
                Err(_) => None,
            }
        })
        .collect::<Vec<u32>>();

    calories_per_elf.sort();

    calories_per_elf.iter().rev().take(3).sum()
}

fn checksum(value: &str) -> String {
    let checkum = &value[2..];
    checkum.to_string()
}

fn checksum2(value: &str) -> &str {
    let checkum = &value[2..];
    checkum
}

fn data(value: &str) -> String {
    let mut data: VecDeque<char> = value.chars().collect();
    data.push_front('*');
    data.push_back('*');

    let data: String = data.iter().collect();

    data
}

fn part4(input: String) -> SantaComputer {
    let part1 = part1(input);
    let secret = get_santa_computer_secret(part1);
    let data = data(&secret);
    let checksum = checksum(&secret);

    SantaComputer { data, checksum }
}

fn part5(input: String) -> String {
    let part4 = part4(input);
    serde_json::to_string(&part4).unwrap()
}

fn part8(input: String) -> u32 {
    todo!("Implement part 8");
}

fn main() {
    let input = read_input(None);

    let answer = part1(input);

    println!("Answer: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_part1_sample() {
        let input = read_input(Some(indoc!(
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "
        )));
        dbg!(&input);
        let answer = part1(input);
        assert_eq!(answer, 24000);
    }

    #[test]
    fn test_part1_sample_drunk_elf() {
        let input = read_input(Some(indoc!(
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            Bleurg!!
            9000

            10000
            "
        )));
        dbg!(&input);
        let answer = part1(input);
        assert_eq!(answer, 16000);
    }

    #[test]
    fn test_part1() {
        let input = read_input(None);
        dbg!(&input);
        let answer = part1(input);
        assert_eq!(answer, 69693);
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input(Some(indoc!(
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "
        )));
        dbg!(&input);
        let answer = part2(input);
        assert_eq!(answer, 45000);
    }

    #[test]
    fn test_part2() {
        let input = read_input(None);
        dbg!(&input);
        let answer = part2(input);
        assert_eq!(answer, 200945);
    }

    // #[ignore]
    // #[test]
    // fn test_data() {
    //     let value = String::from("A0C0");
    //     assert_eq!(data(value), String::from("*A0C0*"));
    // }
    //
    // #[ignore]
    // #[test]
    // fn test_checksum() {
    //     let value = String::from("A0C0");
    //     assert_eq!(checksum(value), String::from("C0"));
    // }
    fn test_checksum2() {
        let value = String::from("A0C0");
        // assert_eq!(checksum2(value), "C0");
        // drop(value);
        assert_eq!(checksum2(&value), "C0");
    }

    #[test]
    fn test_data() {
        let value = String::from("A0C0");
        assert_eq!(data(&value), String::from("*A0C0*"));
    }

    #[test]
    fn test_checksum() {
        let value = String::from("A0C0");
        assert_eq!(checksum(&value), String::from("C0"));
    }

    #[test]
    fn test_part4_sample() {
        let input = read_input(Some(indoc!(
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "
        )));
        dbg!(&input);
        let answer = part4(input);
        assert_eq!(
            answer,
            SantaComputer {
                data: String::from("*5DC0*"),
                checksum: String::from("C0")
            }
        );
    }

    #[test]
    fn test_part5_sample() {
        let input = read_input(Some(indoc!(
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "
        )));
        dbg!(&input);
        let answer = part5(input);
        assert_eq!(answer, String::from(r#"{"data":"*5DC0*","checksum":"C0"}"#));
    }

    #[test]
    fn test_part6_sample() {
        let input = read_input(Some(indoc!(
            // 11000 + 10000 + 3000+2000+1000 = 27000
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            bidule
            9000

            10000
            "
        )));
        dbg!(&input);
        let answer = part6(input);
        assert_eq!(answer, 27000);
    }

    #[ignore]
    #[test]
    fn test_part8_sample() {
        let input = read_input(Some(indoc!(
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "
        )));
        dbg!(&input);
        let answer = part8(input);
        assert_eq!(answer, 45000);
    }
}
