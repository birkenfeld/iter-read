// Copyright (c) 2015-2016 Georg Brandl.  Licensed under the Apache License,
// Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::io::Result;

use IterReadItem;


macro_rules! impl_byte {
    ($ty:ty, $pat:pat, $id:ident) => {
        impl<'a> IterReadItem for $ty {
            type Buffer = ();
            fn read<I: Iterator<Item=Self>>(target: &mut [u8], it: &mut I,
                                            _: &mut ()) -> Result<usize> {
                let mut len = 0;
                for (slot, $pat) in target.iter_mut().zip(it) {
                    *slot = $id;
                    len += 1;
                }
                Ok(len)
            }
        }
    }
}

impl_byte!(u8, v, v);
impl_byte!(&'a u8, &v, v);

macro_rules! impl_slice_like {
    ($ty:ty, $bty:ty, $conv:ident) => {
        impl<'a> IterReadItem for $ty {
            type Buffer = (usize, Option<$bty>, usize);
            fn read<I: Iterator<Item=Self>>(target: &mut [u8], it: &mut I,
                                            buf: &mut Self::Buffer) -> Result<usize> {
                while buf.2 == buf.0 {
                    match it.next() {
                        None => { *buf = (0, None, 0); return Ok(0) },
                        Some(v) => *buf = (v.len(), Some(v.$conv()), 0),
                    }
                }
                let mut len = 0;
                for (slot, elt) in target.iter_mut().zip(
                    &buf.1.as_ref().unwrap()[buf.2..])
                {
                    *slot = *elt;
                    len += 1;
                }
                buf.2 += len;
                Ok(len)
            }
        }
    }
}

impl_slice_like!(&'a [u8], &'a [u8], into);
impl_slice_like!(&'a Vec<u8>, &'a [u8], as_slice);
impl_slice_like!(Vec<u8>, Vec<u8>, into);
impl_slice_like!(&'a str, &'a [u8], as_bytes);
impl_slice_like!(&'a String, &'a [u8], as_bytes);
impl_slice_like!(String, Vec<u8>, into_bytes);
