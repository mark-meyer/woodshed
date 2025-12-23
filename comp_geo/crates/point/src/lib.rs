use num_traits::Num;
use std::fmt::Debug;
use std::ops::{Add, Sub};

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

    pub fn cross(&self, other:&Self) -> N
    where 
        N: Num
    {
        self.x() * other.y() - self.y() * other.x()
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

/// Square Distance between two points
/// ```
/// use point::Point;
/// let p1 = Point {coords: [3, 4]};
/// let p2 = Point {coords: [0, 0]};
/// assert_eq!(&p1.square_distance(&p2), &25)
/// ```
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
/// Addition betweein points
/// ```
/// use point::Point;
/// let p1 = Point {coords: [3, 4]};
/// let p2 = Point {coords: [1, -1]};
/// assert_eq!(p1 + p2, Point {coords: [4, 3]})
/// ```
impl<const D: usize, N: Num + Copy> Add for Point<D, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let coords = std::array::from_fn(|i| {
            self.coords[i] + rhs.coords[i]
        });

        Point { coords }
    }
}

/// ```
/// use point::Point;
/// let p1 = Point {coords: [3, 4]};
/// let p2 = Point {coords: [1, -1]};
/// assert_eq!(&p1 + &p2, Point {coords: [4, 3]})
/// ```
impl<'a, 'b, const D: usize, N: Num + Copy> Add<&'b Point<D, N>> for &'a Point<D, N> {
    type Output = Point<D, N>;

    fn add(self, rhs: &'b Point<D, N>) -> Self::Output {
        Point {
            coords: std::array::from_fn(|i| self.coords[i] + rhs.coords[i])
        }
    }
}

impl<const D: usize, N: Num + Copy> Sub for Point<D, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let coords = std::array::from_fn(|i| {
            self.coords[i] - rhs.coords[i]
        });
        Point { coords }
    }
}

impl<'a, 'b, const D: usize, N: Num + Copy> Sub<&'b Point<D, N>> for &'a Point<D, N> {
    type Output = Point<D, N>;

    fn sub(self, rhs: &'b Point<D, N>) -> Self::Output {
        Point {
            coords: std::array::from_fn(|i| self.coords[i] - rhs.coords[i])
        }
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
