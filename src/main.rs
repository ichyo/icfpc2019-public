use clap::{App, Arg};
use glob::glob;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::sink;
use std::io::{Read, Write};
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
    fn revert_with(&self, command: Command) -> Option<Point> {
        let (x, y) = (self.x, self.y);
        match command {
            Command::MoveDown => Some(Point::new(x, y + 1)),
            Command::MoveUp => match self.y {
                0 => None,
                _ => Some(Point::new(x, y - 1)),
            },
            Command::MoveLeft => Some(Point::new(x + 1, y)),
            Command::MoveRight => match self.x {
                0 => None,
                _ => Some(Point::new(x - 1, y)),
            },
            _ => unreachable!(),
        }
    }
}

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
enum BoosterType {
    NewHand,
    FastMove,
    Drill,
    Unknown,
}

type Booster = (BoosterType, Point);

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

fn solve_small(input: Input) -> Vec<Command> {
    let mut rng = thread_rng();
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

    let mut moves = [
        Command::MoveUp,
        Command::MoveDown,
        Command::MoveLeft,
        Command::MoveRight,
    ];
    let mut res = Vec::new();
    let mut cp = input.initial;
    if !passed[cp.y][cp.x] {
        passed[cp.y][cp.x] = true;
        remaining -= 1;
    }
    let mut start = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let connect_to_invalid = moves
                .iter()
                .filter_map(|m| Point::new(x, y).move_with(*m))
                .filter(|p| p.x < width && p.y < height)
                .any(|p| !valid[p.y][p.x]);

            if valid[y][x] && connect_to_invalid {
                start.push(Point::new(x, y));
            }
        }
    }
    let start = start.choose(&mut rng).unwrap();
    {
        let mut data = vec![vec![None; width]; height];
        let mut queue = VecDeque::new();
        queue.push_back(cp);
        data[cp.y][cp.x] = Some(Command::Noop);
        while let Some(c) = queue.pop_front() {
            if c == *start {
                passed[c.y][c.x] = true;
                remaining -= 1;

                let mut local_cmds = Vec::new();
                let mut iter = c;
                while iter != cp {
                    let cmd = data[iter.y][iter.x].unwrap();
                    local_cmds.push(cmd);
                    iter = iter.revert_with(cmd).unwrap();
                }
                local_cmds.reverse();
                res.extend(local_cmds);

                cp = c;
                break;
            }
            moves.shuffle(&mut rng);
            for m in &moves {
                if let Some(nc) = c.move_with(*m) {
                    if nc.x < width
                        && nc.y < height
                        && data[nc.y][nc.x].is_none()
                        && valid[nc.y][nc.x]
                    {
                        data[nc.y][nc.x] = Some(*m);
                        queue.push_back(nc);
                    }
                }
            }
        }
    }
    while remaining > 0 {
        let mut data = vec![vec![None; width]; height];
        let mut queue = VecDeque::new();
        queue.push_back(cp);
        data[cp.y][cp.x] = Some(Command::Noop);
        while let Some(c) = queue.pop_front() {
            if !passed[c.y][c.x] {
                passed[c.y][c.x] = true;
                remaining -= 1;

                let mut local_cmds = Vec::new();
                let mut iter = c;
                while iter != cp {
                    let cmd = data[iter.y][iter.x].unwrap();
                    local_cmds.push(cmd);
                    iter = iter.revert_with(cmd).unwrap();
                }
                local_cmds.reverse();
                res.extend(local_cmds);

                cp = c;
                break;
            }
            let mut rng = thread_rng();
            moves.shuffle(&mut rng);
            for m in &moves {
                if let Some(nc) = c.move_with(*m) {
                    if nc.x < width
                        && nc.y < height
                        && data[nc.y][nc.x].is_none()
                        && valid[nc.y][nc.x]
                    {
                        data[nc.y][nc.x] = Some(*m);
                        queue.push_back(nc);
                    }
                }
            }
        }
    }

    res
}

fn solve<R: Read, W: Write>(input_f: &mut R, f: &mut W) {
    let mut input = String::new();
    input_f.read_to_string(&mut input).unwrap();
    let input = input.trim_end();
    let input = read_input(&input);
    let cmds = (0..100)
        .map(|_| solve_small(input.clone()))
        .min_by_key(|cmds| cmds.len())
        .unwrap();
    for cmd in cmds {
        let c = match cmd {
            Command::MoveUp => 'W',
            Command::MoveDown => 'S',
            Command::MoveLeft => 'A',
            Command::MoveRight => 'D',
            _ => unreachable!(),
        };
        write!(f, "{}", c);
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
        .arg(
            Arg::with_name("number")
                .long("number")
                .short("n")
                .takes_value(true)
                .help("number of test cases to solve"),
        )
        .get_matches();

    let input_root = matches.value_of("input").expect("no input specified");
    let output_root = matches.value_of("output");
    let number = match matches.value_of("number") {
        Some(s) => match s.parse::<u32>() {
            Ok(n) => n,
            _ => u32::max_value(),
        },
        _ => u32::max_value(),
    };

    let files = find_files(&input_root);
    files.into_par_iter().take(number as usize).for_each(|f| {
        let input_path = format!("{}/{}", input_root, f);
        let mut input_file = File::open(&input_path).unwrap();
        match output_root {
            Some(output_root) => {
                let output_path = format!("{}/{}", output_root, output_file_name(&f));
                let mut output_file = File::create(&output_path).unwrap();
                solve(&mut input_file, &mut output_file);
            }
            None => {
                solve(&mut input_file, &mut std::io::stdout());
            }
        };
    });
}
