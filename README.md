Ergonomic stooge sort implementation.

Implements 2 methods for stooge-sorting `[T]`/`Vec<T>`:

* `.stooge_sort()` (for `Ord` types)
* `.stooge_sort_by()` (for everything else; bring your own comparator function!)

# Usage

Add the following to your `Cargo.toml`,

```text
[dependencies]
stoogesort = "0.1.0"
```

and import the [`Stooge`] extension trait.

```
use stoogesort::Stooge;
```

Usage should be identical to the `.sort()` and
 `.sort_by()` methods in [`slice`][slice::sort].

# Examples

Sorting an [Ord](std::cmp) type using
[`.stooge_sort()`](Stooge::stooge_sort):

```
use stoogesort::Stooge;
let mut nums = [3, 2, 1, -5];
nums.stooge_sort();
assert_eq!(nums, [-5, 1, 2, 3]);
```

Sorting a [PartialOrd](std::cmp) type using
[`.stooge_sort_by()`](Stooge::stooge_sort_by)

```
use stoogesort::Stooge;
let mut floats = [0.1, 0.0, 1.0, -1.6];
floats.stooge_sort_by(|a, b| a.partial_cmp(b).unwrap());
assert_eq!(floats, [-1.6, 0.0, 0.1, 1.0]);
```

# Acknowledgements

* The Rust project code and docs (license in `LICENSE.rust`) I blatantly plagiarized

* The pseudocode in the [Wikipedia article for stooge sort](https://en.wikipedia.org/wiki/Stooge_sort)

# Prior Art (or: "Why do it again?")

It's funny to implement slow algorithms in a "blazing-fast" language!

Also, I wanted to learn how to write a library (and one with a
cromulent, natural interface), use traits, and publish to crates.io.

To my knowledge, 2 stooge sort implementations (not counting crates that
promise to include stooge sort) already exist:

* [`stooge`](https://github.com/tydus101/rust_stooge_sort)

* [`sorting_rs`](https://github.com/flakusha/sorting_rs)

At risk of sounding rude, I don't like them very much.

## `stooge`

`stooge` can be forgiven here, as it was explicitly written as a
learning project, but it's illustrative.

Pulled from that crate's README (in all its Rust 2015 glory):

```ignore
extern crate stooge;

fn main() {
	let mut v: Vec<i32> = vec![1, 5, 4, 3];
	stooge::sort(&mut v);
	return v; // [1, 3, 4, 5]
}
```

The name is clever, but not particularly conducive to production use --
were this a serious project, I would have to write either `stooge::sort(v)`
or `sort(v)` instead of `v.sort()`. It's backwards.

The `sort()` function is also implemented on `[T: PartialOrd]`,
which I find unsatisfying. More on that on the next subsection.


## `sorting_rs`

This one is theoretically is intended for 'production' use
(judging by the version number), or at least for dissection.
However, its interface is still... wonky.

```ignore
let mut vec = vec![5,3,2,4];
sorting_rs::oddeven_sort(&mut vec);
assert_eq!(vec, &[2,3,4,5]);
```

There is [an obvious receiver here, it should've been a method](https://rust-lang.github.io/api-guidelines/predictability.html#c-method).

At the same time, it also implements these sorting algorithms
on `[T]` where `T: PartialOrd`, which I'll now describe my problem with:

`PartialOrd`'s `.lt()` (used by the `<` operator), `.gt()` (used by `>`),
and its "-or-equal-to" variants are not mathematically airtight.
Suppose there's an integer type, that, for some reason, has a NaN value.
I'll call it `NaNInt`. NaN is not a number -- it is nonsense to use NaN
in a comparison. Since there is at least one pair of `NaNInts` for which
comparison is impossible or nonsensical, they form a
[partial order](https://en.wikipedia.org/wiki/Partially_ordered_set).
Thus, the `NaNInt` type can only be `PartialOrd`. As such:

```ignore
use std::cmp::Ordering;

let a = NaNInt::from(1);
let b = NaNInt::from(2);
let c = NaNInt::NaN;

assert_eq!(a.partial_cmp(b), Some(Ordering::Less));
assert_eq!(a.partial_cmp(c), None);
assert_eq!(c.partial_cmp(c), None);
```

Okay, we're good. This makes sense. Now, let's consider
what happens when we use `.lt()` and `.gt()` on a NaN,
keeping in mind that these methods return a `bool`, not an `Option`.

```ignore
let a = NaNInt::from(1);
let b = NaNInt::from(2);
let c = NaNInt::NaN;

assert_eq!(c > a, false);
assert_eq!(c < a, false);
assert_eq!(b < c, false);
assert_eq!(b > c, false);
assert_eq!(c > c, false);
assert_eq!(c < c, false);
```

That's right! It's nonsense. A NaN is never less than or greater than
anything else, including itself. This causes sorting on `[T: PartialOrd]`
to be unspecified if it contains incomparable pairs of elements.

Indeed, this is documented (rather indirectly) in [`slice::sort_by`].

> The comparator function must define a total ordering for the elements in
> the slice. If the ordering is not total, the order of the elements is unspecified.

For this reason, the standard library instead provides 2 methods
(+ variants for unstable and cached and keyed) for sorting slices. They are:

* [`slice::sort()`]

* [`slice::sort_by()`]

Notice the bound [`Ord`](std::cmp) on [`sort()`](slice) and the distinct lack
of one on [`sort_by()`](slice). The standard library is protecting you from an
attempt to naively sort `PartialOrd`s by making you gaze upon
`.sort_by(|a, b| a.partial_cmp(b))` in the [`sort_by()`](slice) example.

As such, I've taken the liberty of imitating the standard library in this library.
It also makes implementation easy, since I can just copy function signatures and
even the code itself at times.
