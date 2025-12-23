use point::Point;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LineSegment {
    pub start: Point<2, i32>,
    pub end: Point<2, i32>
}

#[derive(Debug, PartialEq)]
pub enum LineIntersection {
    None,
    Point(Point<2, f64>),
    Overlap(LineSegment)
}

impl LineSegment {
    pub fn intersects(&self, other: &Self) -> LineIntersection {
        let p = self.start;
        let q = other.start;
        let r = Point { coords: [self.end.x() - self.start.x(), self.end.y() - self.start.y()] };
        let s = Point { coords: [other.end.x() - other.start.x(), other.end.y() - other.start.y()] };
        let r_cross_s = r.cross(&s);
        let q_minus_p = Point {coords: [q.x() - p.x(), q.y() - p.y()]};
        let q_minus_p_cross_r = q_minus_p.cross(&r);

        // Parellel
        if r_cross_s == 0 {
            if q_minus_p_cross_r == 0 {
                // Collinear: Check for overlap
                return self.solve_collinear(other);
            } else {
                // Parallel and non-collinear
                return LineIntersection::None;
            }
        }

        // Intersection Point
        // t = (q - p) × s / (r × s)
        // u = (p - q) × r / (s × r)
        let t = q_minus_p.cross(&s) as f64 / r_cross_s as f64;
        let u = q_minus_p_cross_r as f64 / r_cross_s as f64;

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            let intersect_x = p.x() as f64 + t * r.x() as f64;
            let intersect_y = p.y() as f64 + t * r.y() as f64;
            LineIntersection::Point(Point { coords: [intersect_x, intersect_y] })
        } else {
            LineIntersection::None
        }
    }

    fn solve_collinear(&self, other: &LineSegment) -> LineIntersection {
        // Sort points so start <= end (lexicographically)
        fn sort_points(a: Point<2, i32>, b: Point<2, i32>) -> (Point<2, i32>, Point<2, i32>) {
            if a.coords < b.coords { (a, b) } else { (b, a) }
        }

        let (s1, e1) = sort_points(self.start, self.end);
        let (s2, e2) = sort_points(other.start, other.end);

        // Find the "max of starts" and "min of ends"
        let overlap_start = if s1.coords > s2.coords { s1 } else { s2 };
        let overlap_end = if e1.coords < e2.coords { e1 } else { e2 };

        // Check if the overlap is valid
        if overlap_start.coords < overlap_end.coords {
            // Overlap is a segment
            LineIntersection::Overlap(LineSegment {
                start: overlap_start,
                end: overlap_end,
            })
        } else if overlap_start.coords == overlap_end.coords {
            // Overlap is exactly one point
            LineIntersection::Point(Point {
                coords: [overlap_start.x() as f64, overlap_start.y() as f64],
            })
        } else {
            LineIntersection::None
        }
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_point() {
        let seg1 = LineSegment {
            start: [0, 0].into(),
            end: [2, 2].into()
        };
        let seg2 = LineSegment {
            start: [0, 2].into(),
            end: [2, 0].into() 
        };
        assert_eq!(seg1.intersects(&seg2), LineIntersection::Point([1.0, 1.0].into()) )
    }
     #[test]
    fn intersection_end() {
        let seg1 = LineSegment {
            start: [0, 0].into(),
            end: [2, 2].into()
        };
        let seg2 = LineSegment {
            start: [2, 2].into(),
            end: [2, 4].into() 
        };
        assert_eq!(seg1.intersects(&seg2), LineIntersection::Point([2.0, 2.0].into()) )
    }
    #[test]
    fn intersection_none() {
        let seg1 = LineSegment {
            start: [0, 0].into(),
            end: [2, 2].into()
        };
        let seg2 = LineSegment {
            start: [3, 3].into(),
            end: [4, 4].into() 
        };
        assert_eq!(seg1.intersects(&seg2), LineIntersection::None )
    }

    #[test]
    fn intersection_overlap() {
        let seg1 = LineSegment {
            start: [0, 0].into(),
            end: [4, 4].into()
        };
        let seg2 = LineSegment {
            start: [1, 1].into(),
            end: [5, 5].into() 
        };
        assert_eq!(seg1.intersects(&seg2), LineIntersection::Overlap(LineSegment { start: [1, 1].into(), end: [4, 4].into() }) )
    }
    #[test]
    fn intersection_collinear_one_point() {
        let seg1 = LineSegment {
            start: [0, 0].into(),
            end: [4, 4].into()
        };
        let seg2 = LineSegment {
            start: [4, 4].into(),
            end: [5, 5].into() 
        };
        assert_eq!(seg1.intersects(&seg2), LineIntersection::Point([4.0, 4.0].into()) )
    }
}