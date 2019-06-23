use crate::models::*;
use crate::utils::Matrix;

use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use std::time::Instant;

#[derive(Clone, Eq, PartialEq)]
pub struct Robot {
    current_place: Place,
    bodies_diff: Vec<Point>,
    new_bodies: VecDeque<Point>,
    commands: Vec<Command>,
    executed: Vec<Command>,
}

impl Robot {
    fn clone_from(robot: &Robot) -> Robot {
        let current_place = robot.current_place;

        let bodies_diff = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(1, -1),
        ];
        let new_bodies = VecDeque::from(vec![
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0),
            Point::new(5, 0),
            Point::new(6, 0),
            Point::new(7, 0),
            Point::new(8, 0),
        ]);

        let commands = robot.commands.clone();
        let executed = Vec::new();
        Robot {
            current_place,
            bodies_diff,
            new_bodies,
            commands,
            executed,
        }
    }

    fn initialize(task: &Task) -> Robot {
        let current_point = task.initial;
        let current_dir = Direction::Right;
        let current_place = Place::new(current_point, current_dir);

        let bodies_diff = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(1, -1),
        ];

        let new_bodies = VecDeque::from(vec![
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0),
            Point::new(5, 0),
            Point::new(6, 0),
            Point::new(7, 0),
            Point::new(8, 0),
            Point::new(9, 0),
            Point::new(10, 0),
            Point::new(11, 0),
            Point::new(12, 0),
            Point::new(13, 0),
        ]);

        let commands = Vec::new();
        let executed = Vec::new();
        Robot {
            current_place,
            bodies_diff,
            new_bodies,
            commands,
            executed,
        }
    }

    fn move_with(&mut self, m: &Move) {
        self.current_place = self.current_place.move_with(m);
    }

    fn consume_new_hand(&mut self) -> Option<Point> {
        self.new_bodies.pop_front()
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

    fn commands(&self) -> Commands {
        Commands::new(
            self.robots
                .iter()
                .map(|r| r.executed.clone())
                .collect::<Vec<_>>(),
        )
    }

    fn hand_reach(&self, place: Place, diff: Point) -> Option<Point> {
        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            let d = std::cmp::max(diff.x.abs(), diff.y.abs());
            assert!(diff.x.abs() == 0 || diff.x.abs() == d);
            assert!(diff.y.abs() == 0 || diff.y.abs() == d);
            let unit = Point::new(diff.x / d, diff.y / d);
            for i in 1..=d {
                let diff = Point::new(unit.x * i, unit.y * i);
                let hand = place.hand(diff);
                if let Some(true) = self.valid.get(hand) {
                } else {
                    return None;
                }
            }
        }
        Some(place.hand(diff))
    }

    fn is_goal(&self, robot_idx: usize, goal: Place) -> bool {
        if self.remaining_clone > 0 {
            return match self.booster_map.get(goal.point()) {
                Some(Some(BoosterType::Cloning)) => true,
                _ => false,
            };
        }

        if self.clone_count > 0 {
            return match self.booster_map.get(goal.point()) {
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
                    return match self.booster_map.get(goal.point()) {
                        Some(Some(BoosterType::NewHand)) => true,
                        _ => false,
                    };
                }
            }
        }

        let not_passed = self.robots[robot_idx]
            .bodies_diff
            .iter()
            .cloned()
            .filter_map(|diff| self.hand_reach(goal, diff))
            .any(|p| match self.passed.get(p) {
                Some(false) => true,
                _ => false,
            });

        let is_booster = match self.booster_map.get(goal.point()) {
            Some(Some(BoosterType::NewHand)) => true,
            _ => false,
        };

        let is_valid = match self.valid.get(goal.point()) {
            Some(true) => true,
            _ => false,
        };

        is_valid && (not_passed || is_booster)
    }

    fn count_pass(&self, robot_idx: usize, place: Place) -> usize {
        self.robots[robot_idx]
            .bodies_diff
            .iter()
            .cloned()
            .filter_map(|diff| self.hand_reach(place, diff))
            .filter(|p| match self.passed.get(*p) {
                Some(false) => true,
                _ => false,
            })
            .count()
    }

    fn find_shortest_path(&self, robot_idx: usize, start: Place) -> Option<Vec<Move>> {
        let mut rng = thread_rng();
        let mut moves = [
            Move::MoveUp,
            Move::MoveDown,
            Move::MoveLeft,
            Move::MoveRight,
            Move::TurnLeft,
            Move::TurnRight,
        ];

        let mut data: HashMap<Place, (Move, u32)> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        data.insert(start, (Move::Noop, 0));

        let mut goal = None;
        let mut goal_value = None;

        while let Some(place) = queue.pop_front() {
            let (_, cost) = data[&place];

            if self.is_goal(robot_idx, place) {
                let value = (
                    u32::max_value() - cost,
                    self.count_pass(robot_idx, place),
                    rng.gen::<usize>(),
                );
                match goal_value {
                    Some(goal_value) if goal_value > value => {}
                    _ => {
                        goal = Some(place);
                        goal_value = Some(value);
                    }
                }
            }

            if goal.is_some() {
                continue;
            }

            moves.shuffle(&mut rng);
            for m in &moves {
                let nplace = place.move_with(m);
                if let Some(true) = self.valid.get(nplace.point()) {
                    data.entry(nplace).or_insert_with(|| {
                        queue.push_back(nplace);
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
        let bodies = self.robots[robot_idx]
            .bodies_diff
            .iter()
            .filter_map(|diff| self.hand_reach(self.robots[robot_idx].current_place, *diff))
            .collect::<Vec<_>>();

        let current_point = self.robots[robot_idx].current_place.point();
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
        let current_place = self.robots[robot_idx].current_place;

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
            if let Some(Some(BoosterType::Spawn)) = self.booster_map.get(current_place.point()) {
                self.clone_count -= 1;
                robot.commands.insert(self.turn, Command::Cloning);
                robot.commands.truncate(self.turn + 1);
                return;
            }
        }

        if self.turn < self.robots[robot_idx].commands.len() {
            return;
        }

        let base_moves = self.find_shortest_path(robot_idx, current_place);
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
        if self.remaining_pass == 0 {
            return false;
        }

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
    state.commands()
}
