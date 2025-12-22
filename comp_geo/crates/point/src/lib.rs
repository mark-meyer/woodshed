use num_traits::Num;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<const D: usize, N> {
    pub coords: [N; D],
}

impl<const D: usize, N: Default + Copy> Default for Point<D, N> {
    fn default() -> Self {
        Self {
            coords: [N::default(); D],
        }
    }
}

impl<const D: usize, N> From<[N; D]> for Point<D, N> {
    fn from(coords: [N; D]) -> Self {
        Self { coords }
    }
}

// 2D implementations
impl<N: Copy> Point<2, N> {
    pub fn x(&self) -> N {
        self.coords[0]
    }
    pub fn y(&self) -> N {
        self.coords[1]
    }
}

// 3D implementations
impl<N: Copy> Point<3, N> {
    pub fn x(&self) -> N {
        self.coords[0]
    }
    pub fn y(&self) -> N {
        self.coords[1]
    }
    pub fn z(&self) -> N {
        self.coords[2]
    }
}

// Generic math functions
impl<const D: usize, N: Num + Copy> Point<D, N> {
    pub fn square_distance(&self, other: &Self) -> N
    where
        N: std::iter::Sum,
    {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(&a, &b)| (a - b) * (a - b))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_distance() {
        let p1 = Point::default();
        let p2: Point<2, i32> = [4, 3].into();
        let sq_dist = p1.square_distance(&p2);
        assert_eq!(sq_dist, 25);
    }
    #[test]
    fn ordering() {
        let mut points: Vec<Point<3, i32>> = vec![[10, 10, 0].into(), [11, 1, -1].into(), [10, 9, 5].into()];
        points.sort_by_key(|p| (p.x(), p.y()));
        assert_eq!(points, vec![[10, 9, 5].into(), [10, 10, 0].into(), [11, 1, -1].into()] );
    }
}
