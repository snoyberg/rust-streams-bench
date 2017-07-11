# Various Rust stream implementations

Intended to address the inner loop problem when using iterator adapters like
`filter`, in the same way that stream fusion in Haskell uses a `Skip`
constructor.

Requires unstable due to usage of `feature(test)`.
