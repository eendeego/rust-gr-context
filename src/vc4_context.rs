// Original work from:
// https://github.com/seankerr/rust-rpi-examples/blob/master/opengles/hello_opengl/src/main.rs
// https://jan.newmarch.name/RPi/EGL/

extern crate egl;
extern crate videocore;

use std::ptr;

use egl::{EGLConfig, EGLContext, EGLDisplay, EGLNativeDisplayType, EGLSurface};

// use crate::gbm_formats;
use dispmanx::DisplayHandle;

use videocore::bcm_host;
use videocore::dispmanx;
#[allow(unused_imports)]
use videocore::dispmanx::{FlagsAlpha, Transform, VCAlpha, Window};
use videocore::image::Rect;

#[link(name = "EGL")]
#[link(name = "GLESv2")]
extern "C" {}

#[rustfmt::skip]
const CONTEXT_ATTRIBS: [egl::EGLint; 3] = [
  egl::EGL_CONTEXT_CLIENT_VERSION, 2,
  egl::EGL_NONE,
];

#[rustfmt::skip]
const ATTRIBUTES: [egl::EGLint; 15] = [
  egl::EGL_SURFACE_TYPE,    egl::EGL_WINDOW_BIT,
  egl::EGL_RED_SIZE,        8,
  egl::EGL_GREEN_SIZE,      8,
  egl::EGL_BLUE_SIZE,       8,
  egl::EGL_ALPHA_SIZE,      8,
  egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_ES2_BIT,
  egl::EGL_BIND_TO_TEXTURE_RGBA, egl::EGL_TRUE as egl::EGLint,
  egl::EGL_NONE,
];

// 0x34_32_52_58 => 'X', 'R', '2', '4'
// const GBM_FORMAT: u32 = gbm_formats::GBM_FORMAT_XRGB8888;

// RPi 3 is returning
// 0x00_01_a4_18 => 0x18, 0xa4, 0x01, 0x00
// 0x00_00_94_28 => '(', 0x94, 0x00, 0x00 // GBM_FORMAT_RGB565 or GBM_FORMAT_BGR565
// 0x00_00_84_28 => '(', 0x83, 0x00, 0x00
// const GBM_FORMAT: u32 = 0x00_00_94_28;

/*
 * Original: https://github.com/matusnovak/rpi-opengl-without-x
 */
fn egl_get_error_str() -> &'static str {
  match egl::get_error() {
    egl::EGL_SUCCESS => "The last function succeeded without error.",
    egl::EGL_NOT_INITIALIZED => {
      "EGL is not initialized, or could not be initialized, for the \
      specified EGL display connection."
    }
    egl::EGL_BAD_ACCESS => {
      "EGL cannot access a requested resource (for example a context \
      is bound in another thread)."
    }
    egl::EGL_BAD_ALLOC => "EGL failed to allocate resources for the requested operation.",
    egl::EGL_BAD_ATTRIBUTE => {
      "An unrecognized attribute or attribute value was passed in the \
      attribute list."
    }
    egl::EGL_BAD_CONTEXT => {
      "An EGLContext argument does not name a valid EGL rendering \
      context."
    }
    egl::EGL_BAD_CONFIG => {
      "An EGLConfig argument does not name a valid EGL frame buffer \
      configuration."
    }
    egl::EGL_BAD_CURRENT_SURFACE => {
      "The current surface of the calling thread is a window, pixel \
      buffer or pixmap that is no longer valid."
    }
    egl::EGL_BAD_DISPLAY => {
      "An EGLDisplay argument does not name a valid EGL display \
      connection."
    }
    egl::EGL_BAD_SURFACE => {
      "An EGLSurface argument does not name a valid surface (window, \
      pixel buffer or pixmap) configured for GL rendering."
    }
    egl::EGL_BAD_MATCH => {
      "Arguments are inconsistent (for example, a valid context \
      requires buffers not supplied by a valid surface)."
    }
    egl::EGL_BAD_PARAMETER => "One or more argument values are invalid.",
    egl::EGL_BAD_NATIVE_PIXMAP => {
      "A NativePixmapType argument does not refer to a valid native \
      pixmap."
    }
    egl::EGL_BAD_NATIVE_WINDOW => {
      "A NativeWindowType argument does not refer to a valid native \
      window."
    }
    egl::EGL_CONTEXT_LOST => {
      "A power management event has occurred. The application must \
      destroy all contexts and reinitialise OpenGL ES state and \
      objects to continue rendering."
    }
    _ => "Unknown error!",
  }
}

fn init_egl() -> (
  EGLConfig,
  EGLContext,
  EGLDisplay,
  i32, /* egl major */
  i32, /* egl minor */
) {
  bcm_host::init();

  let egl_display = egl::get_display(egl::EGL_DEFAULT_DISPLAY)
    .unwrap_or_else(|| panic!("Failed to get EGL display\n\n{}", egl_get_error_str()));

  // init display
  let mut egl_major = 0i32;
  let mut egl_minor = 0i32;
  if !egl::initialize(egl_display, &mut egl_major, &mut egl_minor) {
    panic!("Failed to initialize EGL\n\n{}", egl_get_error_str());
  }

  // choose first available configuration
  let egl_config = egl::choose_config(egl_display, &ATTRIBUTES, 1)
    .unwrap_or_else(|| panic!("Failed to get EGL configuration\n\n{}", egl_get_error_str()));

  // bind opengl es api
  if !egl::bind_api(egl::EGL_OPENGL_ES_API) {
    panic!(
      "Failed to bind EGL OpenGL ES API\n\n{}",
      egl_get_error_str()
    );
  }

  // create egl context
  let egl_context = egl::create_context(
    egl_display,
    egl_config,
    egl::EGL_NO_CONTEXT,
    &CONTEXT_ATTRIBS,
  )
  .unwrap_or_else(|| panic!("Failed to create EGL context\n\n{}", egl_get_error_str()));

  return (egl_config, egl_context, egl_display, egl_major, egl_minor);
}

