use crate::models::*;
use crate::utils::Matrix;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

trait Solver {
    fn solve(task: Task) -> Vec<Command>;
}

fn find_shortest_path(
    width: usize,
    height: usize,
    valid: &Matrix<bool>,
    passed: &Matrix<bool>,
    start: Point,
    bodies_diff: &[Point],
    booster_map: &Matrix<Option<BoosterType>>,
    drill_mode: bool,
) -> Vec<Move> {
    let mut rng = thread_rng();
    let mut moves = [
        Move::MoveUp,
        Move::MoveDown,
        Move::MoveLeft,
        Move::MoveRight,
    ];

    let mut data: Matrix<Option<(Move, u16)>> = Matrix::new(width, height, None);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    data.set(start, Some((Move::Noop, 0)));
    while let Some(c) = queue.pop_front() {
        let cost = match data.get(c) {
            Some(Some((_, cost))) => *cost,
            _ => panic!("no data is expected"),
        };

        let not_passed = bodies_diff
            .iter()
            .map(|diff| c + *diff)
            .any(|p| match passed.get(p) {
                Some(false) => true,
                _ => false,
            });

        let is_booster = match booster_map.get(c) {
            Some(Some(_)) => true,
            _ => false,
        };

        let is_valid = match valid.get(c) {
            Some(true) => true,
            _ => false,
        };

        if is_valid && (not_passed || is_booster) {
            let mut res = Vec::new();
            let mut iter = c;
            while iter != start {
                let (mv, _cost) = match data.get(iter) {
                    Some(Some((mv, cost))) => (mv, cost),
                    _ => panic!("no data"),
                };
                iter = iter.revert_with(mv);
                res.push(mv.clone());
            }
            res.reverse();
            return res;
        }

        moves.shuffle(&mut rng);
        for m in &moves {
            let nc = c.move_with(m);
            if let Some(None) = data.get(nc) {
                match (valid.get(nc), drill_mode) {
                    (Some(true), _) | (Some(false), true) => {
                        data.set(nc, Some((m.clone(), cost + 1)));
                        queue.push_back(nc);
                    }
                    _ => {}
                }
            }
        }
    }
    panic!("cannot reach anywhere");
}

fn update_point(
    point: Point,
    bodies_diff: &[Point],
    passed: &mut Matrix<bool>,
    booster_map: &mut Matrix<Option<BoosterType>>,
    hand_count: &mut usize,
    drill_count: &mut usize,
    remaining: &mut usize,
) {
    bodies_diff.iter().map(|diff| point + *diff).for_each(|b| {
        if let Some(false) = passed.get(b) {
            passed.set(b, true);
            *remaining -= 1;
        }
    });
    if let Some(Some(kind)) = booster_map.get(point) {
        match kind {
            BoosterType::NewHand => *hand_count += 1,
            BoosterType::Drill => *drill_count += 1,
            _ => {}
        }
        booster_map.set(point, None);
    }
}

pub fn solve_small(task: Task) -> Vec<Command> {
    let map_points = task.map.enumerate_points();

    let width = map_points.iter().map(|p| p.x).max().unwrap() as usize + 1;
    let height = map_points.iter().map(|p| p.y).max().unwrap() as usize + 1;

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

    let mut res = Vec::new();
    let mut current_point = task.initial;

    let mut bodies_diff = vec![
        Point::new(0, 0),
        Point::new(1, 1),
        Point::new(1, 0),
        Point::new(1, -1),
    ];
    let mut new_bodies = VecDeque::from(vec![
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(-1, -1),
        Point::new(0, -1),
        Point::new(0, 1),
    ]);

    let mut hand_count = 0;
    let mut drill_count = 0;

    while remaining > 0 {
        while hand_count > 0 && !new_bodies.is_empty() {
            let new_hand = new_bodies.pop_front().unwrap();
            hand_count -= 1;
            bodies_diff.push(new_hand);
            res.push(Command::NewHand(new_hand));
        }
        update_point(
            current_point,
            &bodies_diff,
            &mut passed,
            &mut booster_map,
            &mut hand_count,
            &mut drill_count,
            &mut remaining,
        );
        let moves = find_shortest_path(
            width,
            height,
            &valid,
            &passed,
            current_point,
            &bodies_diff,
            &booster_map,
            false,
        );
        let moves = if drill_count > 0 {
            let drill_moves = find_shortest_path(
                width,
                height,
                &valid,
                &passed,
                current_point,
                &bodies_diff,
                &booster_map,
                true,
            );
            if drill_moves.len() * 2 <= moves.len()
                && drill_moves.len() >= 10
                && drill_moves.len() < 30
            {
                drill_count -= 1;
                res.push(Command::Drill);
                drill_moves
            } else {
                moves
            }
        } else {
            moves
        };
        for m in moves {
            current_point = current_point.move_with(&m);
            update_point(
                current_point,
                &bodies_diff,
                &mut passed,
                &mut booster_map,
                &mut hand_count,
                &mut drill_count,
                &mut remaining,
            );
            res.push(Command::Move(m));
        }
    }

    res
}
