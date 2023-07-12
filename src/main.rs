mod geometry;
use geometry::intersect;
use geometry::Point;

use std::env;
use std::fs;
use std::time::Instant;

fn run(s: &str) {
    let points: Vec<(Point, Point)> = s
        .lines()
        .map(|l| {
            let splits: Vec<&str> = l.split(' ').collect();

            let p1 = Point {
                x: splits[0].parse().expect("should be a number"),
                y: splits[1].parse().expect("should be a number"),
            };
            let p2 = Point {
                x: splits[2].parse().expect("should be a number"),
                y: splits[3].parse().expect("should be a number"),
            };

            (p1, p2)
        })
        .collect();

    let mut count: i64 = 0;

    let now = Instant::now();

    for (i, line) in points.iter().enumerate() {
        for other_line in points.iter().skip(i + 1) {
            let b = intersect(&line.0, &line.1, &other_line.0, &other_line.1);
            if b {
                count += 1
            }
        }
    }
    let elapsed = now.elapsed();

    println!("intersecting lines: {}", count);
    println!("elapsed time: {:.4?}", elapsed);
}

fn process_file(filename: &String) {
    println!("processing file: {}", filename);
    let s = fs::read_to_string(filename).expect("cannot read file");

    run(&s);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for file in args.iter().skip(1) {
        process_file(file)
    }
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
        // assert all permutations as well
        assert_eq!(intersect(&p1, &p2, &q1, &q2), expected);
        assert_eq!(intersect(&p1, &p2, &q2, &q1), expected);
        assert_eq!(intersect(&p2, &p1, &q1, &q2), expected);
        assert_eq!(intersect(&p2, &p1, &q2, &q1), expected);

        // swap x and y coordinates
        let p1 = Point {
            x: first.1,
            y: first.0,
        };
        let p2 = Point {
            x: first.3,
            y: first.2,
        };
        let q1 = Point {
            x: second.1,
            y: second.0,
        };
        let q2 = Point {
            x: second.3,
            y: second.2,
        };
        // assert all permutations as well
        assert_eq!(intersect(&p1, &p2, &q1, &q2), expected);
        assert_eq!(intersect(&p1, &p2, &q2, &q1), expected);
        assert_eq!(intersect(&p2, &p1, &q1, &q2), expected);
        assert_eq!(intersect(&p2, &p1, &q2, &q1), expected);
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
    fn test_parallel() {
        assert_intersect((0.0, 0.0, 2.0, 2.0), (2.0, 1.0, 4.0, 3.0), false);
    }

    #[test]
    fn test_parallel2() {
        assert_intersect((2.0, 5.0, 5.0, 4.0), (3.0, 2.0, 6.0, 1.0), false);
    }

    #[test]
    fn test_colinear_but_common_part() {
        assert_intersect((0.0, 0.0, 2.0, 0.0), (1.0, 0.0, 3.0, 0.0), true);
    }

    #[test]
    fn test_colinear_but_common_part_swapped() {
        assert_intersect((0.0, 0.0, 2.0, 0.0), (3.0, 0.0, 1.0, 0.0), true);
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
