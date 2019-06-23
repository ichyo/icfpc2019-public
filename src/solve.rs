use crate::models::*;
use crate::utils::Matrix;

use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use std::time::Instant;

#[derive(Clone, Eq, PartialEq)]
pub struct Robot {
    current_point: Point,
    bodies_diff: Vec<Point>,
    new_bodies: VecDeque<Point>,
    commands: Vec<Command>,
    executed: Vec<Command>,
}

impl Robot {
    fn clone_from(robot: &Robot) -> Robot {
        let current_point = robot.current_point;

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
            //Point::new(0, -1),
            //Point::new(0, 1),
        ]);

        let commands = robot.commands.clone();
        let executed = Vec::new();
        Robot {
            current_point,
            bodies_diff,
            new_bodies,
            commands,
            executed,
        }
    }

    fn initialize(task: &Task) -> Robot {
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

        let commands = Vec::new();
        let executed = Vec::new();
        Robot {
            current_point,
            bodies_diff,
            new_bodies,
            commands,
            executed,
        }
    }

    fn move_with(&mut self, m: &Move) {
        self.current_point = self.current_point.move_with(m);
    }

    fn consume_new_hand(&mut self) -> Option<Point> {
        self.new_bodies.pop_front()
    }
    fn bodies(&self) -> Vec<Point> {
        self.bodies_diff
            .iter()
            .cloned()
            .map(|diff| self.current_point + diff)
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct State<'a> {
    task: &'a Task,
    turn: usize,
    valid: Matrix<bool>,
    passed: Matrix<bool>,
    booster_map: Matrix<Option<BoosterType>>,
    remaining_hand: usize,
    remaining_clone: usize,
    remaining_pass: usize,
    hand_count: usize,
    tele_count: usize,
    clone_count: usize,
    robots: Vec<Robot>,
}

