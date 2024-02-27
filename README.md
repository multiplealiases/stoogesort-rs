Ergonomic stooge sort implementation.

Implements 2 methods for stooge-sorting `[T]`/`Vec<T>`:

* `.stooge_sort()` (for `Ord` types)
* `.stooge_sort_by()` (for everything else; bring your own comparator function!)


# Usage

Usage should be identical to the `.sort()` and
 `.sort_by()` methods in [`slice`][slice::sort].

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
(and with the exact same syntax as [`slice::sort_by`])
```
use stoogesort::Stooge;
let mut floats = [0.1, 0.0, 1.0, -1.6];
floats.stooge_sort_by(|a, b| a.partial_cmp(b).unwrap());
assert_eq!(floats, [-1.6, 0.0, 0.1, 1.0]);
```

# Prior Art

