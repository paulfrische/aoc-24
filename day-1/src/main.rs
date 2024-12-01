use std::collections::{BinaryHeap, HashMap};

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::{complete::digit1, streaming::space1},
    multi::{many0, many1},
};

struct SortedHeapIterator<T> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> Iterator for SortedHeapIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

trait IntoIteratorSorted<T> {
    fn into_sorted_iter(self) -> SortedHeapIterator<T>;
}

impl<T> IntoIteratorSorted<T> for BinaryHeap<T> {
    fn into_sorted_iter(self) -> SortedHeapIterator<T> {
        SortedHeapIterator { heap: self }
    }
}

fn main() -> anyhow::Result<()> {
    println!("part1: {}", part1(include_str!("input.txt"))?);
    println!("part2: {}", part2(include_str!("input.txt"))?);

    Ok(())
}

fn parse_collumn(input: &str) -> nom::IResult<&str, (u64, u64)> {
    let (input, _) = many0(tag("\n"))(input)?;
    let (input, n1) = many1(digit1)(input)?;
    let (input, _) = many1(space1)(input)?;
    let (input, n2) = many1(digit1)(input)?;

    Ok((
        input,
        (n1.join("").parse().unwrap(), n2.join("").parse().unwrap()),
    ))
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    let mut input = input;
    loop {
        let (rest, (na, nb)) = parse_collumn(input).map_err(|e| anyhow!("{}", e))?;
        input = rest;

        left.push(na);
        right.push(nb);

        if input.is_empty() || input == "\n" {
            break;
        }
    }

    Ok(left
        .into_sorted_iter()
        .zip(right.into_sorted_iter())
        .map(|(a, b)| a.abs_diff(b))
        .fold(0, |a, b| a + b))
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut numbers = vec![];
    let mut number_frequencies = HashMap::new();

    let mut input = input;
    loop {
        let (rest, (na, nb)) = parse_collumn(input).map_err(|e| anyhow!("{}", e))?;
        input = rest;

        numbers.push(na);

        if !number_frequencies.contains_key(&nb) {
            number_frequencies.insert(nb, 1);
        } else {
            number_frequencies.insert(nb, number_frequencies.get(&nb).unwrap() + 1);
        }

        if input.is_empty() || input == "\n" {
            break;
        }
    }

    Ok(numbers
        .iter()
        .map(|n| *n * number_frequencies.get(n).unwrap_or(&0))
        .fold(0, |a, e| a + e))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let example = r"3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part1(example)?, 11);

        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let example = r"3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part2(example)?, 31);

        Ok(())
    }
}
