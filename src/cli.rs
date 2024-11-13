use crate::timus::*;
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
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
            Box::new(BufWriter::new(
                File::create(output_file_path).expect("unable to open output file"),
            ))
        } else {
            Box::new(io::stdout())
        };

        match self.problem {
            SolvedProblem::T1001 => t1001::solve(&mut input_stream, &mut output_stream),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum SolvedProblem {
    // Timus Online Judge https://acm.timus.ru/

    // https://acm.timus.ru/problem.aspx?space=1&num=1001
    T1001,
}
