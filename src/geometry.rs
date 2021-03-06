// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::CGFloat;

pub const CG_ZERO_POINT: CGPoint = CGPoint {
    x: 0.0,
    y: 0.0,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    #[inline]
    pub fn new(width: CGFloat, height: CGFloat) -> CGSize {
        CGSize {
            width: width,
            height: height,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    #[inline]
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint {
        CGPoint {
            x: x,
            y: y,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize
}

impl CGRect {
    #[inline]
    pub fn new(origin: &CGPoint, size: &CGSize) -> CGRect {
        CGRect {
            origin: *origin,
            size: *size,
        }
    }

    #[inline]
    pub fn inset(&self, size: &CGSize) -> CGRect {
        unsafe {
            ffi::CGRectInset(*self, size.width, size.height)
        }
    }
}

mod ffi {
    use base::CGFloat;
    use geometry::CGRect;

    #[link(name = "ApplicationServices", kind = "framework")]
    extern {
        pub fn CGRectInset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
    }
}

