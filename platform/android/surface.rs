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
use opengles::gl2::{NO_ERROR, RGB, RGBA, GLint, GLsizei, UNSIGNED_BYTE, TEXTURE_2D, GLbyte};
use opengles::gl2;
use egl::egl::{EGLContext, EGLDisplay, EGL_DEFAULT_DISPLAY, EGLSurface, EGL_DRAW, EGL_BACK_BUFFER, EGLNativePixmapType};
use egl::egl::{EGL_RED_SIZE, EGL_BLUE_SIZE, EGL_GREEN_SIZE, EGL_ALPHA_SIZE,
                    EGL_NONE};
use egl::egl::{EGLConfig, EGLint, EGLBoolean};
use egl::egl::{EGL_NO_DISPLAY, EGL_DEFAULT_DISPLAY, EGL_NO_CONTEXT, EGL_CONTEXT_CLIENT_VERSION};
use egl::egl::{GetCurrentDisplay, GetDisplay, DestroyContext, GetCurrentSurface, GetCurrentContext, BindTexImage, eglCreatePixmapSurface, eglSwapBuffers, ChooseConfig};
use std::cast;
use std::libc::{c_char, c_int, c_uint, c_void, malloc, free};
use std::ptr;
use std::vec;

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
    pixmap: EGLNativePixmapType,
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
    pub fn from_pixmap(pixmap: EGLNativePixmapType) -> NativeSurface {
        NativeSurface {
            pixmap : pixmap,
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
            pixmap: ptr::null(),
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
        //let _bound = texture.bind();

        let mut numconfig : EGLint = 0;
        let mut myconfig : EGLConfig = ptr::null();
        let attr_list = ~[ EGL_RED_SIZE, 8,
        EGL_GREEN_SIZE, 8,
        EGL_BLUE_SIZE, 8,
        EGL_ALPHA_SIZE, 8,
        EGL_NONE];
        let attr_list = vec::raw::to_ptr(attr_list) as *i32;
        
        let ret = ChooseConfig(native_context.display, attr_list, &mut myconfig, 1, &mut numconfig);
        let attribute_list = 0 as *i32;

        unsafe {
        let mut image = eglCreatePixmapSurface(native_context.display, myconfig, self.pixmap, attribute_list);
        eglSwapBuffers(native_context.display, image);
        }
        //BindTexImage(native_context.display, self.egl_surface, EGL_BACK_BUFFER);

/*        let mut depth = 4 as c_int;
        let mut len = _size.width * _size.height * depth as int;
        let mut pixmap:~[u8] = ~[];
        let mut cnt = 0;
        while cnt < len {
            pixmap.push(125);
            cnt+=1;
        }

        let mut _pixmap:&[u8] = pixmap;
        unsafe {
        gl2::tex_image_2d(TEXTURE_2D, 0, RGBA as GLint, _size.width as GLsizei, _size.height as GLsizei, 0, RGBA, UNSIGNED_BYTE, Some(_pixmap));
        }
        */
    }

    /// This may only be called on the painting side.
    #[fixed_stack_segment]
    fn upload(&self, graphics_context: &NativePaintingGraphicsContext, data: &[u8]) {
        //To be implemented
    }

    fn get_id(&self) -> int {
        self.pixmap as int
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
