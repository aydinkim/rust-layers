// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of cross-process surfaces for Linux. This uses X pixmaps.

use platform::surface::NativeSurfaceMethods;
use texturegl::Texture;

use geom::size::Size2D;
use opengles::gl2::NO_ERROR;
use opengles::gl2;
use std::cast;
use std::libc::{c_char, c_int, c_uint, c_void};
use std::ptr;

/// The display and visual info. This is needed in order to upload on the painting side. This holds
/// a *strong* reference to the display and will close it when done.
///
/// FIXME(pcwalton): Mark nonsendable and noncopyable.
pub struct NativePaintingGraphicsContext{
    //To be implemented
    dummy: bool,
}

impl NativePaintingGraphicsContext {
    #[fixed_stack_segment]
    pub fn from_metadata(metadata: &NativeGraphicsMetadata) -> NativePaintingGraphicsContext {
        //To be implemented
        unsafe {
            NativePaintingGraphicsContext {
                dummy : false,
            }
        }
    }
}

impl Drop for NativePaintingGraphicsContext {
    #[fixed_stack_segment]
    fn drop(&mut self) {
        //To be implemented
    }
}

/// The display, visual info, and framebuffer configuration. This is needed in order to bind to a
/// texture on the compositor side. This holds only a *weak* reference to the display and does not
/// close it.
///
/// FIXME(pcwalton): Unchecked weak references are bad and can violate memory safety. This is hard
/// to fix because the Display is given to us by the native windowing system, but we should fix it
/// someday.
///
/// FIXME(pcwalton): Mark nonsendable.
pub struct NativeCompositingGraphicsContext {
    //To be implemented
    dummy : bool,
}

impl NativeCompositingGraphicsContext {
    pub fn new() -> NativeCompositingGraphicsContext {
        //To be implemented
        NativeCompositingGraphicsContext {
            dummy : true,
        }
    }
}

pub struct NativeGraphicsMetadata {
    dummy: bool,
}

#[deriving(Eq)]
pub enum NativeSurfaceTransientData {
    NoTransientData,
}

pub struct NativeSurface {
    //To be implemented
    /// Whether this pixmap will leak if the destructor runs. This is for debugging purposes.
    will_leak: bool,
}

impl Drop for NativeSurface {
    fn drop(&mut self) {
        if self.will_leak {
            fail!("You should have disposed of the pixmap properly with destroy()! This pixmap \
                   will leak!");
        }
    }
}

impl NativeSurface {
    #[fixed_stack_segment]
    pub fn from_eglsurface(surface: /*to be changed to egl surface type*/*c_void) -> NativeSurface {
        NativeSurface {
            will_leak: true,
        }
    }
}

impl NativeSurfaceMethods for NativeSurface {
    #[fixed_stack_segment]
    fn new(native_context: &NativePaintingGraphicsContext, size: Size2D<i32>, stride: i32)
           -> NativeSurface {
               //To be implemented
               NativeSurface {
                   will_leak: false,
               }
    }

    /// This may only be called on the compositor side.
    #[fixed_stack_segment]
    fn bind_to_texture(&self,
                       native_context: &NativeCompositingGraphicsContext,
                       texture: &Texture,
                       _size: Size2D<int>) {
        //To be implemented
    }

    /// This may only be called on the painting side.
    #[fixed_stack_segment]
    fn upload(&self, graphics_context: &NativePaintingGraphicsContext, data: &[u8]) {
        //To be implemented
    }

    fn get_id(&self) -> int {
        //To be implemented
        1
    }

#[fixed_stack_segment]
    fn destroy(&mut self, graphics_context: &NativePaintingGraphicsContext) {
        unsafe {
            //To be implemented
            self.mark_wont_leak()
        }
    }

    fn mark_will_leak(&mut self) {
        self.will_leak = true
    }
    fn mark_wont_leak(&mut self) {
        self.will_leak = false
    }
}
