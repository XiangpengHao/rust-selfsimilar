# rust-selfsimilar
[![Crates.io](https://img.shields.io/crates/v/selfsimilar.svg)](https://crates.io/crates/selfsimilar)

A fast generator of discrete random numbers.

The self similar distribution (80-20 rule) was described in 
>
    Jim Gray, Prakash Sundaresan, Susanne Englert, Ken Baclawski, and Peter J. Weinberger.1994. Quickly generating billion-record synthetic databases. SIGMOD Rec. 23, 2 (June 1994), 243–252.
    DOI:https://doi.org/10.1145/191843.191886

Integers between 1...N.
    The first h•N integers get 1-h of the distribution.

For example: 
```
if N = 25 and h= .10,
then 80% of the weight goes to the first 5 integers.
and 64% of the weight goes to the first integer.
```


### Usage
```rust
let dist = SelfSimilarDistribution::new(0, 100, 0.2);
let mut rng = rand::thread_rng();
let v = dist.sample(rng);
```
