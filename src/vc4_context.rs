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

fn init_egl() -> (EGLConfig, EGLContext, EGLDisplay) {
  bcm_host::init();

  let egl_display = egl::get_display(egl::EGL_DEFAULT_DISPLAY).expect("Failed to get EGL display");

  // init display
  if !egl::initialize(egl_display, &mut 0i32, &mut 0i32) {
    panic!("Failed to initialize EGL");
  }

  // println!("EGL has {} configs", get_config_count(egl_display));

  // let egl_configs = choose_config(egl_display, &ATTRIBUTES).expect("Couldn't choose config");
  // let egl_configs = choose_config(egl_display, &[]).expect("Couldn't choose config");

  // print_configs(egl_display, &egl_configs);

  // let egl_config = match_config_to_visual(egl_display, GBM_FORMAT as i32, egl_configs)
  //   .expect("Could't match visual");
  // let egl_config = egl_configs[0];

  // choose first available configuration
  let egl_config =
    egl::choose_config(egl_display, &ATTRIBUTES, 1).expect("Failed to get EGL configuration");

  // bind opengl es api
  if !egl::bind_api(egl::EGL_OPENGL_ES_API) {
    panic!("Failed to bind EGL OpenGL ES API");
  }

  // create egl context
  let egl_context = egl::create_context(
    egl_display,
    egl_config,
    egl::EGL_NO_CONTEXT,
    &CONTEXT_ATTRIBS,
  )
  .expect("Failed to create EGL context");

  return (egl_config, egl_context, egl_display);
}

fn init_dispmanx(device: u32) -> (DisplayHandle, Window) {
  // first thing to do is initialize the broadcom host (when doing any graphics on RPi)
  // again ?
  // bcm_host::init();

  // get screen resolution (same display number as display_open()
  let dimensions = bcm_host::graphics_get_display_size(device as u16)
    .expect("Must call bcm_host::init() prior to any display operation on RPi");

  // println!("Display size: {}x{}", dimensions.width, dimensions.height);

  // open the display
  let dispman_display = dispmanx::display_open(device);

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
    // 3 /*layer*/, // layer upon which to draw
    0, /* layer */
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
  .expect("Failed to create EGL surface");

  // set current context
  if !egl::make_current(egl_display, egl_surface, egl_surface, egl_context) {
    panic!("Failed to make EGL current context");
  }

  return egl_surface;
}

pub struct Context {
  egl_context: EGLContext,
  egl_display: EGLDisplay,
  egl_surface: EGLSurface,
  dispman_display: DisplayHandle,

  width: u32,
  height: u32,
}

impl Context {
  pub fn new() -> Self {
    let device = 0u32;

    let (egl_config, egl_context, egl_display) = init_egl();

    let (dispman_display, mut window) = init_dispmanx(device);

    let egl_surface = egl_from_dispmanx(egl_config, egl_context, egl_display, &mut window);

    Context {
      egl_context,
      egl_display,
      egl_surface,
      dispman_display,
      width: window.width as u32,
      height: window.height as u32,
    }
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
