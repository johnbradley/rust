// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

static x : [int, ..4] = [1,2,3,4];
static y : &'static [int] = &[1,2,3,4];

pub fn main() {
    printfln!(x[1]);
    printfln!(y[1]);
    assert_eq!(x[1], 2);
    assert_eq!(x[3], 4);
    assert_eq!(x[3], y[3]);
}
