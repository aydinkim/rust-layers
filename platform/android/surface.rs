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
use opengles::gl2::{EGLImageTargetTexture2DOES, TEXTURE_2D};
use egl::egl::EGLDisplay;
use egl::eglext::{EGLImageKHR, CreateImageKHR, DestroyImageKHR};
use std::cast;
use std::ptr;
use std::util;

pub struct NativeGraphicsMetadata {
    display : EGLDisplay,
}

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
    fn drop(&mut self) {}
}

pub struct NativeCompositingGraphicsContext {
    contents: (),
}

impl NativeCompositingGraphicsContext {
    pub fn new() -> NativeCompositingGraphicsContext {
        NativeCompositingGraphicsContext {
            contents: (),
        }
    }
}

pub struct NativeSurface {
    image: Option<EGLImageKHR>,
    will_leak: bool,
}

impl NativeSurface {
    #[fixed_stack_segment]
    pub fn from_image_khr(image_khr: EGLImageKHR) -> NativeSurface {
        let mut _image: Option<EGLImageKHR> = None;
        if image_khr != ptr::null() {
            _image = Some(image_khr);
        }
        NativeSurface {
            image : _image,
            will_leak: true,
        }
    }
}

impl NativeSurfaceMethods for NativeSurface {
    #[fixed_stack_segment]
    fn new(native_context: &NativePaintingGraphicsContext, size: Size2D<i32>, stride: i32) -> NativeSurface {
        //To be implemented
        NativeSurface {
            image: None,
            will_leak : true,
        }
    }

    /// This may only be called on the compositor side.
    #[fixed_stack_segment]
    fn bind_to_texture(&self,
                       native_context: &NativeCompositingGraphicsContext,
                       texture: &Texture,
                       _size: Size2D<int>) {
        let _bound = texture.bind();

        unsafe {
            match self.image {
                None => {},
                Some(image_khr) => {
                    EGLImageTargetTexture2DOES(TEXTURE_2D, image_khr);
                }
            }
        }
    }

    /// This may only be called on the painting side.
    #[fixed_stack_segment]
    fn upload(&self, graphics_context: &NativePaintingGraphicsContext, data: &[u8]) {
        //To be implemented
    }

    fn get_id(&self) -> int {
        match self.image {
            None => 0,
            Some(image_khr) => image_khr as int,
        }
    }

#[fixed_stack_segment]
    fn destroy(&mut self, graphics_context: &NativePaintingGraphicsContext) {
        match self.image {
            None => {},
            Some(image_khr) => {
                DestroyImageKHR(graphics_context.display, image_khr);
                util::replace(&mut self.image, None);
            }
        }
        self.mark_wont_leak()
    }

    fn mark_will_leak(&mut self) {
        self.will_leak = true
    }
    fn mark_wont_leak(&mut self) {
        self.will_leak = false
    }
}
