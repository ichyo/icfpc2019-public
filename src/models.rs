use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Place(Point, Direction);

impl Place {
    pub fn new(p: Point, d: Direction) -> Place {
        Place(p, d)
    }

    pub fn move_with(&self, m: &Move) -> Place {
        Place::new(self.0.move_with(m), self.1.move_with(m))
    }

    pub fn revert_with(&self, m: &Move) -> Place {
        Place::new(self.0.revert_with(m), self.1.revert_with(m))
    }

    pub fn point(&self) -> Point {
        self.0
    }

    pub fn dir(&self) -> Direction {
        self.1
    }

    pub fn hand(&self, r: Point) -> Point {
        self.0 + self.1.convert(r)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        }
    }
    pub fn convert(self, p: Point) -> Point {
        match self {
            Direction::Right => p,
            Direction::Up => Point::new(-p.y, p.x),
            Direction::Left => Point::new(-p.x, -p.y),
            Direction::Down => Point::new(p.y, -p.x),
        }
    }
    pub fn reconvert(self, p: Point) -> Point {
        match self {
            Direction::Right => p,
            Direction::Up => Point::new(p.y, -p.x),
            Direction::Left => Point::new(-p.x, -p.y),
            Direction::Down => Point::new(-p.y, p.x),
        }
    }
    pub fn move_with(self, kind: &Move) -> Direction {
        match kind {
            Move::MoveUp => self,
            Move::MoveDown => self,
            Move::MoveRight => self,
            Move::MoveLeft => self,
            Move::Noop => self,
            Move::TurnLeft => self.turn_left(),
            Move::TurnRight => self.turn_right(),
        }
    }
    pub fn revert_with(self, kind: &Move) -> Direction {
        match kind {
            Move::MoveUp => self,
            Move::MoveDown => self,
            Move::MoveRight => self,
            Move::MoveLeft => self,
            Move::Noop => self,
            Move::TurnLeft => self.turn_right(),
            Move::TurnRight => self.turn_left(),
        }
    }
}

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
            Move::Noop => self,
            Move::TurnLeft => self,
            Move::TurnRight => self,
        }
    }

    pub fn revert_with(self, kind: &Move) -> Point {
        let (x, y) = (self.x, self.y);
        match kind {
            Move::MoveUp => Point::new(x, y - 1),
            Move::MoveDown => Point::new(x, y + 1),
            Move::MoveRight => Point::new(x - 1, y),
            Move::MoveLeft => Point::new(x + 1, y),
            Move::Noop => self,
            Move::TurnLeft => self,
            Move::TurnRight => self,
        }
    }
}

pub enum LineDirection {
    Verticle,
    Horizontal,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map(pub Vec<Point>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<_>>()
                .join(",")
        )?;
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

    pub fn iter_lines(&self) -> Vec<(LineDirection, Point, Point)> {
        let mut iter = self.0.iter().cloned().cycle().peekable();
        let mut res = Vec::new();
        for _ in 0..self.0.len() {
            let cur = iter.next().unwrap();
            let next = *iter.peek().unwrap();
            if cur.x == next.x {
                res.push((LineDirection::Verticle, cur, next));
            } else if cur.y == next.y {
                res.push((LineDirection::Horizontal, cur, next));
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
            if let LineDirection::Horizontal = dir {
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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
    pub id: String,
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
        write!(
            f,
            "{}#",
            self.obstacles
                .iter()
                .map(|o| format!("{}", o))
                .collect::<Vec<_>>()
                .join(";")
        )?;
        write!(
            f,
            "{}",
            self.boosters
                .iter()
                .map(|b| format!("{}", b))
                .collect::<Vec<_>>()
                .join(";")
        )?;
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
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Move(Move),
    NewHand(Point),
    FastWheel,
    Drill,
    ResetBeacon,
    ShiftBeacon(Point),
    Cloning,
}


impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Move(Move::MoveUp) => write!(f, "W"),
            Command::Move(Move::MoveDown) => write!(f, "S"),
            Command::Move(Move::MoveLeft) => write!(f, "A"),
            Command::Move(Move::MoveRight) => write!(f, "D"),
            Command::Move(Move::Noop) => write!(f, "Z"),
            Command::Move(Move::TurnRight) => write!(f, "E"),
            Command::Move(Move::TurnLeft) => write!(f, "Q"),
            Command::NewHand(p) => write!(f, "B({},{})", p.x, p.y),
            Command::FastWheel => write!(f, "F"),
            Command::Drill => write!(f, "L"),
            Command::ResetBeacon => write!(f, "R"),
            Command::ShiftBeacon(p) => write!(f, "T({},{})", p.x, p.y),
            Command::Cloning => write!(f, "C"),
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

pub struct Commands(Vec<Vec<Command>>);

impl Commands {
    pub fn new(cmds: Vec<Vec<Command>>) -> Commands {
        assert!(!cmds.is_empty());
        Commands(cmds)
    }
    pub fn len(&self) -> usize {
        self.0[0].len()
    }
    pub fn is_empty(&self) -> bool {
        self.0[0].is_empty()
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|cmds| {
                    cmds.iter()
                        .map(|c| format!("{}", c))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("#")
        )
    }
}

#[derive(Default)]
pub struct Buy(Vec<BoosterType>);

impl Buy {
    pub fn new() -> Buy {
        Buy(Vec::new())
    }
    pub fn push(&mut self, b: &BoosterType) {
        self.0.push(b.clone());
    }
    pub fn iter(&self) -> impl Iterator<Item=&BoosterType> {
        self.0.iter()
    }
    pub fn money(&self) -> usize {
        self.0.iter().map(|b|
            match b {
                BoosterType::Cloning => 2000,
                BoosterType::Drill => 700,
                BoosterType::Teleports => 1200,
                BoosterType::FastMove => 300,
                BoosterType::NewHand => 1000,
                BoosterType::Spawn => unreachable!(),
            }
        ).sum::<usize>()
    }
}

impl fmt::Display for Buy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|b| {
                    format!("{}", b)
                })
                .collect::<String>()
        )
    }
}