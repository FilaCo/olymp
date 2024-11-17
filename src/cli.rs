use crate::cf::*;
use crate::timus::*;
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    problem: SolvedProblem,
    #[arg(short, long)]
    input: Option<PathBuf>,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

impl Cli {
    pub fn solve(&self) {
        let mut input_stream: Box<dyn Read> = if let Some(input_file_path) = &self.input {
            Box::new(File::open(input_file_path).expect("unable to open input file"))
        } else {
            Box::new(io::stdin())
        };

        let mut output_stream: Box<dyn Write> = if let Some(output_file_path) = &self.output {
            Box::new(File::create(output_file_path).expect("unable to open output file"))
        } else {
            Box::new(io::stdout())
        };

        match self.problem {
            SolvedProblem::T1001 => t1001::solve(&mut input_stream, &mut output_stream),
            SolvedProblem::T1521 => t1521::solve(&mut input_stream, &mut output_stream),

            SolvedProblem::C2031A => c2031a::solve(&mut input_stream, &mut output_stream),
            SolvedProblem::C2037A => c2037a::solve(&mut input_stream, &mut output_stream),
            SolvedProblem::C2037B => c2037b::solve(&mut input_stream, &mut output_stream),
            SolvedProblem::C2037C => c2037c::solve(&mut input_stream, &mut output_stream),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum SolvedProblem {
    // Timus Online Judge https://acm.timus.ru/
    /// https://acm.timus.ru/problem.aspx?space=1&num=1001
    T1001,
    /// https://acm.timus.ru/problem.aspx?space=1&num=1521
    T1521,

    // Codeforces https://codeforces.com/
    /// https://codeforces.com/problemset/problem/2031/A
    C2031A,
    /// https://codeforces.com/problemset/problem/2037/A
    C2037A,
    /// https://codeforces.com/problemset/problem/2037/B
    C2037B,
    /// https://codeforces.com/problemset/problem/2037/C
    C2037C,
}
