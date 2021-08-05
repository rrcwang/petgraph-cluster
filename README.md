# `petgraph` Clustering Library

This is an exercise in implementing various graph clustering algorithms over the graph
library [`petgraph`](https://docs.rs/petgraph/).

## Goals, in order of priority:

1. Refreshing and updating graph clustering knowledge
2. Designing a clean interface and library structure
3. Written in idiomatic Rust, safe as possible (potential exceptions for FFI)
4. Optimizing for speed, where it does not hinder the above goals

## Planned clustering algorithms:

* [ ] Correlation
  * [ ] Undirected
  * [ ] Directed
  * [ ] Weighted (un)directed
* [ ] Min-cut
* [ ] Sparsest cut
* [ ] SVD
* [ ] LambdaCC

## TODO:

* Benchmark against, at various scales:
  * [ ] NetworkX (Python)
  * [ ] LightGraphs (Julia)
  * [ ] MATLAB
  * [ ] Boost Graph Library
* Iterative optimization
  * Find a crate that does this? Lots of reasons not to do this myself

## Future:

* Consider using GraphBLAS (or similar) bindings for speed?
  * Study LEMON C++ library implementation
* Parallelize correlation clustering
* `no_std` possible (or even desirable)?
