use std::{str::FromStr, cmp::Ordering};


struct Interval {
    start: u32,
    end: u32
}

impl Interval {
    fn complete_overlap(&self, other: &Self) -> bool {
        if self.start >= other.start && self.end <= other.end {
            true
        } else if other.start >= self.start && other.end <= self.end {
            true
        } else {
            false
        }
        
    }

    fn partial_overlap(&self, other: &Self) -> bool {
        let mut interval_vec = vec![self, other];
        interval_vec.sort_by_key(|interv| interv.start);
        match interval_vec[1].start.cmp(&interval_vec[0].end) {
            Ordering::Greater => false,
            _ => true
        }
    }
}

impl FromStr for Interval {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr = s
            .split("-")
            .map(|ch| {
                ch
                .parse::<u32>()
                .unwrap()
            })
            .collect::<Vec<u32>>();
        Ok(Interval { start: arr[0], end: arr[1]})
    }
}

pub fn process_part1(input: &str) -> String {
    let results: u32 = input
        .lines()
        .map(|line| {
            let intervals_vec = line.split(",").map(|range| {
                range.parse::<Interval>().unwrap()
            })
            .collect::<Vec<Interval>>();
            match intervals_vec[0].complete_overlap(&intervals_vec[1]) {
                true => 1,
                false => 0
            }
        }).sum();
    results.to_string()
}


pub fn process_part2(input: &str) -> String {
    let results: u32 = input
        .lines()
        .map(|line| {
            let intervals_vec = line.split(",").map(|range| {
                range.parse::<Interval>().unwrap()
            })
            .collect::<Vec<Interval>>();
            match intervals_vec[0].partial_overlap(&intervals_vec[1]) {
                true => 1,
                false => 0
            }
        }).sum();
    results.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    // #[ignore = "reason"]
    fn part_2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
