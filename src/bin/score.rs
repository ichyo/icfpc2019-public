use clap::{App, Arg};
use icfpc::models::*;
use icfpc::parse::read_all_inputs;
use icfpc::parse::read_commands;
use icfpc::utils::Matrix;
use std::collections::HashMap;

use std::fs::File;
use std::io::Read;

struct ScoreInfo {
    width: usize,
    height: usize,
    best_estimated: usize,
    team_time: usize,
}

impl ScoreInfo {
    fn log_wh(&self) -> f64 {
        let wh = self.width as f64 * self.height as f64;
        wh.log2()
    }

    fn ratio(&self) -> f64 {
        self.best_estimated as f64 / self.team_time as f64
    }

    fn debug(&self) -> String {
        format!(
            "1000.0 * {:5.2} * {:4.2} = {:8.2} ({:6} steps) ({:3} x {:3} = {:6})",
            self.log_wh(),
            self.ratio(),
            self.score(),
            self.team_time,
            self.width,
            self.height,
            self.width * self.height
        )
    }

    fn score(&self) -> f64 {
        1000.0 * self.log_wh() * self.ratio()
    }
}

fn score_small(task: Task, commands: Commands) -> ScoreInfo {
    let map_points = task.map.enumerate_points();

    let width = map_points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = map_points.iter().map(|p| p.y).max().unwrap() + 1;
    let mut remaining = 0;
    let mut passed = Matrix::new(width as usize, height as usize, true);
    let mut valid = Matrix::new(width as usize, height as usize, false);

    for &p in &map_points {
        passed.set(p, false);
        valid.set(p, true);
        remaining += 1;
    }

    for o in &task.obstacles {
        for &p in o.enumerate_points().iter() {
            if let Some(true) = valid.get(p) {
                valid.set(p, false);
                passed.set(p, true);
                remaining -= 1;
            }
        }
    }

    ScoreInfo {
        width: width as usize,
        height: height as usize,
        best_estimated: remaining * 24 / 100,
        team_time: commands.len(),
    }
}

fn main() {
    let matches = App::new("Score checker")
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
    let inputs = read_all_inputs(input_root);

    let mut sum_scores = 0.0;
    for input in inputs {
        let commands = {
            let output_path = format!("{}/{}", output_root, input.output_file_name());
            let mut output_file = File::open(&output_path).unwrap();
            let mut output_str = String::new();
            output_file.read_to_string(&mut output_str).unwrap();
            read_commands(&output_str)
        };
        let mut counter = HashMap::new();
        for b in &input.task.boosters {
            *counter.entry(b.kind.clone()).or_insert(0) += 1;
        }
        let count_info = format!(
            "B:{} F:{} L:{} X:{} R:{} C:{}",
            counter.get(&BoosterType::NewHand).unwrap_or(&0),
            counter.get(&BoosterType::FastMove).unwrap_or(&0),
            counter.get(&BoosterType::Drill).unwrap_or(&0),
            counter.get(&BoosterType::Spawn).unwrap_or(&0),
            counter.get(&BoosterType::Teleports).unwrap_or(&0),
            counter.get(&BoosterType::Cloning).unwrap_or(&0),
        );
        let score_info = score_small(input.task, commands);
        eprintln!("{}: {} ({})", input.id, score_info.debug(), count_info);
        sum_scores += score_info.score();
    }
    println!("output: {}", output_root);
    println!("total_score: {}", sum_scores);
}
