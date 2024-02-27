use std::cmp::Ordering;

pub trait StoogeOrd<T: Ord> {
    fn stooge_sort(&mut self);
}

pub trait Stooge<T> {
    fn stooge_sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering;
}

impl<T: Ord> StoogeOrd<T> for [T] {
    fn stooge_sort(&mut self) {
        if self.is_empty() || self.len() == 1 {
        } else {
            stooge_sort(self, 0, self.len() - 1, &mut T::lt);
        }
    }
}

impl<T> Stooge<T> for [T] {
    fn stooge_sort_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        if self.is_empty() || self.len() == 1 {
        } else {
            stooge_sort(self, 0, self.len() - 1, &mut |a, b| {
                compare(a, b) == Ordering::Less
            });
        }
    }
}

fn stooge_sort<T, F>(v: &mut [T], left: usize, right: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if !is_less(&v[left], &v[right]) {
        v.swap(left, right);
    }

    if (right - left + 1) > 2 {
        let third = (right - left + 1) / 3;
        stooge_sort(v, left, right - third, is_less);
        stooge_sort(v, left + third, right, is_less);
        stooge_sort(v, left, right - third, is_less);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, Rng};

    #[test]
    fn no_elements() {
        let mut test: Vec<u8> = [].to_vec();
        test.stooge_sort();
        assert_eq!(test, []);
    }

    #[test]
    fn one_element() {
        let mut test: Vec<u16> = [8580].to_vec();
        test.stooge_sort();
        assert_eq!(test, [8580])
    }

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

    #[test]
    fn random1000() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-100, 100);
        let mut test: Vec<i64> = (1..=1000).map(|_| rng.sample(&range)).collect();
        let mut reference = test.clone();

        test.stooge_sort();
        reference.sort();

        assert_eq!(test, reference);
    }

    #[test]
    fn sort_floats() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(-1000.0, 1000.0);
        let mut test: Vec<f64> = (1..=100).map(|_| rng.sample(&range)).collect();
        let mut reference: Vec<f64> = test.clone();

        test.stooge_sort_by(|a, b| a.partial_cmp(b).unwrap());
        reference.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(test, reference);
    }
}
