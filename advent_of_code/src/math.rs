use super::int;

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

pub fn lin_sol(mat : [[int;2];2], b : [int;2]) -> Option<[f64;2]> {
    let det = mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0];
    if det == 0 {
        return None;
    }
    let det = det as f64;
    let inv_det = 1.0 / det;
    let x = (b[0] as f64 * mat[1][1] as f64 - b[1] as f64 * mat[0][1] as f64) * inv_det;
    let y = (b[1] as f64 * mat[0][0] as f64 - b[0] as f64 * mat[1][0] as f64) * inv_det;
    Some([x,y])
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