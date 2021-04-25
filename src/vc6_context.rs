use egl;
use gbm::gbm_bo_flags;
use std::fs::File;
use std::os::raw::c_void;
use std::ptr;

use crate::egl_utils::{choose_config, match_config_to_visual};

#[rustfmt::skip]
const ATTRIBUTES: [egl::EGLint; 13] = [
  egl::EGL_SURFACE_TYPE,    egl::EGL_WINDOW_BIT,
  egl::EGL_RED_SIZE,        8,
  egl::EGL_GREEN_SIZE,      8,
  egl::EGL_BLUE_SIZE,       8,
  egl::EGL_ALPHA_SIZE,      0,
  egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_ES2_BIT,
  egl::EGL_NONE,
];

#[rustfmt::skip]
const CONTEXT_ATTRIBS: [egl::EGLint; 3] = [
  egl::EGL_CONTEXT_CLIENT_VERSION, 2,
  egl::EGL_NONE,
];

const GBM_FORMAT: u32 = gbm::GBM_FORMAT_XRGB8888;

#[link(name = "drm")]
#[link(name = "gbm")]
extern "C" {}

pub fn open_card(path: &str) -> File {
  return std::fs::OpenOptions::new()
    .read(true)
    .write(true)
    .open(path)
    .expect("Couldn't open device");
}

pub struct Context {
  device: File,
  connector: drm::DRMModeConnector,
  crtc: drm::DRMModeCrtc,
  gbm_device: *mut gbm::RawDevice,
  gbm_surface: *mut gbm::RawSurface,
  egl_display: egl::EGLDisplay,
  egl_context: egl::EGLContext,
  egl_surface: egl::EGLSurface,

  previous_bo: *mut gbm::RawBO,
  previous_fb: u32,
}

impl Context {
  pub fn new() -> Self {
    let device = open_card("/dev/dri/card1");

    let resources = drm::mode_get_resources(&device).expect("Couldn't get DRM Mode Resources");
    let connector = drm::find_connector(&device, &resources).expect("No connector found");
    let mode_info = &connector.modes[0];
    let encoder = drm::find_encoder(&device, &connector).expect("No encoder found");
    let crtc = drm::mode_get_crtc(&device, encoder.crtc_id);

    let gbm_device = gbm::create_device(&device);
    let gbm_surface = gbm::surface_create(
      gbm_device,
      mode_info.hdisplay as u32,
      mode_info.vdisplay as u32,
      GBM_FORMAT,
      gbm_bo_flags::GBM_BO_USE_SCANOUT | gbm_bo_flags::GBM_BO_USE_RENDERING,
    );

    let egl_display = egl::get_display(gbm_device as *mut c_void).expect("Couldn't get display");
    if !egl::initialize(egl_display, &mut 0i32, &mut 0i32) {
      panic!("Couldn't initialize egl")
    }
    if !egl::bind_api(egl::EGL_OPENGL_ES_API) {
      panic!("Couldn't bind API");
    }

    let egl_configs = choose_config(egl_display, &ATTRIBUTES).expect("Couldn't choose config");
    let egl_config = match_config_to_visual(egl_display, GBM_FORMAT as i32, egl_configs)
      .expect("Could't match visual");
    let egl_context = egl::create_context(
      egl_display,
      egl_config,
      egl::EGL_NO_CONTEXT,
      &CONTEXT_ATTRIBS,
    )
    .expect("Couldn't create context");
    let egl_surface = egl::create_window_surface(
      egl_display,
      egl_config,
      gbm_surface as egl::EGLNativeDisplayType,
      &[],
    )
    .expect("Couldn't create window surface");

    egl::make_current(egl_display, egl_surface, egl_surface, egl_context);

    return Context {
      device,
      connector,
      crtc,
      gbm_device,
      gbm_surface,
      egl_display,
      egl_context,
      egl_surface,
      previous_bo: ptr::null_mut(),
      previous_fb: 0,
    };
  }

  #[inline(always)]
  pub fn width(&mut self) -> u32 {
    self.connector.modes[0].hdisplay as u32
  }

  #[inline(always)]
  pub fn height(&mut self) -> u32 {
    self.connector.modes[0].vdisplay as u32
  }

  pub fn swap_buffers(&mut self) {
    egl::swap_buffers(self.egl_display, self.egl_surface);
    let bo = gbm::surface_lock_front_buffer(self.gbm_surface);
    let fb: u32 = 0;
    drm::mode_add_fb(
      &self.device,
      self.connector.modes[0].hdisplay as u32,
      self.connector.modes[0].vdisplay as u32,
      24,
      32,
      gbm::bo_get_stride(bo),
      gbm::bo_get_handle_u32(bo),
      &fb,
    );
    drm::mode_set_crtc(
      &self.device,
      self.crtc.crtc_id,
      fb,
      0,
      0,
      vec![self.connector.connector_id],
      &self.connector.modes[0].raw,
    );

    if !self.previous_bo.is_null() {
      drm::mode_rm_fb(&self.device, self.previous_fb);
      gbm::surface_release_buffer(self.gbm_surface, self.previous_bo);
    }
    self.previous_bo = bo;
    self.previous_fb = fb;
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    drm::mode_set_crtc(
      &self.device,
      self.crtc.crtc_id,
      self.crtc.buffer_id,
      self.crtc.x,
      self.crtc.y,
      vec![self.connector.connector_id],
      unsafe { &(*self.crtc.raw).mode },
    );
    if !(*self).previous_bo.is_null() {
      drm::mode_rm_fb(&self.device, self.previous_fb);
      gbm::surface_release_buffer(self.gbm_surface, self.previous_bo);
    }

    egl::destroy_surface(self.egl_display, self.egl_surface);
    gbm::surface_destroy(self.gbm_surface);
    egl::destroy_context(self.egl_display, self.egl_context);
    egl::terminate(self.egl_display);
    gbm::device_destroy(self.gbm_device);
  }
}
