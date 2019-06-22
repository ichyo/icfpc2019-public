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

    let width = map_points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = map_points.iter().map(|p| p.y).max().unwrap() + 1;
    let mut remaining = 0;
    let mut passed = Matrix::new(width, height, true);
    let mut valid = Matrix::new(width, height, false);

    for p in &map_points {
        passed.set(p, false);
        valid.set(p, true);
        remaining += 1;
    }

    for o in &task.obstacles {
        for p in o.enumerate_points().iter() {
            if p.y < height && p.x < width && valid.get(p) {
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
    if !passed.get(&cp) {
        passed.set(&cp, true);
        remaining -= 1;
    }
    while remaining > 0 {
        let mut data = Matrix::new(width, height, None);
        let mut queue = VecDeque::new();
        queue.push_back(cp);
        data.set(&cp, Some(Command::Noop));
        let mut reached = false;
        while let Some(c) = queue.pop_front() {
            if !passed.get(&c) {
                passed.set(&c, true);
                remaining -= 1;

                let mut local_cmds = Vec::new();
                let mut iter = c;
                while iter != cp {
                    let cmd = data.get(&iter).unwrap();
                    local_cmds.push(cmd);
                    iter = iter.revert_with(cmd).unwrap();
                }
                local_cmds.reverse();
                res.extend(local_cmds);

                cp = c;
                reached = true;
                break;
            }
            moves.shuffle(&mut rng);
            for m in &moves {
                if let Some(nc) = c.move_with(*m) {
                    if nc.x < width && nc.y < height && data.get(&nc).is_none() && valid.get(&nc) {
                        data.set(&nc, Some(*m));
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