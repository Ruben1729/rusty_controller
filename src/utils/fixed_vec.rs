use core::ops::Index;
use core::marker::Copy;

pub struct FixedVec<T, const CAPACITY: usize> {
    array: [T; CAPACITY],
    length: usize
}

impl<T: Copy, const CAPACITY: usize> Index<usize> for FixedVec<T, CAPACITY> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.array[idx]
    }
}

impl<T: Copy + Default, const CAPACITY: usize> FixedVec<T, CAPACITY> {
    pub fn new() -> Self {
        Self { array: [T::default(); CAPACITY], length: 0 }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.length >= CAPACITY {
            Err("Array is full.")
        } else {
            self.array[self.length] = item;
            self.length += 1;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.length == 0 {
            Err("Array is empty.")
        } else {
            self.length -= 1;
            Ok(self.array[self.length])
        }
    }
}

#[test_case]
fn test_push_and_pop() {
    let mut vec: FixedVec<i32, 5> = FixedVec::new();
    assert_eq!(vec.push(1), Ok(()));
    assert_eq!(vec.push(2), Ok(()));
    assert_eq!(vec.pop(), Ok(2));
    assert_eq!(vec.pop(), Ok(1));
    assert_eq!(vec.pop().is_err(), true);
}

#[test_case]
fn test_index() {
    let mut vec: FixedVec<i32, 5> = FixedVec::new();
    vec.push(10).unwrap();
    vec.push(43).unwrap();
    vec.push(12).unwrap();
    assert_eq!(vec[0], 10);
    assert_eq!(vec[1], 43);
    assert_eq!(vec[2], 12);
}
