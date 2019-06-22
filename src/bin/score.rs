use clap::{App, Arg};
use glob::glob;
use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::str::Chars;

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

fn output_file_name(file_name: &str) -> String {
    format!("prob-{}.sol", &file_name[5..8])
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn move_with(&self, command: Command) -> Option<Point> {
        let (x, y) = (self.x, self.y);
        match command {
            Command::MoveUp => Some(Point::new(x, y + 1)),
            Command::MoveDown => match self.y {
                0 => None,
                _ => Some(Point::new(x, y - 1)),
            },
            Command::MoveRight => Some(Point::new(x + 1, y)),
            Command::MoveLeft => match self.x {
                0 => None,
                _ => Some(Point::new(x - 1, y)),
            },
            _ => Some(*self),
        }
    }
}

#[derive(Debug, Clone)]
enum BoosterType {
    NewHand,
    FastMove,
    Drill,
    Teleports,
    Unknown,
}

type Booster = (BoosterType, Point);

#[derive(Debug, Clone)]
struct Map(Vec<Point>);

enum Direction {
    Verticle,
    Horizontal,
}

impl Map {
    fn iter_lines(&self) -> Vec<(Direction, Point, Point)> {
        let mut iter = self.0.iter().cloned().cycle().peekable();
        let mut res = Vec::new();
        for _ in 0..self.0.len() {
            let cur = iter.next().unwrap();
            let next = *iter.peek().unwrap();
            if cur.x == next.x {
                res.push((Direction::Verticle, cur, next));
            } else if cur.y == next.y {
                res.push((Direction::Horizontal, cur, next));
            } else {
                unreachable!();
            }
        }
        res
    }

    fn enumerate_points(&self) -> Vec<Point> {
        let g_min_x = self.0.iter().map(|p| p.x).min().unwrap();
        let g_max_x = self.0.iter().map(|p| p.x).max().unwrap();
        let mut cross_y_map = HashMap::new();
        for (dir, p, q) in self.iter_lines() {
            if let Horizontal = dir {
                let min_x = cmp::min(p.x, q.x);
                let max_x = cmp::max(p.x, q.x);
                for x in min_x..max_x {
                    cross_y_map.entry(x).or_insert_with(|| Vec::new()).push(p.y);
                }
            }
        }
        let mut res = Vec::new();
        for x in g_min_x..g_max_x {
            let v = cross_y_map.get_mut(&x).unwrap();
            assert!(v.len() % 2 == 0);
            v.sort();
            let mut iter = v.iter();
            while let Some(lb) = iter.next() {
                let ub = iter.next().unwrap();
                for y in *lb..*ub {
                    res.push(Point::new(x, y));
                }
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
struct Input {
    map: Map,
    initial: Point,
    obstacles: Vec<Map>,
    boosters: Vec<Booster>,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Noop,
    TurnRight,
    TurnLeft,
}

fn skip(iter: &mut Peekable<Chars>, expected: char) {
    let c = iter.next().unwrap();
    assert!(c == expected);
}

fn skip_or_empty(iter: &mut Peekable<Chars>, expected: char) {
    if let Some(c) = iter.next() {
        assert!(c == expected);
    }
}

fn read_point(iter: &mut Peekable<Chars>) -> Point {
    skip(iter, '(');
    let mut x = 0;
    loop {
        let c = iter.next().unwrap();
        if c.is_digit(10) {
            x = x * 10 + (c as u8 - '0' as u8);
        } else {
            assert!(c == ',');
            break;
        }
    }
    let mut y = 0;
    loop {
        let c = iter.next().unwrap();
        if c.is_digit(10) {
            y = y * 10 + (c as u8 - '0' as u8);
        } else {
            assert!(c == ')');
            break;
        }
    }
    Point::new(x as usize, y as usize)
}

fn read_map_internal(mut iter: &mut Peekable<Chars>) -> (Map, char) {
    let mut points = Vec::new();
    points.push(read_point(&mut iter));
    loop {
        let c = iter.next().unwrap();
        if c != ',' {
            return (Map(points), c);
        }
        points.push(read_point(&mut iter));
    }
}

fn read_map(mut iter: &mut Peekable<Chars>) -> Map {
    let (m, c) = read_map_internal(&mut iter);
    assert!(c == '#');
    m
}

fn read_initial(mut iter: &mut Peekable<Chars>) -> Point {
    let p = read_point(&mut iter);
    skip(iter, '#');
    p
}

fn read_obstacles(mut iter: &mut Peekable<Chars>) -> Vec<Map> {
    let mut res = Vec::new();
    if *iter.peek().unwrap() == '#' {
        iter.next();
        return res;
    }

    loop {
        let (m, c) = read_map_internal(&mut iter);
        res.push(m);
        if c == '#' {
            break;
        }
        assert!(c == ';');
    }
    res
}

fn read_boosters(mut iter: &mut Peekable<Chars>) -> Vec<Booster> {
    let mut res = Vec::new();
    while let Some(c) = iter.next() {
        let booster_type = match c {
            'B' => BoosterType::NewHand,
            'F' => BoosterType::FastMove,
            'L' => BoosterType::Drill,
            'X' => BoosterType::Unknown,
            'R' => BoosterType::Teleports,
            _ => panic!("unknown type {}", c),
        };
        let point = read_point(&mut iter);
        res.push((booster_type, point));
        skip_or_empty(&mut iter, ';');
    }
    res
}

fn read_input(s: &str) -> Input {
    let mut iter = s.chars().peekable();
    let map = read_map(&mut iter);
    let initial = read_initial(&mut iter);
    let obstacles = read_obstacles(&mut iter);
    let boosters = read_boosters(&mut iter);
    Input {
        map,
        initial,
        obstacles,
        boosters,
    }
}

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
            "1000.0 * {:5.2} * {:4.2} = {:8.2}",
            self.log_wh(),
            self.ratio(),
            self.score()
        )
    }

    fn score(&self) -> f64 {
        1000.0 * self.log_wh() * self.ratio()
    }
}

fn score_small(input: Input, output_len: usize) -> ScoreInfo {
    let map_points = input.map.enumerate_points();

    let width = map_points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = map_points.iter().map(|p| p.y).max().unwrap() + 1;
    let mut remaining = 0;
    let mut passed = vec![vec![true; width]; height];
    let mut valid = vec![vec![false; width]; height];

    for p in &map_points {
        passed[p.y][p.x] = false;
        valid[p.y][p.x] = true;
        remaining += 1;
    }

    for o in &input.obstacles {
        for p in o.enumerate_points().iter() {
            if p.y < height && p.x < width && valid[p.y][p.x] {
                valid[p.y][p.x] = false;
                passed[p.y][p.x] = true;
                remaining -= 1;
            }
        }
    }

    ScoreInfo {
        width,
        height,
        best_estimated: remaining,
        team_time: output_len,
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
    let files = find_files(&input_root);

    let mut sum_scores = 0.0;
    for f in files.iter() {
        let input_path = format!("{}/{}", input_root, f);
        let mut input_file = File::open(&input_path).unwrap();
        let output_path = format!("{}/{}", output_root, output_file_name(&f));
        let mut output_file = File::open(&output_path).unwrap();
        let mut input_str = String::new();
        input_file.read_to_string(&mut input_str);
        let mut output_str = String::new();
        output_file.read_to_string(&mut output_str);
        let output_len = output_str.trim_end().len();
        let score_info = score_small(read_input(&input_str), output_len);
        eprintln!("{}: {}", f, score_info.debug());
        sum_scores += score_info.score();
    }
    println!("output: {}", output_root);
    println!("total_score: {}", sum_scores);
}
