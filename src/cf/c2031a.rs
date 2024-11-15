use std::cmp::{max, min};
use std::io;
use std::io::*;

pub fn solve(input: &mut impl Read, output: &mut impl Write) {
    let mut buf_input = BufReader::new(input);
    let mut buf_output = BufWriter::new(output);

    let mut buf_str = String::new();
    buf_input.read_line(&mut buf_str).expect("correct input");

    let t = buf_str.trim().parse::<i32>().expect("parse integer");
    buf_str.clear();

    for i in 0..t {
        buf_input.read_line(&mut buf_str).expect("correct input");
        let n = buf_str.trim().parse::<usize>().expect("parse integer");
        buf_str.clear();

        buf_input.read_line(&mut buf_str).expect("correct input");
        let columns: Vec<i32> = buf_str
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().expect("parse integer"))
            .collect();
        buf_str.clear();

        let mut d = vec![0; n];

        for i in 0..n {
            d[i] = 1;
            for j in 0..i {
                if columns[j] <= columns[i] {
                    d[i] = max(d[i], 1 + d[j]);
                }
            }
        }

        writeln!(buf_output, "{}", n - d.iter().max().unwrap()).expect("output");
    }
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}
