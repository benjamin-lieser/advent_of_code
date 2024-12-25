use std::str::FromStr;
use super::int;

pub fn split<const N: usize>(input: &str, del: &str) -> [int; N]
{
    let token: Vec<int> = if del == "" {
        input
            .split_ascii_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect()
    } else {
        input
            .split(del)
            .map(|x| x.trim().parse().unwrap())
            .collect()
    };
    let token = token[..N].to_vec();
    token.try_into().unwrap()
}
pub fn split_s<'a, const N: usize>(input: &'a str, del: &str) -> [&'a str; N] {
    let token: Vec<&str> = if del == "" {
        input.split_ascii_whitespace().map(|x| x.trim()).collect()
    } else {
        input.split(del).map(|x| x.trim()).collect()
    };
    let token = token[..N].to_vec();
    token.try_into().unwrap()
}

pub fn read_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}
pub fn read_grid_int(input: &str) -> Vec<Vec<int>> {
    input.lines().map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as int).collect()).collect()
}

pub fn split_empty_line(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

pub fn split_empty_line_static<const N: usize>(input: &str) -> [&str; N] {
    let parts = split_empty_line(input);
    parts.try_into().unwrap()
}

pub fn get_all_int<const N : usize>(input: &str) -> [int;N] {
    let int_regex = regex::Regex::new(r"-?\d+").unwrap();
    let matches : Vec<int> = int_regex.find_iter(input).map(|x| x.as_str().parse().unwrap()).collect();
    matches.try_into().unwrap()
}

pub fn get_all_pos_int<const N : usize>(input: &str) -> [int;N] {
    let int_regex = regex::Regex::new(r"\d+").unwrap();
    let matches : Vec<int> = int_regex.find_iter(input).map(|x| x.as_str().parse().unwrap()).collect();
    matches.try_into().unwrap()
}

pub fn get_all_matches<'a>(input: &'a str, re: &str) -> Vec<&'a str> {
    let re = regex::Regex::new(re).unwrap();
    re.find_iter(input).map(|x| x.as_str()).collect()
}


pub trait SplitOnce {
    fn so(&self, d: &str) -> (&str, &str);
}

impl SplitOnce for str {
    fn so(&self, d: &str) -> (&str, &str) {
        let (a, b) = self.split_once(d).unwrap();
        (a.trim(), b.trim())
    }
}

pub trait SCast {
    fn scast<T: FromStr>(&self, del: &str) -> Vec<T>
    where
        T::Err : std::fmt::Debug
    ;
}

impl SCast for str {
    fn scast<T: FromStr>(&self, del: &str) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        if del == "" {
            self.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        } else {
            self.split(del).map(|x| x.trim().parse().unwrap()).collect()
        }
    }
}
