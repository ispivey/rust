// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -C codegen-units=3
// aux-build:sepcomp_cci_lib.rs

// Test accessing cross-crate inlined items from multiple compilation units.

// pretty-expanded FIXME #23616

extern crate sepcomp_cci_lib;
use sepcomp_cci_lib::{cci_fn, CCI_STATIC};

fn call1() -> uint {
    cci_fn() + CCI_STATIC
}

mod a {
    use sepcomp_cci_lib::{cci_fn, CCI_STATIC};
    pub fn call2() -> uint {
        cci_fn() + CCI_STATIC
    }
}

mod b {
    use sepcomp_cci_lib::{cci_fn, CCI_STATIC};
    pub fn call3() -> uint {
        cci_fn() + CCI_STATIC
    }
}

fn main() {
    assert_eq!(call1(), 1234);
    assert_eq!(a::call2(), 1234);
    assert_eq!(b::call3(), 1234);
}
