struct Point {
    x: f64,
    y: f64,
}

fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}

fn intersect(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {
    if ccw(p1, p2, q1) * ccw(p1, p2, q2) > 0.0 {
        return false;
    }
    if ccw(q1, q2, p1) * ccw(q1, q2, p2) > 0.0 {
        return false;
    }
    return true;
}

use std::env;
use std::time::Instant;
use std::{fs, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // read csv file
    let s = fs::read_to_string(path).expect("cannot read file");

    let points: Vec<Vec<Point>> = s
        .lines()
        .into_iter()
        .map(|l| {
            let splits: Vec<&str> = l.split(" ").collect();

            let p1 = Point {
                x: splits[0].parse().expect("should be a number"),
                y: splits[1].parse().expect("should be a number"),
            };
            let p2 = Point {
                x: splits[2].parse().expect("should be a number"),
                y: splits[3].parse().expect("should be a number"),
            };

            return vec![p1, p2];
        })
        .collect();

    let mut count: i64 = 0;

    let now = Instant::now();
    for line in &points {
        for other_line in &points {
            let b = intersect(&line[0], &line[1], &other_line[0], &other_line[1]);
            if b {
                count += 1
            }
        }
    }
    let elapsed = now.elapsed();

    println!("intersecting lines: {}", count);
    println!("elapsed time: {:.4?}", elapsed);
}
