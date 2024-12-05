use std::{ops::Neg, str::FromStr};

use num::Integer;

use super::int;

pub type Pos = (int, int);

pub const END: Pos = (int::MAX, int::MAX);
pub const START: Pos = (int::MIN, int::MIN);

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}
pub const DIRS: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum DirDiag {
    Right,
    Down,
    Left,
    Up,
    RightDown,
    LeftDown,
    LeftUp,
    RightUp,
}
pub const DIRS_DIAG : [DirDiag;8] = [DirDiag::Right, DirDiag::Down, DirDiag::Left, DirDiag::Up, DirDiag::RightDown, DirDiag::LeftDown, DirDiag::LeftUp, DirDiag::RightUp];

impl DirDiag {
    pub fn step(&self) -> (int, int) {
        match self {
            DirDiag::Right => (1, 0),
            DirDiag::Down => (0, 1),
            DirDiag::Left => (-1, 0),
            DirDiag::Up => (0, -1),
            DirDiag::RightDown => (1, 1),
            DirDiag::LeftDown => (-1, 1),
            DirDiag::LeftUp => (-1, -1),
            DirDiag::RightUp => (1, -1),
        }
    }
}

pub struct DirMul<N>((N, N));


impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        let (rd, cd) = rhs.step();
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
        let (rd, cd) = rhs.step();
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
    pub fn step(&self) -> (int, int) {
        match self {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
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