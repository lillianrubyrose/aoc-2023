use std::collections::HashMap;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn count_steps<'a>(
    mut start: &'a str,
    end_predicate: impl Fn(&'a str) -> bool,
    network: &HashMap<&'a str, Vec<&'a str>>,
    instructions: &[char],
) -> usize {
    instructions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(i, &move_dir)| {
            if end_predicate(start) {
                Some(i)
            } else {
                start = network[&start][(move_dir == 'R') as usize];
                None
            }
        })
        .unwrap_or_else(|| unreachable!())
}

fn main() {
    let contents = include_str!("input.txt");
    let mut lines = contents.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let network = lines
        .skip(1)
        .map(|line| {
            let parts = line.split(" = ").collect::<Vec<_>>();
            (
                parts[0],
                parts[1][1..parts[1].len() - 1].split(", ").collect(),
            )
        })
        .collect::<HashMap<_, _>>();

    println!(
        "p1: {}",
        count_steps("AAA", |pos| pos == "ZZZ", &network, &instructions),
    );

    println!(
        "p2: {}",
        network
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| count_steps(k, |pos| pos.ends_with('Z'), &network, &instructions))
            .fold(1, lcm),
    );
}
