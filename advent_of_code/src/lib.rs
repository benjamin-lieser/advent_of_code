use std::cell::UnsafeCell;
use std::hash::Hash;

pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

mod aoc_tooling;
pub use aoc_tooling::*;

mod parsing;
pub use parsing::*;

mod math;
pub use math::*;

pub use itertools::Itertools;

mod dir;
pub use dir::*;

pub use petgraph::graphmap::DiGraphMap;
pub use petgraph::graphmap::UnGraphMap;
pub use petgraph;

#[allow(non_camel_case_types)]
pub type int = isize;

pub struct Indexer {
    map: HashMap<String, usize>,
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> usize {
        let len = self.map.len();
        *self.map.entry(key.to_string()).or_insert(len)
    }
}

pub fn transpose<T: Default + Copy, S: AsRef<[T]>>(array: &[S]) -> Vec<Vec<T>> {
    let row_length = array[0].as_ref().len();
    let col_length = array.len();
    let mut transpose = vec![vec![T::default(); col_length]; row_length];
    for r in 0..row_length {
        for c in 0..col_length {
            transpose[r][c] = array[c].as_ref()[r];
        }
    }
    transpose
}

pub fn bits_to_int(bits: &[bool]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| if b { 1 << i } else { 0 })
        .sum()
}

pub fn int_to_bits(mut n: u64, length: usize) -> Vec<bool> {
    let mut bits = vec![false; length];
    for i in 0..length {
        bits[i] = (n & 1) == 1;
        n >>= 1;
    }
    bits
}

pub fn cycle_detection<T: Eq + Hash + Clone, F: Fn(T) -> T>(mut init: T, f: F) -> (usize, usize) {
    let mut memory = HashMap::<T, usize>::new();

    memory.insert(init.clone(), 0);
    let mut counter = 0;
    loop {
        counter += 1;
        let next = f(init.clone());
        if let Some(&find) = memory.get(&next) {
            return (find, counter - find);
        }
        memory.insert(next.clone(), counter);
        init = next;
    }
}

pub fn apply_n_times<T>(f: impl Fn(T) -> T, n: usize) -> impl Fn(T) -> T {
    move |arg| (0..n).fold(arg, |a, _| f(a))
}

pub fn index<T: Copy, R: AsRef<[T]>>(grid: &[R], index: (isize, isize)) -> Option<T> {
    let r = grid.as_ref().get(index.1 as usize)?;

    r.as_ref().get(index.0 as usize).copied()
}

pub fn index_mut<'a, T, R: AsMut<[T]> + 'a>(
    grid: &'a mut [R],
    index: (isize, isize),
) -> Option<&'a mut T> {
    let r = grid.as_mut().get_mut(index.1 as usize)?;
    r.as_mut().get_mut(index.0 as usize)
}

pub struct Global<T> {
    inner: UnsafeCell<Option<T>>,
}

impl<T> Global<T> {
    pub const fn new() -> Self {
        Global {
            inner: UnsafeCell::new(None),
        }
    }

    /// Not actually safe, only use when there are no borrows to this
    pub fn set(&self, value: T) {
        unsafe {
            *self.inner.get() = Some(value);
        }
    }

    /// Not actually safe, only use if there are no mutable borrows to this
    pub fn borrow(&self) -> &T {
        unsafe { (&*self.inner.get()).as_ref().unwrap() }
    }

    /// Not actually safe, only use if there are no borrows to this
    pub fn borrow_mut(&self) -> &mut T {
        unsafe { (&mut *self.inner.get()).as_mut().unwrap() }
    }
}

unsafe impl<T> Send for Global<T> {} // Not actually safe, but we are single threaded
unsafe impl<T> Sync for Global<T> {} // Not actually safe, but we are single threaded

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_detection() {
        let f = |x: u8| x.wrapping_add(1);

        assert_eq!(cycle_detection(128, f), (0, 256));
    }

    #[test]
    fn grid_mut() {
        let mut test = vec![vec![5; 10]; 20];

        assert_eq!(index_mut(&mut test, (21, 0)), None);

        assert_eq!(index_mut(&mut test, (0, 0)), Some(&mut 5));

        if let Some(a) = index_mut(&mut test, (1, 1)) {
            *a = 7;
        } else {
            panic!();
        }

        assert_eq!(index(&test, (1, 1)), Some(7));
    }
}
