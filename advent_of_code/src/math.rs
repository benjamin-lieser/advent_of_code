pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub trait ElementMax {
    type Out;
    fn max_elem(&self, b: &Self) -> Self::Out;
}

impl<const N :usize, T : Ord + Copy> ElementMax for [T;N] {
    type Out = Self;
    fn max_elem(&self , b: &Self) -> Self {
        std::array::from_fn(|idx| self[idx].max(b[idx]))
    }
}

impl<T : Ord + Copy> ElementMax for [T] {
    type Out = Vec<T>;
    fn max_elem(&self, b: &Self) -> Vec<T> {
        assert!(self.len() == b.len());
        self.iter().zip(b.iter()).map(|(a,b)| *a.max(b)).collect()
    }
}

pub trait ElementMin {
    type Out;
    fn min_elem(&self, b: &Self) -> Self::Out;
}

impl<const N :usize, T : Ord + Copy> ElementMin for [T;N] {
    type Out = Self;
    fn min_elem(&self , b: &Self) -> Self {
        std::array::from_fn(|idx| self[idx].min(b[idx]))
    }
}
impl<T : Ord + Copy> ElementMin for [T] {
    type Out = Vec<T>;
    fn min_elem(&self, b: &Self) -> Vec<T> {
        assert!(self.len() == b.len());
        self.iter().zip(b.iter()).map(|(a,b)| *a.min(b)).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::ElementMax;

    #[test]
    fn max_element() {
        let a = vec![1,2,3];
        let b = vec![3,2,1];

        assert_eq!(a.max_elem(&b), vec![3,2,3]);
    }
}