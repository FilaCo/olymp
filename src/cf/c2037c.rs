use std::collections::VecDeque;
use std::io;
use std::io::*;

pub fn solve(input: &mut impl Read, output: &mut impl Write) {
    let mut buf_input = BufReader::new(input);
    let mut buf_output = BufWriter::new(output);

    let mut buf_str = String::new();
    buf_input.read_line(&mut buf_str).expect("correct input");

    let t = buf_str.trim().parse::<i32>().expect("parse integer");
    buf_str.clear();

    let prime = sieve(4e5 as u64 + 1);
    for _ in 0..t {
        buf_input.read_line(&mut buf_str).expect("correct input");
        let n = buf_str.trim().parse::<i32>().expect("parse integer");
        buf_str.clear();

        if n < 5 {
            writeln!(buf_output, "-1").expect("correct output");

            continue;
        }

        let mut p: VecDeque<i32> = VecDeque::new();
        p.append(&mut (1..=n).step_by(2).collect());

        let mut rest = VecDeque::new();
        let mut mid = -1;
        for i in (2..=n).step_by(2) {
            if prime[(p.back().unwrap() + i) as usize] {
                rest.push_back(i);
            } else if mid == -1 {
                mid = i;
            } else {
                rest.push_back(i);
            }
        }

        if mid == -1 {
            writeln!(buf_output, "-1").expect("correct output");

            continue;
        }

        p.push_back(mid);
        p.append(&mut rest);

        p.iter()
            .for_each(|e| write!(buf_output, "{} ", e).expect("correct output"));
        writeln!(buf_output).expect("correct output");
    }
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

fn sieve(n: u64) -> Vec<bool> {
    let mut prime = vec![true; (n + 1) as usize];
    prime[0] = true;
    prime[1] = true;

    let sqrt_n = ((n + 1) as f32).sqrt() as u64;
    for i in 2..=sqrt_n {
        if !prime[i as usize] {
            continue;
        }

        let sqr_i = i * i;
        if sqr_i <= n {
            for j in (sqr_i..=n).step_by(i as usize) {
                prime[j as usize] = false;
            }
        }
    }

    prime
}
