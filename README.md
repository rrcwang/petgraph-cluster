This is an exercise in implementing various graph clustering algorithms over the graph library `petgraph`.

## Goals, in order of priority:

* Refreshing and updating graph clustering knowledge
* Designing a clean interface and library structure
* Learning idiomatic Rust, safe as possible (potential exceptions for FFI)
* Optimizing for speed, where it does not hinder the above goals

## Planned clustering algorithms:

* [ ] Correlation
* [ ] Min-cut
* [ ] Sparsest cut
* [ ] SVD
* [ ] LambdaCC

## TODO:

* Benchmark against, at various scales:
  * NetworkX (Python)
  * LightGraphs (Julia)
  * MATLAB
* Iterative optimization
  * Find a crate that does this? Lots of reasons not to do this myself

## Future:

* Consider using GraphBLAS (or similar) bindings for speed?
  * Study LEMON library implementation
