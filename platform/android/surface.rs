// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of cross-process surfaces for Android. This uses EGL surface.

use platform::surface::NativeSurfaceMethods;
use texturegl::Texture;

use geom::size::Size2D;
use opengles::gl2::{NO_ERROR, RGB, GLint, GLsizei, UNSIGNED_BYTE};
use opengles::gl2;
use egl::egl::{EGLContext, EGLDisplay, EGL_DEFAULT_DISPLAY, EGLSurface, EGL_DRAW};
use egl::egl::{GetCurrentDisplay, GetDisplay, DestroyContext, GetCurrentSurface, GetCurrentContext};
use std::cast;
use std::libc::{c_char, c_int, c_uint, c_void, malloc, free};
use std::ptr;

pub struct NativePaintingGraphicsContext{
    display : EGLDisplay,
}

impl NativePaintingGraphicsContext {
    #[fixed_stack_segment]
    pub fn from_metadata(metadata: &NativeGraphicsMetadata) -> NativePaintingGraphicsContext {
       unsafe {
           NativePaintingGraphicsContext {
               display : metadata.display,
           }
       }
    }
}

impl Drop for NativePaintingGraphicsContext {
    #[fixed_stack_segment]
    fn drop(&mut self) {
        //let display: EGLDisplay = GetDisplay(EGL_DEFAULT_DISPLAY as *c_void);
        //DestroyContext(display,self.context);
    }
}

pub struct NativeCompositingGraphicsContext {
    display : EGLDisplay,
    context : EGLContext,
    surface : EGLSurface,
}

impl NativeCompositingGraphicsContext {
    pub fn new() -> NativeCompositingGraphicsContext {
        NativeCompositingGraphicsContext {
            display : GetCurrentDisplay(),
            context : GetCurrentContext(),
            surface : GetCurrentSurface(EGL_DRAW),
        }
    }
}

pub struct NativeGraphicsMetadata {
    display : EGLDisplay,
}

#[deriving(Eq)]
pub enum NativeSurfaceTransientData {
    NoTransientData,
}

pub struct NativeSurface {
    egl_surface: EGLSurface,
    will_leak: bool,
}

impl Drop for NativeSurface {
    fn drop(&mut self) {
        if self.will_leak {
            fail!("EGL fail in NativeSurface::Drop!");
        }
    }
}

impl NativeSurface {
    #[fixed_stack_segment]
    pub fn from_imageKHR(surface: EGLSurface) -> NativeSurface {
        NativeSurface {
            egl_surface: surface,
            will_leak: true,
        }
    }
}

impl NativeSurfaceMethods for NativeSurface {
    #[fixed_stack_segment]
    fn new(native_context: &NativePaintingGraphicsContext, size: Size2D<i32>, stride: i32) -> NativeSurface {
        /*let mut depth = ((stride/size.width) * 8) as c_int;
        let mut len = size.width * size.height * depth;
        unsafe {
            let mut data = malloc(len as u32);
            NativeSurface {
            pixmap: cast::transmute(data),
            will_leak: false,
            }
        }*/

        NativeSurface {
            egl_surface:GetCurrentSurface(EGL_DRAW),
            will_leak : false,
        }
    }

    /// This may only be called on the compositor side.
    #[fixed_stack_segment]
    fn bind_to_texture(&self,
                       native_context: &NativeCompositingGraphicsContext,
                       texture: &Texture,
                       _size: Size2D<int>) {
        //To be implemented
        let _bound = texture.bind();
        //gl2::tex_image_2d(self.target.as_gl_target(), 0, RGB as GLint, _size.width as GLsizei, _size.height as GLsizei, 0, RGB, UNSIGNED_BYTE, self.pixmap);

    }

    /// This may only be called on the painting side.
    #[fixed_stack_segment]
    fn upload(&self, graphics_context: &NativePaintingGraphicsContext, data: &[u8]) {
        //To be implemented
    }

    fn get_id(&self) -> int {
        self.egl_surface as int
    }

#[fixed_stack_segment]
    fn destroy(&mut self, graphics_context: &NativePaintingGraphicsContext) {
        unsafe {
            //free(self.pixmap);
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
