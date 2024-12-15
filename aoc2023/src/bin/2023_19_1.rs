use std::collections::HashMap;

use advent_of_code::*;

fn to_t(s : &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!()
    }
}

struct Rule<'a> {
    dst : &'a str,
    cond : Box<dyn Fn(Part) -> bool>
}

impl<'a> Rule<'a> {
    fn new(s: &'a str) -> Self {
        if let Some((rule, dst)) = s.split_once(':') {

            if let Some((a,n)) = rule.split_once('<') {
                let t = to_t(a);
                let n = n.parse().unwrap();
                Rule{dst: dst, cond : Box::new(move |p| p.val[t] < n)}
            } else if let Some((a,n)) = rule.split_once('>') {
                let t = to_t(a);
                let n = n.parse().unwrap();
                Rule{dst: dst, cond : Box::new(move |p| p.val[t] > n)}
            } else {
                panic!();
            }

        } else {
            Rule{dst: s, cond : Box::new(|_| true)}
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    val: [u32;4]
}

impl Part {
    fn new(s: &str) -> Self {
        let re = regex::Regex::new(r"\{x=([\d]+),m=([\d]+),a=([\d]+),s=([\d]+)\}").unwrap();
        let captures = re.captures(s).unwrap();
        Part{val : [captures[1].parse().unwrap(), captures[2].parse().unwrap(), captures[3].parse().unwrap(), captures[4].parse().unwrap()]}
    }
}

fn do_work(p: Part, w : &str, work: &HashMap<&str, Vec<Rule>>) -> bool {
    let workflow = work.get(w).unwrap();

    for rule in workflow {
        if (rule.cond)(p) {
            if rule.dst == "R" {
                return false;
            } else if rule.dst == "A" {
                return true;
            } else {
                return do_work(p, rule.dst, work);
            }
        }
    }

    panic!()
}

fn main() {
    let input = get_input(2023, 19);

    let (work, parts) = input.split_once("\n\n").unwrap();

    let mut workflows = HashMap::<&str, Vec<Rule>>::new();

    for line in work.lines() {
        let (name, rem) = line.split_once('{').unwrap();
        let rem = &rem[0..rem.len()-1];
        let workflow : Vec<_> = rem.split(',').map(|s| Rule::new(s)).collect();
        workflows.insert(name, workflow);
    }

    let parts: Vec<Part> = parts.lines().map(|l| Part::new(l)).collect();
    
    let mut acc = 0usize;

    for p in parts.iter() {
        if do_work(*p, "in", &workflows) {
            acc += p.val.iter().map(|&x| x as usize).sum::<usize>();
        }
    }

    print!("{}", acc);

}
