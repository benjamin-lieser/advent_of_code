use std::{fmt::Debug, hash::Hash, ops::Neg, str::FromStr};

use num::Integer;

pub use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet};

mod aoc_tooling;
pub use aoc_tooling::{get_input, get_input_inf};

mod parsing;
pub use parsing::{split, split_s, SplitOnce, SCast};

mod math;
pub use math::{lcm, gcd};
pub use math::{ElementMax, ElementMin};

#[allow(non_camel_case_types)]
pub type int = isize;

pub type Pos = (isize, isize);

pub const END: Pos = (isize::MAX, isize::MAX);
pub const START: Pos = (isize::MIN, isize::MIN);

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

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

pub struct DirMul<N>((N, N));

pub const DIRS: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];

impl<N: Neg<Output = N> + Integer> std::ops::Add<Dir> for (N, N) {
    type Output = (N, N);

    fn add(self, rhs: Dir) -> Self::Output {
        let (rd, cd) = rhs.d();
        (self.0 + rd, self.1 + cd)
    }
}

impl<N: Neg<Output = N> + Integer> std::ops::Add<DirMul<N>> for (N, N) {
    type Output = (N, N);

    fn add(self, rhs: DirMul<N>) -> Self::Output {
        let (rd, cd) = rhs.0;
        (self.0 + rd, self.1 + cd)
    }
}

impl std::ops::Mul<Dir> for isize {
    type Output = DirMul<isize>;

    fn mul(self, rhs: Dir) -> Self::Output {
        let (rd, cd) = rhs.d::<isize>();
        DirMul((self * rd, self * cd))
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::Right),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "U" => Ok(Dir::Up),
            _ => Err(()),
        }
    }
}

impl Dir {
    pub fn d<N: Neg<Output = N> + Integer>(&self) -> (N, N) {
        let zero = N::zero();
        let one = N::one();

        match self {
            Dir::Right => (one, zero),
            Dir::Down => (zero, one),
            Dir::Left => (-one, zero),
            Dir::Up => (zero, -one),
        }
    }

    pub fn turn_lr(&self) -> [Self; 2] {
        match self {
            Dir::Right => [Dir::Down, Dir::Up],
            Dir::Down => [Dir::Left, Dir::Right],
            Dir::Left => [Dir::Up, Dir::Down],
            Dir::Up => [Dir::Right, Dir::Left],
        }
    }

    pub fn turn_l(&self) -> Self {
        match self {
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
        }
    }
    pub fn turn_r(&self) -> Self {
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }

    pub fn opp(&self) -> Self {
        match self {
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Left,
        }
    }
}

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
