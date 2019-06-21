use clap::{App, Arg};
use glob::glob;
use std::fs::File;
use std::io::Write;

fn find_files(input_root: &str) -> Vec<String> {
    glob(&format!("{}/prob-*.desc", input_root))
        .expect("glob pattern")
        .map(|p| {
            p.unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
}

fn output_file(file_name: &str) -> String {
    format!("prob-{}.sol", &file_name[5..8])
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
    let output_root = matches.value_of("output").expect("no output specified");

    let files = find_files(&input_root);
    for f in files {
        let input_path = format!("{}/{}", input_root, f);
        let output_path = format!("{}/{}", output_root, output_file(&f));
        let mut file = File::create(output_path).unwrap();
        writeln!(file, "W");
    }
}
