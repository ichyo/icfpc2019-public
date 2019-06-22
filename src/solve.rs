use crate::models::*;
use crate::utils::Matrix;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

trait Solver {
    fn solve(task: Task) -> Vec<Command>;
}

pub fn solve_small(task: Task) -> Vec<Command> {
    let mut rng = thread_rng();
    let map_points = task.map.enumerate_points();

    let width = map_points.iter().map(|p| p.x).max().unwrap() as usize + 1;
    let height = map_points.iter().map(|p| p.y).max().unwrap() as usize + 1;
    let mut remaining = 0;
    let mut passed = Matrix::new(width, height, true);
    let mut valid = Matrix::new(width, height, false);

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

    let mut moves = [
        Command::MoveUp,
        Command::MoveDown,
        Command::MoveLeft,
        Command::MoveRight,
    ];
    let mut res = Vec::new();
    let mut cp = task.initial;
    if !passed.get(cp).unwrap() {
        passed.set(cp, true);
        remaining -= 1;
    }
    let bodies_diff = vec![
        Point::new(0, 0),
        Point::new(1, 1),
        Point::new(1, 0),
        Point::new(1, -1),
    ];
    while remaining > 0 {
        let mut data = Matrix::new(width, height, None);
        let mut queue = VecDeque::new();
        queue.push_back(cp);
        data.set(cp, Some(Command::Noop));
        let mut reached = false;
        while let Some(c) = queue.pop_front() {
            let bodies = bodies_diff
                .iter()
                .map(|diff| c.add(diff))
                .collect::<Vec<_>>();
            let not_passed = bodies.iter().any(|p| {
                if let Some(false) = passed.get(*p) {
                    true
                } else {
                    false
                }
            });
            if not_passed {
                for body in bodies {
                    if let Some(false) = passed.try_set(body, true) {
                        remaining -= 1;
                    }
                }

                let mut local_cmds = Vec::new();
                let mut iter = c;
                while iter != cp {
                    let cmd = data.get(iter).unwrap().unwrap();
                    local_cmds.push(cmd);
                    iter = iter.revert_with(cmd);
                }
                local_cmds.reverse();
                res.extend(local_cmds);

                cp = c;
                reached = true;
                break;
            }
            moves.shuffle(&mut rng);
            for m in &moves {
                let nc = c.move_with(*m);
                if let Some(None) = data.get(nc) {
                    if let Some(true) = valid.get(nc) {
                        data.set(nc, Some(*m));
                        queue.push_back(nc);
                    }
                }
            }
        }
        if !reached {
            panic!("cannot reach anywhere: remaining {}", remaining);
        }
    }

    res
}