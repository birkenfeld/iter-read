// Copyright (c) 2015-2018 Georg Brandl.  Licensed under the Apache License,
// Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::io;

use IterReadItem;


macro_rules! impl_byte {
    ($el_ty:ty, $($deref:tt)*) => {
        impl<'a> IterReadItem for $el_ty {
            impl_byte!(@inner [$($deref)*] []);
        }

        impl<'a, E> IterReadItem for Result<$el_ty, E> where io::Error: From<E> {
            impl_byte!(@inner [$($deref)*] [?]);
        }
    };
    (@inner [$($deref:tt)*] [$($try:tt)*]) => {
        type Buffer = ();

        fn read<I: Iterator<Item=Self>>(target: &mut [u8], it: &mut I,
                                        _: &mut ()) -> io::Result<usize> {
            let mut len = 0;
            for (slot, elt) in target.iter_mut().zip(it) {
                *slot = $($deref)* elt $($try)*;
                len += 1;
            }
            Ok(len)
        }
    }
}


macro_rules! impl_slice_like {
    ($el_ty:ty, $buf_ty:ty, $conv:ident) => {
        impl<'a> IterReadItem for $el_ty {
            impl_slice_like!(@inner $buf_ty, $conv, buf:
                             Some(v) => {
                                 *buf = (v.$conv(), 0);
                             });
        }

        impl<'a, E> IterReadItem for Result<$el_ty, E> where io::Error: From<E> {
            impl_slice_like!(@inner $buf_ty, $conv, buf:
                             Some(Ok(v)) => {
                                 *buf = (v.$conv(), 0);
                             }
                             Some(Err(err)) => {
                                 *buf = (Default::default(), 0);
                                 return Err(err.into());
                             });
        }
    };
    (@inner $buf_ty:ty, $conv:ident, $buf:ident : $($match:tt)+) => {
        type Buffer = ($buf_ty, usize);  // buffer, consumed

        fn read<I: Iterator<Item=Self>>(target: &mut [u8], it: &mut I,
                                        $buf: &mut Self::Buffer) -> io::Result<usize> {
            while $buf.1 == $buf.0.len() {
                match it.next() {
                    $($match)+
                    None => {
                        *$buf = (Default::default(), 0);
                        return Ok(0);
                    }
                }
            }
            let mut len = 0;
            for (slot, elt) in target.iter_mut().zip(&$buf.0[$buf.1..]) {
                *slot = *elt;
                len += 1;
            }
            $buf.1 += len;
            Ok(len)
        }
    }
}

impl_byte!(u8, );
impl_byte!(&'a u8, *);

impl_slice_like!(&'a [u8], &'a [u8], into);
impl_slice_like!(&'a Vec<u8>, &'a [u8], as_slice);
impl_slice_like!(Vec<u8>, Vec<u8>, into);
impl_slice_like!(&'a str, &'a [u8], as_bytes);
impl_slice_like!(&'a String, &'a [u8], as_bytes);
impl_slice_like!(String, Vec<u8>, into_bytes);
