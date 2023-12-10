use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    span: i64,
}

impl Range {
    fn new(start: i64, span: i64) -> Range {
        Range { start, span }
    }

    fn new_with_end(start: i64, end: i64) -> Range {
        Range::new(start, end - start)
    }

    fn end(&self) -> i64 {
        self.start + self.span
    }

    fn intersect(&self, other: &Range) -> Option<Range> {
        let start = std::cmp::max(self.start, other.start);
        let end = std::cmp::min(self.end(), other.end());
        if start < end {
            Some(Range::new_with_end(start, end))
        } else {
            None
        }
    }

    fn move_by(&self, delta: i64) -> Range {
        Range::new(self.start + delta, self.span)
    }

    fn subtract(&self, other: &Range) -> Vec<Range> {
        let mut segments = Vec::new();
        if self.start < other.start {
            segments.push(Range::new_with_end(
                self.start,
                std::cmp::min(self.end(), other.start),
            ));
        }
        if other.end() < self.end() {
            segments.push(Range::new_with_end(
                std::cmp::max(other.end(), self.start),
                self.end(),
            ));
        }
        segments
    }
}

fn ints(line: &str) -> Vec<i64> {
    let regex = Regex::new(r#"\d+"#).unwrap();
    regex
        .find_iter(line)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect()
}

fn solve(mut ranges: Vec<Range>, maps: Vec<Vec<(Range, i64)>>) -> i64 {
    for map in maps {
        let mut new = Vec::new();

        for (range, delta) in map {
            let mut rem = Vec::new();
            for range2 in &ranges {
                if let Some(intersection) = range.intersect(range2) {
                    new.push(intersection.move_by(delta));
                    rem.extend(range2.subtract(&intersection));
                } else {
                    rem.push(*range2);
                }
            }

            ranges = rem;
        }

        ranges.extend(new);
    }

    ranges.iter().map(|r| r.start).min().unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let mut parts = input.split("\n\n");

    let seeds = ints(parts.next().unwrap());
    let maps: Vec<Vec<(Range, i64)>> = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let numbers = ints(line);
                    (Range::new(numbers[1], numbers[2]), numbers[0] - numbers[1])
                })
                .collect()
        })
        .collect();

    let part1 = solve(
        seeds.iter().map(|&value| Range::new(value, 1)).collect(),
        maps.clone(),
    );
    println!("p1: {}", part1);

    let part2 = solve(
        seeds
            .chunks(2)
            .map(|chunk| Range::new(chunk[0], chunk[1]))
            .collect(),
        maps,
    );
    println!("p2: {}", part2);
}
