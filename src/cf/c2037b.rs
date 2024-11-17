use std::io;
use std::io::*;

pub fn solve(input: &mut impl Read, output: &mut impl Write) {
    let mut buf_input = BufReader::new(input);
    let mut buf_output = BufWriter::new(output);

    let mut buf_str = String::new();
    buf_input.read_line(&mut buf_str).expect("correct input");

    let t = buf_str.trim().parse::<i32>().expect("parse integer");
    buf_str.clear();

    for _ in 0..t {
        buf_input.read_line(&mut buf_str).expect("correct input");
        let k = buf_str.trim().parse::<i32>().expect("parse integer");
        buf_str.clear();

        buf_input.read_line(&mut buf_str).expect("correct input");
        let mut v: Vec<i32> = buf_str
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().expect("parse integer"))
            .collect();
        buf_str.clear();

        v.sort();

        let mut n = 0;
        let mut m = 0;
        for i in 1..(k - 1) {
            if (k - 2) % i != 0 {
                continue;
            }

            let res_n = v.binary_search(&i);

            if res_n.is_err() {
                continue;
            }

            if v.binary_search(&((k - 2) / i)).is_err() {
                continue;
            }

            n = i;
            m = (k - 2) / i;

            break;
        }

        writeln!(buf_output, "{} {}", n, m).expect("correct output");
    }
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}
