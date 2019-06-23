use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)?;
        Ok(())
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn move_with(self, kind: &Move) -> Point {
        let (x, y) = (self.x, self.y);
        match kind {
            Move::MoveUp => Point::new(x, y + 1),
            Move::MoveDown => Point::new(x, y - 1),
            Move::MoveRight => Point::new(x + 1, y),
            Move::MoveLeft => Point::new(x - 1, y),
            _ => self,
        }
    }

    pub fn revert_with(self, kind: &Move) -> Point {
        let (x, y) = (self.x, self.y);
        match kind {
            Move::MoveUp => Point::new(x, y - 1),
            Move::MoveDown => Point::new(x, y + 1),
            Move::MoveRight => Point::new(x - 1, y),
            Move::MoveLeft => Point::new(x + 1, y),
            _ => unreachable!(),
        }
    }
}

pub enum Direction {
    Verticle,
    Horizontal,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map(pub Vec<Point>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|p| format!("{}", p)).collect::<Vec<_>>().join(","))?;
        Ok(())
    }
}

impl Map {
    pub fn new(ps: Vec<Point>) -> Map {
        Map(ps)
    }

    pub fn compute_width(&self) -> usize {
        self.0.iter().map(|p| p.x).max().unwrap() as usize + 1
    }

    pub fn compute_height(&self) -> usize {
        self.0.iter().map(|p| p.y).max().unwrap() as usize + 1
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
                    cross_y_map.entry(x).or_insert_with(Vec::new).push(p.y);
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BoosterType {
    NewHand,
    FastMove,
    Drill,
    Teleports,
    Cloning,
    Spawn,
}

impl fmt::Display for BoosterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoosterType::NewHand => write!(f, "B"),
            BoosterType::FastMove => write!(f, "F"),
            BoosterType::Drill => write!(f, "L"),
            BoosterType::Teleports => write!(f, "R"),
            BoosterType::Cloning => write!(f, "C"),
            BoosterType::Spawn => write!(f, "X"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Booster {
    pub kind: BoosterType,
    pub point: Point,
}

impl Booster {
    pub fn new(kind: BoosterType, point: Point) -> Booster {
        Booster { kind, point }
    }
}

impl fmt::Display for Booster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.kind, self.point)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub width: usize,
    pub height: usize,
    pub map: Map,
    pub initial: Point,
    pub obstacles: Vec<Map>,
    pub boosters: Vec<Booster>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#", self.map)?;
        write!(f, "{}#", self.initial)?;
        write!(f, "{}#", self.obstacles.iter().map(|o| format!("{}", o)).collect::<Vec<_>>().join(";"))?;
        write!(f, "{}", self.boosters.iter().map(|b| format!("{}", b)).collect::<Vec<_>>().join(";"))?;
        Ok(())
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Move {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Noop,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Move(Move),
    TurnRight,
    TurnLeft,
    NewHand(Point),
    FastWheel,
    Drill,
    ResetBeacon,
    ShiftBeacon(Point),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Move(Move::MoveUp) => write!(f, "W"),
            Command::Move(Move::MoveDown) => write!(f, "S"),
            Command::Move(Move::MoveLeft) => write!(f, "A"),
            Command::Move(Move::MoveRight) => write!(f, "D"),
            Command::Move(Move::Noop) => write!(f, "Z"),
            Command::TurnRight => write!(f, "E"),
            Command::TurnLeft => write!(f, "Q"),
            Command::NewHand(p) => write!(f, "B({}, {})", p.x, p.y),
            Command::FastWheel => write!(f, "F"),
            Command::Drill => write!(f, "L"),
            Command::ResetBeacon => write!(f, "R"),
            Command::ShiftBeacon(p) => write!(f, "T({}, {})", p.x, p.y),
        }
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub block: usize,
    pub epock: usize,
    pub max_length: usize,
    pub vertex_min: usize,
    pub vertex_max: usize,
    pub hand_count: usize,
    pub fast_count: usize,
    pub drill_count: usize,
    pub tele_count: usize,
    pub clone_count: usize,
    pub spawn_count: usize,
    pub includes: Vec<Point>,
    pub excludes: Vec<Point>,
}
