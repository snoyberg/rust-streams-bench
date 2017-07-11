# Various Rust stream implementations

Intended to address the inner loop problem when using iterator adapters like
`filter`, in the same way that stream fusion in Haskell uses a `Skip`
constructor.

Requires unstable due to usage of `feature(test)`.

Results on my system:

```
test tests::bench_v1        ... bench:  10,245,830 ns/iter (+/- 1,209,521)
test tests::bench_v2        ... bench:   4,950,595 ns/iter (+/- 774,231)
test tests::bench_v3        ... bench:   6,926,134 ns/iter (+/- 640,009)
test tests::bench_v4        ... bench:   4,265,632 ns/iter (+/- 585,089)
test tests::bench_v5        ... bench:   4,917,540 ns/iter (+/- 746,488)
test tests::bench_v6        ... bench:   4,185,733 ns/iter (+/- 648,818)
test tests::bench_v7        ... bench:  21,839,695 ns/iter (+/- 2,060,186)
test tests::bench_v8        ... bench:   4,053,212 ns/iter (+/- 731,559)
test tests::bench_zzz_iters ... bench:   5,176,363 ns/iter (+/- 755,918)
test tests::bench_zzz_loops ... bench:   4,188,582 ns/iter (+/- 561,500)
```

Notes:

* `v8` is the fastest. It is identical in approach to iterators,
  except it uses a representation like `Option<Option<T>>` to allow
  for skip. By comparison, `v1` simply uses three different
  constructors in the `Step` enum, and is significantly slower. (It
  seems like this is something that compiler optimizations could
  address.)

* `v6` is slightly slower than `v8`, and uses move semantics for the
  stream instead of mutable references (closer to the Haskell
  implementation). It uses just two constructors in the `Step`
  together with an `Option` wrapping (like `v8`). `v7` takes the three
  constructor approach, and is much slower.

* `v2` is completely isomorphic to iterators, and is just a sanity
  check that the performance between the two is the same.

* `v3` uses a separate `is_done` function to be checked each time a
  `Skip` is encountered

* `v4` puts a `bool` inside the `Skip` to indicate if processing is
  done

* `v5` is normal iterators plus move semantics
