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

        let mut d = vec![0u64; n + 1];

        buf_input.read_line(&mut buf_str).expect("correct input");
        buf_str
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().expect("parse integer"))
            .for_each(|num| d[num] += 1);
        buf_str.clear();

        let result = d.iter().fold(0u64, |acc, e| acc + (e / 2));

        writeln!(buf_output, "{}", result).expect("output");
    }
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}
