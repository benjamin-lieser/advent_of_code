use std::collections::{HashMap, VecDeque};

use advent_of_code::*;

struct FlipFlop<'a> {
    state: bool,
    next: Vec<&'a str>,
}

struct And<'a> {
    state: HashMap<&'a str, bool>,
    next: Vec<&'a str>,
}

impl<'a> FlipFlop<'a> {
    fn new(out: Vec<&'a str>) -> Self {
        FlipFlop {
            state: false,
            next: out,
        }
    }
}

impl<'a> And<'a> {
    fn new(out: Vec<&'a str>) -> Self {
        And {
            state: HashMap::new(),
            next: out,
        }
    }

    fn register(&mut self, name: &'a str) {
        self.state.insert(name, false);
    }
}

trait Gate: Sized {
    fn send(&mut self, from: &str, high: bool) -> Vec<(&'static str, bool)>;
}

impl<'a: 'static> Gate for FlipFlop<'a> {
    fn send(&mut self, _: &str, high: bool) -> Vec<(&'static str, bool)> {
        if high {
            vec![]
        } else {
            self.state = !self.state;
            self.next.iter().map(|&n| (n, self.state)).collect()
        }
    }
}
impl<'a: 'static> Gate for And<'a> {
    fn send(&mut self, from: &str, high: bool) -> Vec<(&'static str, bool)> {
        *self.state.get_mut(from).unwrap() = high;

        if self.state.values().all(|x| *x) {
            self.next.iter().map(|&n| (n, false)).collect()
        } else {
            self.next.iter().map(|&n| (n, true)).collect()
        }
    }
}

enum G<'a> {
    F(FlipFlop<'a>),
    A(And<'a>),
}

impl<'a: 'static> G<'a> {
    fn send(&mut self, from: &str, high: bool) -> Vec<(&'static str, bool)> {
        match self {
            G::F(i) => i.send(from, high),
            G::A(i) => i.send(from, high),
        }
    }
    fn register(&mut self, name: &'a str) {
        match self {
            G::F(_) => {}
            G::A(i) => i.register(name),
        }
    }
}

fn main() {
    let input = get_input(2023, 20);

    let mut gates = HashMap::<&'static str, G>::new();

    let mut broadcast = vec![];

    for line in input.lines() {
        let line = Box::new(line.to_owned()).leak();

        let (name, out) = line.split_once("->").unwrap();

        let out: Vec<&str> = out.split(',').map(str::trim).collect();

        match name.as_bytes()[0] {
            b'%' => {
                let n = &name.trim()[1..];
                gates.insert(n, G::F(FlipFlop::new(out)));
            }
            b'&' => {
                let n = &name.trim()[1..];
                gates.insert(n, G::A(And::new(out)));
            }
            b'b' => {
                broadcast = out;
            }

            _ => panic!(),
        }
    }

    for line in input.lines() {
        let line = Box::new(line.to_owned()).leak();
        let (name, out) = line.split_once("->").unwrap();

        let out: Vec<&str> = out.split(',').map(str::trim).collect();
        let name = &name.trim()[1..];

        for next in out.iter() {
            if let Some(node) = gates.get_mut(next) {
                node.register(name);
            }
        }
    }

    let mut low_counter = 0usize;
    let mut hight_counter = 0usize;

    for _press in 0..1000 {
        low_counter += 1;
        let mut q = VecDeque::<(&'static str, &'static str, bool)>::new();

        for b in broadcast.iter() {
            q.push_back(("broadcaster", b, false));
            low_counter += 1;
        }

        while !q.is_empty() {
            let (from, to, high) = q.pop_front().unwrap();
            if let Some(node) = gates.get_mut(to) {
                let new = node.send(from, high);
                for (node, high) in new.iter() {
                    if *high {
                        hight_counter += 1;
                    } else {
                        low_counter += 1;
                    }

                    q.push_back((to, *node, *high));
                }
            }
        }
    }

    println!("{}", low_counter * hight_counter);
}