fn init_dispmanx(device: u16) -> (DisplayHandle, Window) {
  // get screen resolution (same display number as display_open()
  let dimensions = bcm_host::graphics_get_display_size(device).expect("Could not get display size");

  // println!("Display size: {}x{}", dimensions.width, dimensions.height);

  // open the display
  let dispman_display = dispmanx::display_open(device as u32);

  // get update handle
  let dispman_update = dispmanx::update_start(0 /* priority */);

  // setup the destination rectangle where opengl will be drawing
  let mut dest_rect = Rect {
    x: 0,
    y: 0,
    width: dimensions.width as i32,
    height: dimensions.height as i32,
  };

  // setup the source rectangle where opengl will be drawing
  let mut src_rect = Rect {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
    // width: (dimensions.width as i32) << 16,
    // height: (dimensions.height as i32) << 16,
  };

  // draw opengl context on a clean background (cleared by the clear color)
  // let mut alpha = VCAlpha {
  //   flags: FlagsAlpha::FIXED_ALL_PIXELS,
  //   opacity: 255,
  //   mask: 0,
  // };

  // draw opengl context on top of whatever is running behind it
  // note: changing the layer for the dispmanx element will also adjust where it's drawn, if
  //       there are other graphical applications running

  //let mut alpha = VCAlpha{ flags:   FlagsAlpha::FROM_SOURCE,
  //                         opacity: 0,
  //                         mask:    0 };

  // create our dispmanx element upon which we'll draw opengl using EGL
  let dispman_element = dispmanx::element_add(
    dispman_update,
    dispman_display,
    0, /*layer*/
    &mut dest_rect,
    0, /* src */
    &mut src_rect,
    dispmanx::DISPMANX_PROTECTION_NONE,
    // &mut alpha,
    ptr::null_mut(), /* alpha */
    ptr::null_mut(), /* clamp */
    Transform::NO_ROTATE,
  );

  // Build an EGL_DISPMANX_WINDOW_T from the Dispmanx window
  let window = Window {
    element: dispman_element,
    width: dimensions.width as i32,
    height: dimensions.height as i32,
  };

  // submit changes
  dispmanx::update_submit_sync(dispman_update);

  // if dispmanx::element_remove(dispman_update, dispman_element) {
  //   panic!("Failed to element remove");
  // }

  return (dispman_display, window);
}

fn egl_from_dispmanx(
  egl_config: EGLConfig,
  egl_context: EGLContext,
  egl_display: EGLDisplay,
  window: &mut Window,
) -> EGLSurface {
  // create surface
  let egl_surface = egl::create_window_surface(
    egl_display,
    egl_config,
    (window as *mut _) as EGLNativeDisplayType,
    &[],
  )
  .unwrap_or_else(|| panic!("Failed to create EGL surface\n\n{}", egl_get_error_str()));

  // set current context
  if !egl::make_current(egl_display, egl_surface, egl_surface, egl_context) {
    panic!(
      "Failed to make EGL current context\n\n{}",
      egl_get_error_str()
    );
  }

  return egl_surface;
}

// Can't derive Debug because Window doensn't implement it
pub struct Context {
  egl_major: i32,
  egl_minor: i32,
  egl_context: EGLContext,
  egl_display: EGLDisplay,
  egl_surface: EGLSurface,
  dispman_display: DisplayHandle,
  window: Window,
  width: u32,
  height: u32,
}

impl Context {
  pub fn new() -> Self {
    let device = 0u16; /* LCD */

    let (egl_config, egl_context, egl_display, /*egl_surface,*/ egl_major, egl_minor) = init_egl();

    let (dispman_display, window) = init_dispmanx(device);

    let width: u32 = window.width as u32;
    let height: u32 = window.height as u32;

    let mut context = Context {
      egl_major,
      egl_minor,
      egl_context,
      egl_display,
      egl_surface: ptr::null_mut() as EGLSurface,
      dispman_display,
      window,
      width,
      height,
    };

    context.egl_surface =
      egl_from_dispmanx(egl_config, egl_context, egl_display, &mut context.window);

    return context;
  }

  pub fn egl_version(&self) -> (i32, i32) {
    (self.egl_major, self.egl_minor)
  }

  #[inline(always)]
  pub fn width(&mut self) -> u32 {
    self.width
  }

  #[inline(always)]
  pub fn height(&mut self) -> u32 {
    self.height
  }

  pub fn swap_buffers(&mut self) {
    egl::swap_buffers(self.egl_display, self.egl_surface);
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    if self.egl_surface != egl::EGL_NO_SURFACE
      && !egl::destroy_surface(self.egl_display, self.egl_surface)
    {
      println!("Error destroying surface");
    }

    // if !dispmanx::element_remove(dispman_update, dispman_element) {
    //   println!("Failed to element remove");
    // }

    if self.egl_context != egl::EGL_NO_CONTEXT
      && !egl::destroy_context(self.egl_display, self.egl_context)
    {
      println!("Error destroying main context");
    }
    if self.egl_display != egl::EGL_NO_DISPLAY && !egl::terminate(self.egl_display) {
      println!("Error terminating display");
    }
    if !egl::release_thread() {
      println!("Error releasing EGL thread resources");
    }
    if dispmanx::display_close(self.dispman_display) {
      println!("Error closing Dispmanx display");
    }
    bcm_host::deinit();
  }
}
