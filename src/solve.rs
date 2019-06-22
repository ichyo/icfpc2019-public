use crate::models::*;

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
    let mut passed = vec![vec![true; width]; height];
    let mut valid = vec![vec![false; width]; height];

    for p in &map_points {
        passed[p.y][p.x] = false;
        valid[p.y][p.x] = true;
        remaining += 1;
    }

    for o in &task.obstacles {
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
    let mut cp = task.initial;
    if !passed[cp.y][cp.x] {
        passed[cp.y][cp.x] = true;
        remaining -= 1;
    }
    while remaining > 0 {
        let mut data = vec![vec![None; width]; height];
        let mut queue = VecDeque::new();
        queue.push_back(cp);
        data[cp.y][cp.x] = Some(Command::Noop);
        let mut reached = false;
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
                reached = true;
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
        if !reached {
            panic!("cannot reach anywhere: remaining {}", remaining);
        }
    }

    res
}