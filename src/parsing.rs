use std::fmt::Debug;
use std::str::FromStr;

pub fn split<const N: usize, T: FromStr + Clone>(input: &str, del: &str) -> [T; N]
where
    T: Debug,
    T::Err: Debug,
{
    let token: Vec<T> = if del == "" {
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
