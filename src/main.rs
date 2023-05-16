struct Point {
    x: f64,
    y: f64,
}

fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}

fn overlap_for_colinear(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {
    // x
    let q1x_in_px = q1.x >= p1.x.min(p2.x) && q1.x <= p1.x.max(p2.x);
    let q2x_in_px = q2.x >= p2.x.min(p2.x) && q2.x <= p2.x.max(p2.x);

    let x_test = q1x_in_px || q2x_in_px;

    if !x_test {
        // early return
        return false;
    }

    // y
    let q1y_in_py = q1.y >= p1.y.min(p2.y) && q1.y <= p1.y.max(p2.y);
    let q2y_in_py = q2.y >= p2.y.min(p2.y) && q2.y <= p2.y.max(p2.y);

    let y_test = q1y_in_py || q2y_in_py;

    x_test && y_test
}

fn intersect(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {
    let ccwq1 = ccw(p1, p2, q1);
    let ccwq2 = ccw(p1, p2, q2);
    if ccwq1 * ccwq2 > 0.0 {
        return false;
    }

    let ccwp1 = ccw(q1, q2, p1);
    let ccwp2 = ccw(q1, q2, p2);
    if ccwp1 * ccwp2 > 0.0 {
        return false;
    }

    if ccwq1 == 0.0 && ccwq2 == 0.0 && ccwp1 == 0.0 && ccwp2 == 0.0 {
        // lines are colinear --> check for overlap
        return overlap_for_colinear(p1, p2, q1, q2);
    }

    return true;
}

use std::env;
use std::time::Instant;
use std::{fs, vec};

fn run(s: &String, intersection_test: fn(&Point, &Point, &Point, &Point) -> bool) {
    let points: Vec<(Point, Point)> = s
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

            return (p1, p2);
        })
        .collect();

    let mut count: i64 = 0;

    let now = Instant::now();

    let mut i: usize = 0;
    for line in points.iter() {
        for other_line in points.iter().skip(i + 1) {
            let b = intersection_test(&line.0, &line.1, &other_line.0, &other_line.1);
            if b {
                count += 1
            }
        }
        i += 1;
    }
    let elapsed = now.elapsed();

    println!("intersecting lines: {}", count);
    println!("elapsed time: {:.4?}", elapsed);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // read csv file
    let s = fs::read_to_string(path).expect("cannot read file");

    run(&s, intersect);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_intersect(first: (f64, f64, f64, f64), second: (f64, f64, f64, f64), expected: bool) {
        let p1 = Point {
            x: first.0,
            y: first.1,
        };
        let p2 = Point {
            x: first.2,
            y: first.3,
        };
        let q1 = Point {
            x: second.0,
            y: second.1,
        };
        let q2 = Point {
            x: second.2,
            y: second.3,
        };
        assert_eq!(intersect(&p1, &p2, &q1, &q2), expected);
    }

    #[test]
    fn test_colinear() {
        assert_intersect((0.0, 0.0, 0.0, 1.0), (1.0, 0.0, 1.0, 1.0), false);
    }

    #[test]
    fn test_basic_cross() {
        assert_intersect((1.0, 0.0, 1.0, 2.0), (0.0, 1.0, 2.0, 1.0), true);
    }

    #[test]
    fn test_basic_cross_swapped() {
        assert_intersect((0.0, 1.0, 2.0, 1.0), (1.0, 0.0, 1.0, 2.0), true);
    }

    #[test]
    fn test_basic_cross_inverted() {
        assert_intersect((0.0, 1.0, 2.0, 1.0), (1.0, 0.0, 1.0, 2.0), true);
    }

    #[test]
    fn test_sideways_t() {
        assert_intersect((0.0, 0.0, 0.0, 2.0), (0.0, 1.0, 2.0, 1.0), true);
    }

    #[test]
    fn test_right_angle() {
        assert_intersect((0.0, 0.0, 0.0, 2.0), (0.0, 2.0, 2.0, 2.0), true);
    }

    #[test]
    fn test_colinear_but_common_point() {
        assert_intersect((0.0, 0.0, 1.0, 0.0), (1.0, 0.0, 2.0, 0.0), true);
    }

    #[test]
    fn test_colinear_but_common_part() {
        assert_intersect((0.0, 0.0, 2.0, 0.0), (1.0, 0.0, 3.0, 0.0), true);
    }

    #[test]
    fn test_colinear_but_common_part_and_angled() {
        assert_intersect((0.0, 0.0, 1.0, 1.0), (0.5, 0.5, 1.5, 1.5), true);
    }

    #[test]
    fn test_colinear_and_apart() {
        assert_intersect((0.0, 0.0, 1.0, 0.0), (2.0, 0.0, 3.0, 0.0), false);
    }

    #[test]
    fn test_colinear_apart_and_angled() {
        assert_intersect((0.0, 0.0, 0.5, 0.5), (1.0, 1.0, 1.5, 1.5), false);
    }

    #[test]
    fn test_colinear_and_included() {
        assert_intersect((0.0, 0.0, 3.0, 0.0), (1.0, 0.0, 2.0, 0.0), true);
    }
}
