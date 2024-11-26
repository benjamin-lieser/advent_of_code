use std::collections::HashMap;

use advent_of_code::*;

fn to_t(s: &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!(),
    }
}

struct Rule<'a> {
    dst: &'a str,
    cond: Box<dyn Fn(PartRange) -> (PartRange, PartRange)>,
}

impl<'a> Rule<'a> {
    fn new(s: &'a str) -> Self {
        if let Some((rule, dst)) = s.split_once(':') {
            if let Some((a, n)) = rule.split_once('<') {
                let t = to_t(a);
                let n = n.parse::<u32>().unwrap();

                Rule {
                    dst,
                    cond: Box::new(move |p| {
                        if p.val[t].1 <= n {
                            (p, PartRange::empty())
                        } else if p.val[t].0 >= n {
                            (PartRange::empty(), p)
                        } else {
                            let mut pass = p;
                            let mut fail = p;

                            pass.val[t].1 = n;

                            fail.val[t].0 = n;

                            (pass, fail)
                        }
                    }),
                }
            } else if let Some((a, n)) = rule.split_once('>') {
                let t = to_t(a);
                let n = n.parse::<u32>().unwrap();
                Rule {
                    dst,
                    cond: Box::new(move |p| {
                        if p.val[t].0 > n {
                            (p, PartRange::empty())
                        } else if p.val[t].1 - 1 <= n {
                            (PartRange::empty(), p)
                        } else {
                            let mut pass = p;
                            let mut fail = p;

                            pass.val[t].0 = n + 1;

                            fail.val[t].1 = n + 1;

                            (pass, fail)
                        }
                    }),
                }
            } else {
                panic!();
            }
        } else {
            Rule {
                dst: s,
                cond: Box::new(|p| (p, PartRange::empty())),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartRange {
    val: [(u32, u32); 4],
}

impl PartRange {
    fn new() -> Self {
        PartRange {
            val: [(1, 4001); 4],
        }
    }

    fn empty() -> Self {
        PartRange { val: [(0, 0); 4] }
    }

    fn count(&self) -> usize {
        self.val.iter().map(|(a,b)| (b - a) as usize).product()
    }
}

fn do_work(p: PartRange, w: &str, step: usize, work: &HashMap<&str, Vec<Rule>>) -> usize {
    if p == PartRange::empty() {
        return 0;
    }

    let rule = &work.get(w).unwrap()[step];

    let (pass, fail) = (rule.cond)(p);

    if rule.dst == "R" {
        return do_work(fail, w, step + 1, work);
    } else if rule.dst == "A" {
        return do_work(fail, w, step + 1, work) + pass.count();
    } else {
        
        let a = do_work(pass, rule.dst, 0, work);
        let b = do_work(fail, w, step + 1, work);

        return a + b;
    }
}

fn main() {
    let input = get_input_aoc(19);
    //let input = std::fs::read_to_string("data/2023_19").unwrap();

    let (work, _) = input.split_once("\n\n").unwrap();

    let mut workflows = HashMap::<&str, Vec<Rule>>::new();

    for line in work.lines() {
        let (name, rem) = line.split_once('{').unwrap();
        let rem = &rem[0..rem.len() - 1];
        let workflow: Vec<_> = rem.split(',').map(|s| Rule::new(s)).collect();
        workflows.insert(name, workflow);
    }

    let acc = do_work(PartRange::new(), "in", 0, &workflows);

    print!("{}", acc);
}
