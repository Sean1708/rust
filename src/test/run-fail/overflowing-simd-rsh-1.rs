// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern:thread '<main>' panicked at 'shift operation overflowed'
// compile-flags: -C debug-assertions

#![feature(core_simd)]

use std::simd::i32x4;

// (Work around constant-evaluation)
fn id<T>(x: T) -> T { x }

fn main() {
    let _x = i32x4(-1, 0, 0, 0) >> id(i32x4(32, 0, 0, 0));
}
