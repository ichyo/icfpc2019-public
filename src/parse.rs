use crate::models::*;
use glob::glob;
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;
use std::str::Chars;

pub struct Input {
    pub id: String,
    pub task: Task,
}

impl Input {
    fn new(input_file: &str, task: Task) -> Input {
        let id = &input_file[5..8];
        Input {
            id: id.to_owned(),
            task,
        }
    }

    pub fn output_file_name(&self) -> String {
        format!("prob-{}.sol", self.id)
    }
}

pub fn read_all_inputs(dir: &str) -> Vec<Input> {
    find_files(dir)
        .into_iter()
        .map(|f| {
            let task_path = format!("{}/{}", dir, f);
            let mut task_file = File::open(&task_path).unwrap();
            let mut task_str = String::new();
            task_file.read_to_string(&mut task_str).unwrap();
            let task_str = task_str.trim_end();
            let task = read_task(&task_str);
            Input::new(&f, task)
        })
        .collect::<Vec<_>>()
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
    let mut x = 0usize;
    loop {
        let c = iter.next().unwrap();
        if c.is_digit(10) {
            x = x * 10 + (c as u8 - b'0') as usize;
        } else {
            assert!(c == ',');
            break;
        }
    }
    let mut y = 0usize;
    loop {
        let c = iter.next().unwrap();
        if c.is_digit(10) {
            y = y * 10 + (c as u8 - b'0') as usize;
        } else {
            assert!(c == ')');
            break;
        }
    }
    Point::new(x, y)
}

fn read_map_internal(mut iter: &mut Peekable<Chars>) -> (Map, char) {
    let mut points = Vec::new();
    points.push(read_point(&mut iter));
    loop {
        let c = iter.next().unwrap();
        if c != ',' {
            return (Map::new(points), c);
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
            'C' => BoosterType::Cloning,
            _ => panic!("unknown type {}", c),
        };
        let point = read_point(&mut iter);
        res.push((booster_type, point));
        skip_or_empty(&mut iter, ';');
    }
    res
}

fn read_task(s: &str) -> Task {
    let mut iter = s.chars().peekable();
    let map = read_map(&mut iter);
    let initial = read_initial(&mut iter);
    let obstacles = read_obstacles(&mut iter);
    let boosters = read_boosters(&mut iter);
    Task {
        map,
        initial,
        obstacles,
        boosters,
    }
}

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