impl<'a> State<'a> {
    fn initialize(task: &'a Task) -> State<'a> {
        let map_points = task.map.enumerate_points();

        let width = task.width;
        let height = task.height;

        let mut remaining_pass = 0;
        let mut remaining_hand = 0;
        let mut remaining_clone = 0;
        let mut booster_map = Matrix::new(width, height, None);
        let mut passed = Matrix::new(width, height, true);
        let mut valid = Matrix::new(width, height, false);

        for &p in &map_points {
            passed.set(p, false);
            valid.set(p, true);
            remaining_pass += 1;
        }

        for b in &task.boosters {
            booster_map.set(b.point, Some(b.kind.clone()));
            match b.kind {
                BoosterType::Cloning => remaining_clone += 1,
                BoosterType::NewHand => remaining_hand += 1,
                _ => {}
            }
        }

        if task.boosters.iter().all(|b| b.kind != BoosterType::Spawn) {
            remaining_clone = 0;
        }

        for o in &task.obstacles {
            for &p in o.enumerate_points().iter() {
                if let Some(true) = valid.get(p) {
                    valid.set(p, false);
                    passed.set(p, true);
                    remaining_pass -= 1;
                }
            }
        }

        let turn = 0;
        let hand_count = 0;
        let tele_count = 0;
        let clone_count = 0;
        let robots = vec![Robot::initialize(task)];

        State {
            task,
            turn,
            valid,
            passed,
            booster_map,
            remaining_hand,
            remaining_clone,
            remaining_pass,
            hand_count,
            tele_count,
            clone_count,
            robots,
        }
    }

    fn is_goal(&self, robot_idx: usize, goal: Point) -> bool {
        if self.remaining_clone > 0 {
            return match self.booster_map.get(goal) {
                Some(Some(BoosterType::Cloning)) => true,
                _ => false,
            };
        }

        if self.clone_count > 0 {
            return match self.booster_map.get(goal) {
                Some(Some(BoosterType::Spawn)) => true,
                _ => false,
            };
        }

        if self.remaining_hand > 0 {
            if let Some((first_robot_index, _)) = self
                .robots
                .iter()
                .enumerate()
                .find(|(_, r)| !r.new_bodies.is_empty())
            {
                if robot_idx == first_robot_index {
                    return match self.booster_map.get(goal) {
                        Some(Some(BoosterType::NewHand)) => true,
                        _ => false,
                    };
                }
            }
        }

        let not_passed = self.robots[robot_idx]
            .bodies_diff
            .iter()
            .map(|diff| goal + *diff)
            .any(|p| match self.passed.get(p) {
                Some(false) => true,
                _ => false,
            });

        let is_booster = match self.booster_map.get(goal) {
            Some(Some(BoosterType::NewHand)) => true,
            _ => false,
        };

        let is_valid = match self.valid.get(goal) {
            Some(true) => true,
            _ => false,
        };

        is_valid && (not_passed || is_booster)
    }

    fn count_pass(&self, robot_idx: usize, point: Point) -> usize {
        self.robots[robot_idx]
            .bodies_diff
            .iter()
            .map(|diff| point + *diff)
            .filter(|p| match self.passed.get(*p) {
                Some(false) => true,
                _ => false,
            })
            .count()
    }

    fn find_shortest_path(&self, robot_idx: usize, start: Point) -> Option<Vec<Move>> {
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

        let mut goal = None;
        let mut goal_value = None;

        while let Some(c) = queue.pop_front() {
            let (_, cost) = data[&c];

            if self.is_goal(robot_idx, c) {
                let value = (
                    u32::max_value() - cost,
                    self.count_pass(robot_idx, c),
                    rng.gen::<usize>(),
                );
                match goal_value {
                    Some(goal_value) if goal_value > value => {}
                    _ => {
                        goal = Some(c);
                        goal_value = Some(value);
                    }
                }
            }

            if goal.is_some() {
                continue;
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

        if let Some(goal) = goal {
            let mut res = Vec::new();
            let mut iter = goal;
            while iter != start {
                let (mv, _) = &data[&iter];
                iter = iter.revert_with(mv);
                res.push(mv.clone());
            }
            res.reverse();
            return Some(res);
        }

        None
    }

    fn pass_current_point(&mut self, robot_idx: usize) {
        let bodies = self.robots[robot_idx].bodies();
        let current_point = self.robots[robot_idx].current_point;
        for b in bodies {
            if let Some(false) = self.passed.get(b) {
                self.passed.set(b, true);
                self.remaining_pass -= 1;
            }
        }
        if let Some(Some(kind)) = self.booster_map.get(current_point) {
            match kind {
                BoosterType::NewHand => {
                    self.remaining_hand -= 1;
                    self.hand_count += 1;
                    self.booster_map.set(current_point, None);
                }
                BoosterType::Teleports => {
                    self.tele_count += 1;
                    self.booster_map.set(current_point, None);
                }
                BoosterType::Drill => {
                    self.booster_map.set(current_point, None);
                }
                BoosterType::Cloning => {
                    self.remaining_clone -= 1;
                    self.clone_count += 1;
                    self.booster_map.set(current_point, None);
                }
                BoosterType::Spawn => {}
                BoosterType::FastMove => {
                    self.booster_map.set(current_point, None);
                }
            }
        }
    }

    pub fn fill_next_command(&mut self, robot_idx: usize) {
        assert!(self.turn <= self.robots[robot_idx].commands.len());
        let current_point = self.robots[robot_idx].current_point;

        if self.hand_count > 0 {
            let robot = &mut self.robots[robot_idx];
            if let Some(new_hand) = robot.consume_new_hand() {
                self.hand_count -= 1;
                robot.commands.insert(self.turn, Command::NewHand(new_hand));
                robot.commands.truncate(self.turn + 1);
                return;
            }
        }

        if self.clone_count > 0 {
            let robot = &mut self.robots[robot_idx];
            if let Some(Some(BoosterType::Spawn)) = self.booster_map.get(current_point) {
                self.clone_count -= 1;
                robot.commands.insert(self.turn, Command::Cloning);
                robot.commands.truncate(self.turn + 1);
                return;
            }
        }

        if self.turn < self.robots[robot_idx].commands.len() {
            return;
        }

        let base_moves = self.find_shortest_path(robot_idx, current_point);
        if let Some(base_moves) = base_moves {
            for m in &base_moves {
                self.robots[robot_idx]
                    .commands
                    .push(Command::Move(m.clone()));
            }
            if !base_moves.is_empty() {
                return;
            }
        }

        self.robots[robot_idx]
            .commands
            .push(Command::Move(Move::Noop));
    }

    // true if it continues
    pub fn next_state(&mut self) -> bool {
        let turn = self.turn;
        let robots_len = self.robots.len();
        for idx in 0..robots_len {
            self.pass_current_point(idx);

            if self.remaining_pass == 0 {
                return false;
            }

            self.fill_next_command(idx);

            assert!(turn < self.robots[idx].commands.len());
            let m = self.robots[idx].commands[turn].clone();
            match &m {
                Command::Move(m) => {
                    self.robots[idx].move_with(&m);
                }
                Command::NewHand(ref p) => {
                    self.robots[idx].bodies_diff.push(*p);
                }
                Command::Cloning => {
                    let new_robot = Robot::clone_from(&self.robots[idx]);
                    assert!(new_robot.commands.len() == self.turn + 1);
                    self.robots.push(new_robot);
                }
                _ => unreachable!(),
            }
            self.robots[idx].executed.push(m);
        }
        self.turn += 1;
        assert!(self.turn < 1_000_000_000);

        self.remaining_pass > 0
    }
}

pub fn solve_small_while(task: Task, duration: Duration) -> Commands {
    let mut res = solve_small(task.clone());
    let now = Instant::now();;
    loop {
        if now.elapsed() >= duration {
            break;
        }
        let new = solve_small(task.clone());
        if new.len() < res.len() {
            res = new;
        }
    }
    res
}

pub fn solve_small(task: Task) -> Commands {
    let mut state = State::initialize(&task);
    loop {
        if !state.next_state() {
            break;
        }
    }
    Commands::new(
        state
            .robots
            .into_iter()
            .map(|r| r.executed)
            .collect::<Vec<_>>(),
    )
}
