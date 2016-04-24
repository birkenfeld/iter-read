`iter-read`
===========

[![Build status](https://api.travis-ci.org/birkenfeld/iter-read.png)](https://travis-ci.org/birkenfeld/iter-read)
[![Latest Version](https://img.shields.io/crates/v/iter-read.svg)](https://crates.io/crates/iter-read)

[Documentation](https://birkenfeld.github.io/iter-read/iter_read/index.html)

This crate is a small library that provides a type that implements
`std::io::Read` for iterators over bytes (`u8`) and sequences of it, and also
`Result<u8, E>`, `Result<Vec<u8>, E>` etc.

Installation
============

This crate works with Cargo and can be found on
[crates.io](https://crates.io/crates/iter-read) with a `Cargo.toml` like:

```toml
[dependencies]
iter-read = "*"
```

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
