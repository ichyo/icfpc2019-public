use clap::{App, Arg};
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use indicatif::ProgressBar;

use icfpc::models::*;
use icfpc::parse::read_all_inputs;
use icfpc::solve::solve_small;


fn solve<W: Write>(task: Task, f: &mut W) {
    let cmds = solve_small(task);
    for cmd in cmds {
        write!(f, "{}", cmd).unwrap();
    }
}

fn main() {
    let matches = App::new("ICFPC 2019")
        .version("0.1.0")
        .arg(
            Arg::with_name("input")
                .long("input")
                .takes_value(true)
                .help("input root directory"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .help("output directory"),
        )
        .get_matches();

    let input_root = matches.value_of("input").expect("no input specified");
    let output_root = matches.value_of("output");

    let inputs = read_all_inputs(&input_root);
    let progress_bar = ProgressBar::new(inputs.len() as u64);
    inputs.into_par_iter().for_each(|input| {
        let mut output_file: Box<Write> = match output_root {
            Some(output_root) => {
                let output_path = format!("{}/{}", output_root, input.output_file_name());
                let output_file = File::create(&output_path).unwrap();
                Box::new(output_file)
            }
            None => Box::new(std::io::stdout()),
        };
        solve(input.task, &mut output_file);
        progress_bar.inc(1);
    });
    progress_bar.finish();
}
