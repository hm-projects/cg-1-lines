pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
}

pub fn overlap_for_colinear(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {
    // x
    let p_smallest_x = p1.x.min(p2.x);
    let p_largest_x = p1.x.max(p2.x);
    let q_smallest_x = q1.x.min(q2.x);
    let q_largest_x = q1.x.max(q2.x);

    let qx_not_in_px = q_smallest_x > p_largest_x || q_largest_x < p_smallest_x;

    if qx_not_in_px {
        // early return
        return false;
    }

    // y
    let p_smallest_y = p1.y.min(p2.y);
    let p_largest_y = p1.y.max(p2.y);
    let q_smallest_y = q1.y.min(q2.y);
    let q_largest_y = q1.y.max(q2.y);

    let qy_not_in_py = q_smallest_y > p_largest_y || q_largest_y < p_smallest_y;

    !qy_not_in_py
}

pub fn intersect(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {
    // let overlap = overlap_for_colinear(p1, p2, q1, q2);
    // if !overlap {
    //     return false;
    // }

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

    true
}