// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate objc;
extern crate cocoa;
extern crate metal;
extern crate winit;

use cocoa::base::{selector, id, class, nil, BOOL, NO, YES};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
                        NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication, NSView,
                    NSApplicationActivateIgnoringOtherApps};

use metal::*;

use winit::os::macos::WindowExt;

use std::ffi::CStr;
use std::mem;

trait CAMetalLayer {
    unsafe fn layer(_: Self) -> id {
        msg_send![class("CAMetalLayer"), layer]
    }

    unsafe fn device(self) -> id;
    unsafe fn setDevice_(self, device: id);

    unsafe fn pixelFormat(self) -> id;
    unsafe fn setPixelFormat_(self, format: MTLPixelFormat);
}

impl CAMetalLayer for id {
    unsafe fn device(self) -> id {
        msg_send![self, device]
    }

    unsafe fn setDevice_(self, device: id) {
        msg_send![self, setDevice:device]
    }

    unsafe fn pixelFormat(self) -> id {
        msg_send![self, pixelFormat]
    }

    unsafe fn setPixelFormat_(self, format: MTLPixelFormat) {
        msg_send![self, setPixelFormat:format]
    }
}

fn main() {
    let window = winit::WindowBuilder::new().with_title("Metal".into()).build().unwrap();

    unsafe {
        let window: id = mem::transmute(window.get_nswindow());
        let device = MTLCreateSystemDefaultDevice();

        let layer = CAMetalLayer::layer(nil);
        layer.setDevice_(device);
        layer.setPixelFormat_(MTLPixelFormat::MTLPixelFormatBGRA8Unorm);

        let view = window.contentView(); 
        view.setWantsBestResolutionOpenGLSurface_(YES);
        view.setWantsLayer(YES);
        view.setLayer(layer);

        println!("device: {:?}", CStr::from_ptr(device.name().UTF8String()));
        println!("threadgroup: {:?}", device.maxThreadsPerThreadgroup());
    }

    loop {
        for event in window.poll_events() {
            match event {
                winit::Event::Closed => break,
                _ => ()
            }
        }
    }
}