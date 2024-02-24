pub trait StoogeExt {
    fn stooge_sort(&mut self);
}

impl<T> StoogeExt for Vec<T> where T: Ord {
    fn stooge_sort(&mut self) {
        _stooge_sort(self, 0, self.len() - 1);
    }
}

fn _stooge_sort<T>(v: &mut Vec<T>, left: usize, right: usize)
where T: Ord {
    if v[left] > v[right] {
        v.swap(left, right);
    }

    if (right - left + 1) > 2 {
        let third = ((right as f64 - left as f64 + 1.0) / 3.0).floor() as usize;
        _stooge_sort(v, left, right - third);
        _stooge_sort(v, left + third, right);
        _stooge_sort(v, left, right - third);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, Rng};

    #[test]
    fn trivial() {
        let mut test = [1, 0].to_vec();
        test.stooge_sort();
        assert_eq!(test, [0, 1]);
    }

    #[test]
    fn sort_example() {
        let mut test = [-5, 4, 1, -3, 2].to_vec();
        test.stooge_sort();
        assert_eq!(test, [-5, -3, 1, 2, 4]);
    }

    #[test]
    fn random50() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-100, 100);
        let mut test: Vec<i64> = (1..=50).map(|_| rng.sample(&range)).collect();
        let mut reference = test.clone();

        test.stooge_sort();
        reference.sort();

        assert_eq!(test, reference);
    }
}
