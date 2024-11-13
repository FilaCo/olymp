use std::io;
use std::io::{BufRead, BufReader, Read, Write};

pub fn solve(input: &mut impl Read, output: &mut impl Write) {
    let mut buf_input = BufReader::new(input);

    let result = buf_input
        .lines()
        .map(|t| t.expect("correct input"))
        .map(|line| {
            line.split(' ')
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .map(|t| t.parse::<i64>().expect("correct integer"))
                .map(|t| t as f64)
                .map(|t| t.sqrt())
                .collect::<Vec<f64>>()
        })
        .reduce(|mut acc, e| {
            e.iter().for_each(|t| acc.push(*t));

            acc
        })
        .expect("no numbers");

    result
        .iter()
        .rev()
        .for_each(|item| writeln!(output, "{:.*}", 4, item).expect("valid output"));
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}
