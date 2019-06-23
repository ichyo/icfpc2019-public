use crate::models::*;
use crate::utils::Range;
use rand::prelude::*;
use std::collections::VecDeque;

fn construct_map_from_ranges(ranges: &[Range]) -> Map {
    let len = ranges.len();
    let mut vertexes = Vec::new();
    vertexes.push(Point::new(0, ranges[0].start as i32));
    vertexes.push(Point::new(0, ranges[0].end as i32));
    for y in 0..len - 1 {
        if ranges[y].end != ranges[y + 1].end {
            vertexes.push(Point::new((y + 1) as i32, ranges[y].end as i32));
            vertexes.push(Point::new((y + 1) as i32, ranges[y + 1].end as i32));
        }
    }
    vertexes.push(Point::new(len as i32, ranges[len - 1].end as i32));
    vertexes.push(Point::new(len as i32, ranges[len - 1].start as i32));
    for y in (0..len - 1).rev() {
        if ranges[y].start != ranges[y + 1].start {
            vertexes.push(Point::new((y + 1) as i32, ranges[y + 1].start as i32));
            vertexes.push(Point::new((y + 1) as i32, ranges[y].start as i32));
        }
    }
    Map::new(vertexes)
}

fn consume_points_for(source: &mut VecDeque<Point>, num: usize, kind: BoosterType) -> Vec<Booster> {
    let mut res = Vec::new();
    for _ in 0..num {
        res.push(Booster::new(kind.clone(), source.pop_front().unwrap()));
    }
    res
}

pub fn solve_pazzle(puzzle: Puzzle) -> Option<Task> {
    let len = puzzle.max_length - 1;
    assert!(puzzle.includes.iter().all(|p| p.x < len as i32));
    assert!(puzzle.includes.iter().all(|p| p.y < len as i32));
    assert!(puzzle.excludes.iter().all(|p| p.x < len as i32));
    assert!(puzzle.excludes.iter().all(|p| p.y < len as i32));

    let mut include_xs = vec![vec![]; len];
    let mut exclude_xs = vec![vec![]; len];
    for p in &puzzle.excludes {
        exclude_xs[p.y as usize].push(p.x as usize);
    }
    for p in &puzzle.includes {
        include_xs[p.y as usize].push(p.x as usize);
    }

    let global_range = Range::new(0, len);
    let mut x_ranges = vec![vec![]; len];
    for y in 0..len {
        let exs = &mut exclude_xs[y];
        if exs.is_empty() {
            x_ranges[y].push(global_range);
        } else {
            exs.sort();
            x_ranges[y].push(global_range.split(exs[0]).0);
            x_ranges[y].push(global_range.split(exs[exs.len() - 1]).1);
            for i in 0..exs.len() - 1 {
                x_ranges[y].push(global_range.split(exs[i]).1.split(exs[i + 1]).0);
            }
        }
    }
    let mut reachables = vec![vec![]; len];
    for r in &x_ranges[0] {
        let ixs = &include_xs[0];
        if r.contains_all(ixs) {
            reachables[0].push((*r, *r));
        }
    }
    for y in 1..len {
        let mut next = Vec::new();
        let ixs = &include_xs[y];
        for to in &x_ranges[y] {
            if !to.contains_all(ixs) {
                continue;
            }
            for (from, _) in &reachables[y - 1] {
                if !to.intersect(*from) {
                    continue;
                }
                next.push((*to, *from));
                break;
            }
        }
        reachables[y].extend(next);
    }

    if reachables[len - 1].is_empty() {
        eprintln!("unreachable");
        return None;
    }

    let mut ranges = Vec::new();
    let (mut cur_r, mut next_r) = reachables[len - 1][0];
    ranges.push(cur_r);
    for y in (0..len - 1).rev() {
        let (new_cur_r, new_next_r) = reachables[y]
            .iter()
            .cloned()
            .find(|(r, _)| *r == next_r)
            .unwrap();
        cur_r = new_cur_r;
        next_r = new_next_r;
        ranges.push(cur_r);
    }
    ranges.reverse();

    let map = construct_map_from_ranges(&ranges);

    if map.len() < puzzle.vertex_min {
        eprintln!(
            "NG: vertex {} is less than {}",
            map.len(),
            puzzle.vertex_min
        );
        return None;
    }

    if map.len() > puzzle.vertex_max {
        eprintln!(
            "NG: vertex {} is greater than {}",
            map.len(),
            puzzle.vertex_max
        );
        return None;
    }

    let mut points = map.enumerate_points();
    if points.len() < len * len / 5 {
        eprintln!("NG: area {} is less than {}", points.len(), len * len / 5);
        return None;
    }

    let mut rand = thread_rng();
    points.shuffle(&mut rand);
    let mut point_source = points.into_iter().collect::<VecDeque<_>>();
    let initial = point_source.pop_front().unwrap();

    let mut boosters = Vec::new();
    boosters.extend(consume_points_for(
        &mut point_source,
        puzzle.hand_count,
        BoosterType::NewHand,
    ));
    boosters.extend(consume_points_for(
        &mut point_source,
        puzzle.fast_count,
        BoosterType::FastMove,
    ));
    boosters.extend(consume_points_for(
        &mut point_source,
        puzzle.drill_count,
        BoosterType::Drill,
    ));
    boosters.extend(consume_points_for(
        &mut point_source,
        puzzle.tele_count,
        BoosterType::Teleports,
    ));
    boosters.extend(consume_points_for(
        &mut point_source,
        puzzle.clone_count,
        BoosterType::Cloning,
    ));
    boosters.extend(consume_points_for(
        &mut point_source,
        puzzle.spawn_count,
        BoosterType::Spawn,
    ));

    let task = Task {
        width: len,
        height: len,
        map,
        initial,
        obstacles: Vec::new(),
        boosters,
    };

    Some(task)
}
