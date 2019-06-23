use clap::{App, Arg};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

use icfpc::models::*;
use icfpc::parse::read_all_inputs;
use icfpc::solve::solve_small_while;
use std::time::Duration;

fn solve<W: Write>(task: Task, f: &mut W, duration: Duration) {
    let cmds = solve_small_while(task, duration);
    write!(f, "{}", cmds).unwrap();
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
        .arg(
            Arg::with_name("duration")
                .long("duration")
                .takes_value(true)
                .help("millis to wait"),
        )
        .get_matches();

    let input_root = matches.value_of("input").expect("no input specified");
    let output_root = matches.value_of("output");
    let millis = matches
        .value_of("duration")
        .unwrap_or("300")
        .parse::<u64>()
        .unwrap();
    let duration = Duration::from_millis(millis);

    let inputs = read_all_inputs(&input_root);
    let progress_bar = ProgressBar::new(inputs.len() as u64);
    inputs.into_par_iter().for_each(|input| {
        let mut output_file: Box<dyn Write> = match output_root {
            Some(output_root) => {
                let output_path = format!("{}/{}", output_root, input.output_file_name());
                let output_file = File::create(&output_path).unwrap();
                Box::new(output_file)
            }
            None => Box::new(std::io::stdout()),
        };
        solve(input.task, &mut output_file, duration);
        progress_bar.inc(1);
    });
    progress_bar.finish();
}
