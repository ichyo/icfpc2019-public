use crate::models::Point;

pub struct Matrix<T: Copy> {
    width: usize,
    inner: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(width: usize, height: usize, init: T) -> Matrix<T> {
        let n = width * height;
        Matrix {
            width,
            inner: vec![init; n],
        }
    }

    pub fn get(&self, p: &Point) -> T {
        self.inner[p.y * self.width + p.x]
    }

    pub fn get_mut(&mut self, p: &Point) -> &mut T {
        &mut self.inner[p.y * self.width + p.x]
    }

    pub fn set(&mut self, p: &Point, value: T) {
        *self.get_mut(p) = value
    }
}