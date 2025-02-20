# Bluem

**Bluem** (pronounced *bloom*) is a fast standard Bloom Filter
implementation that requires only two hash functions, generated
by `std::collections::hash_map::DefaultHasher`.

If an item is not present in the filter then `contains` is guaranteed
to return `false` for the queried item.

The probability that `contains` returns `true` for an item that is not
present in the filter is called the False Positive Rate (`fp_rate`).

### What is a Bloom Filter?

A Bloom filter is a space-efficient probabilistic data structure,
conceived by Burton Howard Bloom in 1970, that is used to test whether
an element is a member of a set. False positive matches are possible,
but false negatives are not – in other words, a query returns either
"possibly in set" or "definitely not in set". Elements can be added
to the set, but not removed (though this can be addressed with the
counting Bloom filter variant); the more items added, the larger the
probability of false positives.

### Example Usage

```rust
use bluem::BloomFilter;

let items_count = 1_000_000;
let fp_rate = 0.01;

let mut bloom = BloomFilter::new(items_count, fp_rate);
bloom.insert("foo");
bloom.insert("bar");
bloom.insert("baz");

bloom.contains("foo");              // true
bloom.contains("pterodactyl");      // false
```

### Contribute

+ I <3 pull requests and bug reports!
+ Don't hesitate to [tell me my code-fu sucks](https://github.com/samarthkulshrestha/bluem/issues/new), but please tell me why.

### License

This project is licensed under the MIT License.
Copyright (c) 2025 Samarth Kulshrestha.
