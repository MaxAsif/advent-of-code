#![feature(array_chunks)]

fn get_priority(ch: char) -> Option<u32> {
    if ch >= 'a' && ch <= 'z' {
        Some(ch as u32 - 'a' as u32 + 1)
    } else if ch >= 'A' && ch  <= 'Z' {
        Some(ch as u32 - 'A' as u32 + 27)
    } else {
        None
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input.lines()
        .map(|rucksack| {
            let middle = rucksack.len() / 2;
            let first_compartment = &rucksack[..middle];
            let second_compartment = &rucksack[middle..];

            let common = first_compartment
                .chars()
                .find(|c| second_compartment.contains(*c)).unwrap();

            get_priority(common).unwrap()
        }).sum();

    result.to_string()
}


pub fn process_part2(input: &str) -> String {
    // let sacks = input.lines().collect::<Vec<&str>>();
    // let len = sacks.len();
    // let mut i = 0;
    // let mut result = 0;
    // while i < len {
    //     let common = sacks[i].chars().find(|c| sacks[i+1].contains(*c) && sacks[i+2].contains(*c)).unwrap();
    //     i += 3;
    //     result += get_priority(common);
    // }
    // result.to_string()

    let result = input
        .lines()
        .array_chunks::<3>()
        .map(|[a,b,c]| {
            let common = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            dbg!(a,b,c);
            get_priority(common).unwrap()
        })
        .sum();

    result.to_string()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part_2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
