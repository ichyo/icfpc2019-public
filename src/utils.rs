use crate::models::Point;

pub struct Matrix<T: Copy> {
    width: usize,
    height: usize,
    inner: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(width: usize, height: usize, init: T) -> Matrix<T> {
        let n = width * height;
        Matrix {
            width,
            height,
            inner: vec![init; n],
        }
    }

    pub fn get(&self, p: Point) -> Option<T> {
        if p.x >= 0 && p.y >= 0 && (p.x as usize) < self.width && (p.y as usize) < self.height {
            Some(self.inner[p.y as usize * self.width + p.x as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if p.x >= 0 && p.y >= 0 && (p.x as usize) < self.width && (p.y as usize) < self.height {
            Some(&mut self.inner[p.y as usize * self.width + p.x as usize])
        } else {
            None
        }
    }

    pub fn try_set(&mut self, p: Point, value: T) -> Option<T> {
        if let Some(r) = self.get_mut(p) {
            let old = *r;
            *r = value;
            Some(old)
        } else {
            None
        }
    }

    pub fn set(&mut self, p: Point, value: T) {
        if let Some(r) = self.get_mut(p) {
            *r = value;
        } else {
            panic!("out of bound : {:?}", p);
        }
    }
}