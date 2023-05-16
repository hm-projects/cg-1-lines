# Computational Geometry | Assignment 1: Lines

- Nicolas Bissig
- Antonino Grasso

## Quick Overview

Implementation: Rust

Quick summary: We test with the $ccw$ if two lines intersect, and for the colinear edge case, we use a simple overlap check

Results and runtime:
|Dataset|Amount of intersections|Runtime|
|---|---|---|
|s_1000_1.dat|11|~ 1 ms|
|s_10000_1.dat|733|~ 135 ms|
|s_100000_1.dat|77138|~ 15 s|

## Data structures

### Point

Our representation of a point in $\R^2$:

```rs
struct Point {
    x: f64,
    y: f64,
}
```

## Algorithm

The main loop expects a vector of point pairs, that represent a line.
Every line is checked for intersection with every other line, except for itself, and all pairs that already have been tested.

```rs
    let mut count: i64 = 0;
    let now = Instant::now();
    let mut i: usize = 0;
    for line in points.iter() {
        for other_line in points.iter().skip(i + 1) {
            let b = intersect(&line.0, &line.1, &other_line.0, &other_line.1);
            if b {
                count += 1
            }
        }
        i += 1;
    }
    let elapsed = now.elapsed();
    println!("intersecting lines: {}", count);
    println!("elapsed time: {:.4?}", elapsed);
```

The `intersect` function accepts four points, that represent two lines, and checks if these two lines intersect.
In the edge case of all $ccw$ results being $0$, we have a pair of colinear lines.
In this scenario, we have to check if these two lines overlap.

```rs
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
```

Before we explain the edge case handling, let's take a quick look at the $ccw$ function.

```rs
fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}
```

For the presented edge case, we utilize a simple overlap check for colinear lines.
Here we "project" the lines on the x and y axis, and then check if they overlap in both axis.

```rs
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
```

## Uniqueness of our solution

## Challenges

## Why is our solution correct?

## Bounding Box Test Early Return

(does not work as of now, because the overlap_for_colinear is strictly designed for colinear points)
