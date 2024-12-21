use std::{str::FromStr, vec};

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
pub const DIRS_DIAG: [DirDiag; 8] = [
    DirDiag::Right,
    DirDiag::Down,
    DirDiag::Left,
    DirDiag::Up,
    DirDiag::RightDown,
    DirDiag::LeftDown,
    DirDiag::LeftUp,
    DirDiag::RightUp,
];

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

pub struct Step(int, int);

impl std::ops::Mul<Dir> for int {
    type Output = Step;

    fn mul(self, rhs: Dir) -> Self::Output {
        let (rd, cd) = rhs.step();
        Step(self * rd, self * cd)
    }
}
impl std::ops::Mul<DirDiag> for int {
    type Output = Step;

    fn mul(self, rhs: DirDiag) -> Self::Output {
        let (rd, cd) = rhs.step();
        Step(self * rd, self * cd)
    }
}

impl std::ops::Add<Step> for Pos {
    type Output = Pos;

    fn add(self, rhs: Step) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        let (rd, cd) = rhs.step();
        (self.0 + rd, self.1 + cd)
    }
}

impl std::ops::Add<DirDiag> for Pos {
    type Output = Pos;

    fn add(self, rhs: DirDiag) -> Self::Output {
        let (rd, cd) = rhs.step();
        (self.0 + rd, self.1 + cd)
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

    pub fn from_char2(c: char) -> Option<Self> {
        match c {
            '>' => Some(Dir::Right),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            '^' => Some(Dir::Up),
            _ => None,
        }
    }

    pub fn to_char2(&self) -> char {
        match self {
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Up => '^',
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
            Dir::Up => Dir::Down,
        }
    }
}

pub fn get_dirs(start: Pos, end: Pos) -> (Option<Dir>, Option<Dir>) {
    let x = match start.0.cmp(&end.0) {
        std::cmp::Ordering::Less => Some(Dir::Right),
        std::cmp::Ordering::Greater => Some(Dir::Left),
        std::cmp::Ordering::Equal => None
    };

    let y = match start.1.cmp(&end.1) {
        std::cmp::Ordering::Less => Some(Dir::Down),
        std::cmp::Ordering::Greater => Some(Dir::Up),
        std::cmp::Ordering::Equal => None
    };
    
    (x, y)
}

pub fn get_path(start: Pos, end: Pos) -> Vec<Dir> {
    let mut path = vec![];
    if start.0 < end.0 {
        path.extend(std::iter::repeat(Dir::Right).take((end.0 - start.0) as usize));
    } else if start.0 > end.0 {
        path.extend(std::iter::repeat(Dir::Left).take((start.0 - end.0) as usize));
    }
    if start.1 < end.1 {
        path.extend(std::iter::repeat(Dir::Down).take((end.1 - start.1) as usize));
    } else if start.1 > end.1 {
        path.extend(std::iter::repeat(Dir::Up).take((start.1 - end.1) as usize));
    }
    path
}

pub fn manhattan(p1: Pos, p2: Pos) -> int {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
}

impl Grid<char> {
    pub fn from_str(s: &str) -> Self {
        let grid = s.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn as_str(&self) -> String {
        self.grid.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("")
    }

    pub fn to_int(&self) -> Grid<int> {
        Grid::new(self.grid.iter().map(|row| row.iter().map(|&c| c.to_digit(10).unwrap() as int).collect()).collect())
    }
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid }
    }
    pub fn r(&self) -> int {
        self.grid.len() as int
    }

    pub fn c(&self) -> int {
        self.grid[0].len() as int
    }

    pub fn positions(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.r()).flat_map(move |r| (0..self.c()).map(move |c| (c, r)))
    }
}

impl<T: Copy + Eq> Grid<T> {
    pub fn find(&self, value: T) -> Option<Pos> {
        self.positions().find(|&pos| self.get(pos) == Some(value))
    }

    pub fn swap(&mut self, pos1: Pos, pos2: Pos) {
        let tmp = self.get(pos1).unwrap();
        self.set(pos1, self.get(pos2).unwrap());
        self.set(pos2, tmp);
    }

    pub fn full(rows: int, cols: int, value: T) -> Self {
        Self {
            grid: vec![vec![value; rows as usize]; cols as usize],
        }
    }

    pub fn full_like<A>(grid: &Grid<A>, value: T) -> Self {
        Self::full(grid.r(), grid.c(), value)
    }


    pub fn get(&self, pos: Pos) -> Option<T> {
        let r = self.grid.get(pos.1 as usize)?;
        r.get(pos.0 as usize).copied()
    }

    pub fn set(&mut self, pos: Pos, value: T) {
        self.grid[pos.1 as usize][pos.0 as usize] = value;
    }

    pub fn try_set(&mut self, pos: Pos, value: T) -> bool {
        if let Some(x) = self.get_mut(pos) {
            *x = value;
            true
        } else {
            false
        }
    }

    pub fn count(&self, value: T) -> int {
        self.grid.iter().flatten().filter(|&&x| x == value).count() as int
    }

    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        let r = self.grid.get_mut(pos.1 as usize)?;
        r.get_mut(pos.0 as usize)
    }
}

pub fn clusters(grid: &Grid<char>) -> (Grid<int>, int) {
    let mut visited = Grid::full_like(grid, false);
    let mut clusters = Grid::full_like(grid, -1);

    let mut cluster = 0;

    for r in 0..grid.r() {
        for c in 0..grid.c() {
            if visited.get((c, r)).unwrap() {
                continue;
            }
            let mut stack = vec![(c, r)];
            while let Some((c, r)) = stack.pop() {
                if visited.get((c, r)).unwrap() {
                    continue;
                }
                visited.set((c, r), true);
                clusters.set((c, r), cluster);
                for dir in DIRS {
                    let next = (c, r) + dir;
                    if grid.get(next) == Some(grid.get((c, r)).unwrap()) {
                        stack.push(next);
                    }
                }
            }
            cluster += 1;
        }
    }
    (clusters, cluster)
}
