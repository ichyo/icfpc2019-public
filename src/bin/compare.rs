use clap::{App, Arg};
use icfpc::parse::read_all_inputs;
use icfpc::parse::read_commands;
use std::collections::HashMap;

use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("Compare solutions")
        .version("0.1.0")
        .arg(
            Arg::with_name("input")
                .long("input")
                .takes_value(true)
                .help("input root directory"),
        )
        .arg(
            Arg::with_name("file")
                .long("file")
                .takes_value(true)
                .help("solutions path file"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .help("output root to generate"),
        )
        .get_matches();

    let input_root = matches.value_of("input").expect("no input specified");
    let inputs = read_all_inputs(input_root);
    let output_root = matches.value_of("output").expect("no output specified");

    let path_file = matches.value_of("file").expect("no file specified");
    let mut result: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for output_root in std::fs::read_to_string(path_file).unwrap().lines() {
        for input in &inputs {
            let commands = {
                let output_path = format!("{}/{}", output_root, input.output_file_name());
                let mut output_file = File::open(&output_path).unwrap();
                let mut output_str = String::new();
                output_file.read_to_string(&mut output_str).unwrap();
                read_commands(&output_str)
            };
            result
                .entry(input.id.to_owned())
                .or_default()
                .push((commands.len(), output_root.to_owned()));
        }
    }
    for input in &inputs {
        let v = result.get_mut(&input.id).unwrap();
        v.sort();
        let best_root = &v[0].1;
        println!("{}: {} ({} {})", input.id, best_root, v[0].0, v[1].0);
        let best_path = format!("{}/{}", best_root, input.output_file_name());
        let new_path = format!("{}/{}", output_root, input.output_file_name());
        std::fs::copy(best_path, new_path).unwrap();
        let best_path = format!("{}/{}", best_root, input.buy_file_name());
        let new_path = format!("{}/{}", output_root, input.buy_file_name());
        if std::path::Path::new(&best_path).exists() {
            std::fs::copy(best_path, new_path).unwrap();
        }
    }
}