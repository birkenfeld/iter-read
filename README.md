`iter-read`
===========

[![Build status](https://api.travis-ci.org/birkenfeld/iter-read.png)](https://travis-ci.org/birkenfeld/iter-read)
[![Latest Version](https://img.shields.io/crates/v/iter-read.svg)](https://crates.io/crates/iter-read)

[Documentation](https://birkenfeld.github.io/iter-read/iter_read/index.html)

This crate is a small library that provides a type that implements
`std::io::Read` for iterators over bytes (`u8`) and sequences of it, and also
`Result<u8, E>`, `Result<Vec<u8>, E>` etc.

The iterators must be fused (i.e. guarantee a `None` return from `next()`
after they have returned `None` once); you need to call `.fuse()` on
iterators that don't implement `std::iter::FusedIterator`.

Installation
============

This crate works with Cargo and can be found on
[crates.io](https://crates.io/crates/iter-read) with a `Cargo.toml` like:

```toml
[dependencies]
iter-read = "0.3"
```

Requirements
============

Minimum supported Rust version is 1.31.0.  No other dependencies.

Usage
=====

A simple example:

```
use std::io::Read;
use iter_read::IterRead;
let source = vec![1, 2, 7, 42, 123];
let mut reader = IterRead::new(source.iter());
let mut buf = vec![0; 3];
reader.read_exact(&mut buf).unwrap();
assert_eq!(buf, b"\x01\x02\x07");
```
