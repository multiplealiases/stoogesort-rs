#![doc = include_str!("../README.md")]
use std::cmp::Ordering;

pub trait Stooge<T> {
    /// Sorts the slice using stooge sort.
    ///
    /// This sort is unstable, has worst-case
    /// time complexity of O(n^(log(3)/log(1.5)))
    /// ≈ O(n^2.7095), and recurses at most n levels deep.
    ///
    /// ```
    /// use stoogesort::Stooge;
    /// let mut v = [-5, 4, 1, -3, 2];
    ///
    /// v.stooge_sort();
    /// assert!(v == [-5, -3, 1, 2, 4]);
    /// ```
        fn stooge_sort(&mut self)
    where
        T: Ord;
    /// Sorts the slice using stooge sort with a comparator function.
    ///
    /// This sort is unstable, has worst-case
    /// time complexity of O(n^(log(3)/log(1.5)))
    /// ≈ O(n^2.7095), and recurses at most n levels deep.
    ///
    /// The comparator function must define a total ordering for the elements in the slice. If
    /// the ordering is not total, the order of the elements is unspecified. An order is a
    /// total order if it is (for all `a`, `b` and `c`):
    ///
    /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
    /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
    ///
    /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
    /// `partial_cmp` as our sort function when we know the slice doesn't contain a `NaN`.
    ///
    /// ```
    /// use stoogesort::Stooge;
    /// let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
    /// floats.stooge_sort_by(|a, b| a.partial_cmp(b).unwrap());
    /// assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
    /// ```
    fn stooge_sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering;
    /// Sorts the slice using stooge sort with a key extraction function.
    ///
    /// This sort is unstable, has worst-case
    /// time complexity of O(n^(log(3)/log(1.5)) * m)
    /// ≈ O(n^2.7095 * m), where the key function is O(m),
    /// and recurses at most n levels deep.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [-5i32, 4, 1, -3, 2];
    ///
    /// v.sort_by_key(|k| k.abs());
    /// assert!(v == [1, 2, -3, 4, -5]);
    /// ```
    fn stooge_sort_by_key<F, K>(&mut self, compare: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;
}

impl<T> Stooge<T> for [T] {
    fn stooge_sort(&mut self)
    where
        T: Ord,
    {
        if self.is_empty() || self.len() == 1 {
        } else {
            stooge_sort(self, 0, self.len() - 1, &mut T::lt);
        }
    }
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
    fn stooge_sort_by_key<F, K>(&mut self, mut compare: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        if self.is_empty() || self.len() == 1 {
        } else {
            stooge_sort(self, 0, self.len() - 1, &mut |a, b| {
                compare(a).lt(&compare(b))
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

    #[test]
    fn vec_of_char() {
        let mut test: Vec<char> = "1312".chars().collect();
        test.sort();
        assert_eq!(test, "1123".chars().collect::<Vec<char>>());
    }

    #[test]
    fn vec_of_str() {
        let test: &mut [&str] = &mut ["6502", "2650", "680x0", "Z80"];
        test.stooge_sort();
        assert_eq!(test, ["2650", "6502", "680x0", "Z80"]);
    }

    #[test]
    fn big_iron() {
        let big_iron: String = "to the town of agua fria
        rode a stranger one fine day
        hardly spoke to folks around him
        didn't have too much to say
        no one dared to ask his business
        no one dared to make a slip
        the stranger there among them
        had a big iron on his hip
        big iron on his hip

        it was early in the morning
        when he rode into the town
        he came riding from the south side
        slowly looking all around
        he's an outlaw loose and running
        came the whisper from each lip
        and he's here to do some business
        with the big iron on his hip
        big iron on his hip

        in this town there lived an outlaw
        by the name of texas red
        many men had tried to take him
        and that many men were dead
        he was vicious and a killer
        though a youth of twenty-four
        and the notches on his pistol
        numbered one and nineteen more
        one and nineteen more

        now the stranger started talking
        made it plain to folks around
        was an arizona ranger
        wouldn't be too long in town
        he came here to take an outlaw
        back alive or maybe dead
        and he said it didn't matter
        he was after texas red
        after texas red

        wasn't long before the story
        was relayed to texas red
        but the outlaw didn't worry
        men that tried before were dead
        twenty men had tried to take him
        twenty men had made a slip
        twenty-one would be the ranger
        with the big iron on his hip
        big iron on his hip

        the morning passed so quickly
        it was time for them to meet
        it was twenty past eleven
        when they walked out in the street
        folks were watching from the windows
        everybody held their breath
        they knew this handsome ranger
        was about to meet his death
        'bout to meet his death

        there was forty feet between them
        when they stopped to make their play
        and the swiftness of the ranger
        is still talked about today
        texas red had not cleared leather
        'fore a bullet fairly ripped
        and the ranger's aim was deadly
        with the big iron on his hip
        big iron on his hip

        it was over in a moment
        and the folks had gathered round
        there before them lay the body
        of the outlaw on the ground
        oh he might have went on livin'
        but he made one fatal slip
        when he tried to match the ranger
        with the big iron on his hip
        big iron on his hip

        big iron big iron
        when he tried to match the ranger
        with the big iron on his hip
        big iron on his hip"
            .to_string();
        let mut words: Vec<String> = big_iron
            .replace("\n", "")
            .split(&[' ', '-'])
            .filter(|&x| !x.is_empty())
            .map(|s| s.to_owned())
            .collect();
        words.stooge_sort();
        assert_eq!(
            words,
            [
                "'bout",
                "'fore",
                "a",
                "a",
                "a",
                "a",
                "a",
                "a",
                "a",
                "a",
                "about",
                "about",
                "after",
                "after",
                "agua",
                "aim",
                "alive",
                "all",
                "among",
                "an",
                "an",
                "an",
                "an",
                "and",
                "and",
                "and",
                "and",
                "and",
                "and",
                "and",
                "and",
                "and",
                "and",
                "and",
                "arizona",
                "around",
                "around",
                "around",
                "ask",
                "back",
                "be",
                "be",
                "before",
                "before",
                "before",
                "between",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "big",
                "body",
                "breath",
                "bullet",
                "business",
                "business",
                "but",
                "but",
                "by",
                "came",
                "came",
                "came",
                "cleared",
                "dared",
                "dared",
                "day",
                "dead",
                "dead",
                "dead",
                "deadly",
                "death",
                "death",
                "didn't",
                "didn't",
                "didn't",
                "do",
                "each",
                "early",
                "eleven",
                "everybody",
                "fairly",
                "fatal",
                "feet",
                "fine",
                "folks",
                "folks",
                "folks",
                "folks",
                "for",
                "forty",
                "four",
                "fria",
                "from",
                "from",
                "from",
                "gathered",
                "ground",
                "had",
                "had",
                "had",
                "had",
                "had",
                "had",
                "handsome",
                "hardly",
                "have",
                "have",
                "he",
                "he",
                "he",
                "he",
                "he",
                "he",
                "he",
                "he",
                "he",
                "he",
                "he's",
                "he's",
                "held",
                "here",
                "here",
                "him",
                "him",
                "him",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "hip",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "his",
                "in",
                "in",
                "in",
                "in",
                "in",
                "into",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "iron",
                "is",
                "it",
                "it",
                "it",
                "it",
                "it",
                "it",
                "killer",
                "knew",
                "lay",
                "leather",
                "lip",
                "lived",
                "livin'",
                "long",
                "long",
                "looking",
                "loose",
                "made",
                "made",
                "made",
                "make",
                "make",
                "many",
                "many",
                "match",
                "match",
                "matter",
                "maybe",
                "meet",
                "meet",
                "meet",
                "men",
                "men",
                "men",
                "men",
                "men",
                "might",
                "moment",
                "more",
                "more",
                "morning",
                "morning",
                "much",
                "name",
                "nineteen",
                "nineteen",
                "no",
                "no",
                "not",
                "notches",
                "now",
                "numbered",
                "of",
                "of",
                "of",
                "of",
                "of",
                "oh",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "on",
                "one",
                "one",
                "one",
                "one",
                "one",
                "one",
                "one",
                "or",
                "out",
                "outlaw",
                "outlaw",
                "outlaw",
                "outlaw",
                "outlaw",
                "over",
                "passed",
                "past",
                "pistol",
                "plain",
                "play",
                "quickly",
                "ranger",
                "ranger",
                "ranger",
                "ranger",
                "ranger",
                "ranger",
                "ranger's",
                "red",
                "red",
                "red",
                "red",
                "red",
                "relayed",
                "riding",
                "ripped",
                "rode",
                "rode",
                "round",
                "running",
                "said",
                "say",
                "side",
                "slip",
                "slip",
                "slip",
                "slowly",
                "so",
                "some",
                "south",
                "spoke",
                "started",
                "still",
                "stopped",
                "story",
                "stranger",
                "stranger",
                "stranger",
                "street",
                "swiftness",
                "take",
                "take",
                "take",
                "talked",
                "talking",
                "texas",
                "texas",
                "texas",
                "texas",
                "texas",
                "that",
                "that",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "the",
                "their",
                "their",
                "them",
                "them",
                "them",
                "them",
                "there",
                "there",
                "there",
                "there",
                "they",
                "they",
                "they",
                "this",
                "this",
                "though",
                "time",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "to",
                "today",
                "too",
                "too",
                "town",
                "town",
                "town",
                "town",
                "tried",
                "tried",
                "tried",
                "tried",
                "tried",
                "twenty",
                "twenty",
                "twenty",
                "twenty",
                "twenty",
                "vicious",
                "walked",
                "was",
                "was",
                "was",
                "was",
                "was",
                "was",
                "was",
                "was",
                "was",
                "was",
                "was",
                "wasn't",
                "watching",
                "went",
                "were",
                "were",
                "were",
                "when",
                "when",
                "when",
                "when",
                "when",
                "whisper",
                "windows",
                "with",
                "with",
                "with",
                "with",
                "with",
                "worry",
                "would",
                "wouldn't",
                "youth"
            ]
        )
    }
}
