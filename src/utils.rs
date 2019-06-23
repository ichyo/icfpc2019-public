use crate::models::Point;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    inner: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(width: usize, height: usize, init: T) -> Matrix<T> {
        let n = width * height;
        Matrix {
            width,
            height,
            inner: vec![init; n],
        }
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        if p.x >= 0 && p.y >= 0 && (p.x as usize) < self.width && (p.y as usize) < self.height {
            Some(&self.inner[p.y as usize * self.width + p.x as usize])
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
            Some(std::mem::replace(r, value))
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

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Range {
        Range { start, end }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }
    pub fn contains(&self, x: usize) -> bool {
        self.start <= x && x < self.end
    }
    pub fn contains_all(&self, xs: &[usize]) -> bool {
        xs.iter().all(|x| self.contains(*x))
    }
    pub fn intersect(&self, other: Range) -> bool {
        !(self.end <= other.start || other.end <= self.start)
    }
    pub fn split(&self, mid: usize) -> (Range, Range) {
        assert!(self.contains(mid));
        let left = Range::new(self.start, mid);
        let right = Range::new(mid + 1, self.end);
        (left, right)
    }
    pub fn remove_begin(&self) -> Range {
        Range::new(self.start+1, self.end)
    }
    pub fn remove_end(&self) -> Range {
        Range::new(self.start, self.end - 1)
    }
}