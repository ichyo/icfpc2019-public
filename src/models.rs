use std::cmp;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn move_with(&self, command: Command) -> Option<Point> {
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

    pub fn revert_with(&self, command: Command) -> Option<Point> {
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

pub enum Direction {
    Verticle,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct Map(Vec<Point>);

impl Map {
    pub fn new(ps: Vec<Point>) -> Map {
        Map(ps)
    }
    pub fn iter_lines(&self) -> Vec<(Direction, Point, Point)> {
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

    pub fn enumerate_points(&self) -> Vec<Point> {
        let g_min_x = self.0.iter().map(|p| p.x).min().unwrap();
        let g_max_x = self.0.iter().map(|p| p.x).max().unwrap();
        let mut cross_y_map = HashMap::new();
        for (dir, p, q) in self.iter_lines() {
            if let Direction::Horizontal = dir {
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
pub enum BoosterType {
    NewHand,
    FastMove,
    Drill,
    Teleports,
    Cloning,
    Unknown,
}

pub type Booster = (BoosterType, Point);

#[derive(Debug, Clone)]
pub struct Task {
    pub map: Map,
    pub initial: Point,
    pub obstacles: Vec<Map>,
    pub boosters: Vec<Booster>,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Noop,
    TurnRight,
    TurnLeft,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::MoveUp => write!(f, "W"),
            Command::MoveDown => write!(f, "S"),
            Command::MoveLeft => write!(f, "A"),
            Command::MoveRight => write!(f, "D"),
            Command::Noop => write!(f, "Z"),
            Command::TurnRight => write!(f, "E"),
            Command::TurnLeft => write!(f, "Q"),
        }
    }
}
