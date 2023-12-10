use std::collections::{HashMap, HashSet};

fn neighbors(lines: &Vec<&str>, x: isize, y: isize) -> Vec<(isize, isize, char)> {
    let mut neighbors = Vec::new();

    for x2 in [-1, 0, 1].iter() {
        for y2 in [-1, 0, 1].iter() {
            if *x2 == 0 && *y2 == 0 {
                continue;
            }

            let x3 = x + x2;
            let y3 = y + y2;

            if x3 >= 0 && y3 >= 0 && (x3 as usize) < lines[0].len() && (y3 as usize) < lines.len() {
                neighbors.push((x3, y3, lines[y3 as usize].chars().nth(x3 as usize).unwrap()));
            }
        }
    }

    neighbors
}

fn main() {
    let contents = include_str!("input.txt");
    let lines: Vec<&str> = contents.lines().collect();

    let mut likely_gears: HashMap<(isize, isize), Vec<i32>> = HashMap::new();
    let mut part1 = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            if line.as_bytes()[x].is_ascii_digit() {
                let mut num = String::new();
                let mut gears = HashSet::new();
                let mut is_part_number = false;

                while x < line.len() && line.as_bytes()[x].is_ascii_digit() {
                    num.push(line.chars().nth(x).unwrap());

                    for (x2, x3, c) in neighbors(&lines, x as isize, y as isize) {
                        if c == '*' {
                            gears.insert((x2, x3));
                        }

                        if c != '.' && !c.is_ascii_digit() {
                            is_part_number = true;
                        }
                    }

                    x += 1;
                }

                let num = num.parse::<i32>().unwrap();
                if is_part_number {
                    part1 += num;
                }

                for gear in gears {
                    likely_gears.entry(gear).or_default().push(num);
                }
            } else {
                x += 1;
            }
        }
    }

    println!("p1: {}", part1);
    println!(
        "p2: {}",
        likely_gears
            .values()
            .filter(|g| g.len() == 2)
            .map(|g| g.iter().product::<i32>())
            .sum::<i32>()
    );
}
