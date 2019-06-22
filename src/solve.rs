use crate::models::*;
use crate::utils::Matrix;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Eq, PartialEq)]
pub struct State<'a> {
    task: &'a Task,
    current_point: Point,
    valid: Matrix<bool>,
    passed: Matrix<bool>,
    booster_map: Matrix<Option<BoosterType>>,
    bodies_diff: Vec<Point>,
    new_bodies: VecDeque<Point>,
    remaining: usize,
    hand_count: usize,
    tele_count: usize,
    commands: Vec<Command>,
}

impl<'a> State<'a> {
    fn initialize(task: &'a Task) -> State<'a> {
        let map_points = task.map.enumerate_points();

        let width = task.width;
        let height = task.height;

        let mut remaining = 0;
        let mut booster_map = Matrix::new(width, height, None);
        let mut passed = Matrix::new(width, height, true);
        let mut valid = Matrix::new(width, height, false);

        for &p in &map_points {
            passed.set(p, false);
            valid.set(p, true);
            remaining += 1;
        }

        for b in &task.boosters {
            booster_map.set(b.point, Some(b.kind.clone()));
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


        let current_point = task.initial;

        let bodies_diff = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(1, -1),
        ];
        let new_bodies = VecDeque::from(vec![
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(-1, -1),
            Point::new(0, -1),
            Point::new(0, 1),
        ]);

        let hand_count = 0;
        let tele_count = 0;
        let commands = Vec::new();

        State {
            task,
            current_point,
            valid,
            passed,
            booster_map,
            bodies_diff,
            new_bodies,
            remaining,
            hand_count,
            tele_count,
            commands,
        }
    }

    fn find_shortest_path(&self, start: Point) -> Vec<Move> {
        let mut rng = thread_rng();
        let mut moves = [
            Move::MoveUp,
            Move::MoveDown,
            Move::MoveLeft,
            Move::MoveRight,
        ];

        let mut data: HashMap<Point, (Move, u32)> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        data.insert(start, (Move::Noop, 0));
        while let Some(c) = queue.pop_front() {
            let (_, cost) = data[&c];

            let not_passed =
                self.bodies_diff
                    .iter()
                    .map(|diff| c + *diff)
                    .any(|p| match self.passed.get(p) {
                        Some(false) => true,
                        _ => false,
                    });

            let is_booster = match self.booster_map.get(c) {
                Some(Some(BoosterType::NewHand)) => true,
                _ => false,
            };

            let is_valid = match self.valid.get(c) {
                Some(true) => true,
                _ => false,
            };

            if is_valid && (not_passed || is_booster) {
                let mut res = Vec::new();
                let mut iter = c;
                while iter != start {
                    let (mv, _) = &data[&iter];
                    iter = iter.revert_with(mv);
                    res.push(mv.clone());
                }
                res.reverse();
                return res;
            }

            moves.shuffle(&mut rng);
            for m in &moves {
                let nc = c.move_with(m);
                if let Some(true) = self.valid.get(nc) {
                    data.entry(nc).or_insert_with(|| {
                        queue.push_back(nc);
                        (m.clone(), cost + 1)
                    });
                }
            }
        }
        panic!("cannot reach anywhere");
    }

    fn pass_current_point(&mut self) {
        let bodies = self
            .bodies_diff
            .iter()
            .cloned()
            .map(|diff| self.current_point + diff)
            .collect::<Vec<_>>();
        for b in bodies {
            if let Some(false) = self.passed.get(b) {
                self.passed.set(b, true);
                self.remaining -= 1;
            }
        }
        if let Some(Some(kind)) = self.booster_map.get(self.current_point) {
            match kind {
                BoosterType::NewHand => self.hand_count += 1,
                BoosterType::Teleports => self.tele_count += 1,
                BoosterType::Drill => {}
                _ => {}
            }
            self.booster_map.set(self.current_point, None);
        }
    }

    // true if it continues
    pub fn next_state(&mut self) -> bool {
        while self.hand_count > 0 && !self.new_bodies.is_empty() {
            let new_hand = self.new_bodies.pop_front().unwrap();
            self.hand_count -= 1;
            self.bodies_diff.push(new_hand);
            self.commands.push(Command::NewHand(new_hand));
        }

        self.pass_current_point();
        if self.remaining == 0 {
            return false;
        }

        let base_moves = self.find_shortest_path(self.current_point);

        for m in base_moves {
            self.current_point = self.current_point.move_with(&m);
            self.pass_current_point();
            self.commands.push(Command::Move(m.clone()));
        }

        self.remaining > 0
    }
}

pub fn solve_small(task: Task) -> Vec<Command> {
    let times = 5_000_000 / task.width / task.height;
    (0..times)
        .map(|_| {
            let mut state = State::initialize(&task);
            loop {
                if !state.next_state() {
                    break;
                }
            }
            state.commands
        })
        .min_by_key(|c| c.len())
        .unwrap()
}