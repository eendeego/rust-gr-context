#![allow(dead_code)]

use std::ptr;

use egl;

mod ffi {
  use super::*;

  extern "C" {
    pub fn eglChooseConfig(
      dpy: egl::EGLDisplay,
      attrib_list: *const egl::EGLint,
      configs: *mut egl::EGLConfig,
      config_size: egl::EGLint,
      num_config: *mut egl::EGLint,
    ) -> egl::EGLBoolean;

    pub fn eglGetConfigs(
      dpy: egl::EGLDisplay,
      configs: egl::EGLConfig,
      config_size: egl::EGLint,
      num_config: *mut egl::EGLint,
    ) -> egl::EGLBoolean;
  }
}

pub fn get_config_count(display: egl::EGLDisplay) -> egl::EGLint {
  unsafe {
    let mut count: egl::EGLint = 0;

    ffi::eglGetConfigs(display, ptr::null_mut(), 0, &mut count);

    count
  }
}

pub fn choose_config(
  display: egl::EGLDisplay,
  attrib_list: &[egl::EGLint],
) -> Option<Vec<egl::EGLConfig>> {
  let config_count = get_config_count(display);

  let mut configs: Vec<egl::EGLConfig> = Vec::with_capacity(config_count as usize);

  let attributes = if attrib_list.len() > 0 {
    attrib_list.as_ptr()
  } else {
    ptr::null()
  };

  unsafe {
    let mut returned_configs: egl::EGLint = 0;
    let success = ffi::eglChooseConfig(
      display,
      attributes,
      configs.as_mut_ptr(),
      config_count,
      &mut returned_configs,
    ) == egl::EGL_TRUE;

    if success {
      configs.set_len(returned_configs as usize);
      Some(configs)
    } else {
      None
    }
  }
}

pub fn match_config_to_visual(
  egl_display: egl::EGLDisplay,
  visual_id: egl::EGLint,
  configs: Vec<egl::EGLConfig>,
) -> Option<egl::EGLConfig> {
  let mut id: egl::EGLint = 0;
  for config in configs.iter() {
    if !egl::get_config_attrib(egl_display, *config, egl::EGL_NATIVE_VISUAL_ID, &mut id) {
      continue;
    }

    if id == visual_id {
      return Some(*config);
    }
  }

  return None;
}

pub fn print_config_info(display: egl::EGLDisplay, config: egl::EGLConfig) {
  let mut config_id: egl::EGLint = 0;
  let mut config_caveat: egl::EGLint = 0;
  let mut visual_id: egl::EGLint = 0;
  let mut visual_type: egl::EGLint = 0;
  let mut native_renderable: egl::EGLint = 0;
  let mut renderable_type: egl::EGLint = 0;
  let mut red_size: egl::EGLint = 0;
  let mut green_size: egl::EGLint = 0;
  let mut blue_size: egl::EGLint = 0;
  let mut alpha_size: egl::EGLint = 0;
  let mut depth_size: egl::EGLint = 0;
  let mut buffer_size: egl::EGLint = 0;
  let mut surface_type: egl::EGLint = 0;
  let mut color_buffer_type: egl::EGLint = 0;
  let mut rgb_bind: egl::EGLint = 0;
  let mut rgba_bind: egl::EGLint = 0;

  egl::get_config_attrib(display, config, egl::EGL_CONFIG_ID, &mut config_id);
  egl::get_config_attrib(display, config, egl::EGL_CONFIG_CAVEAT, &mut config_caveat);
  egl::get_config_attrib(display, config, egl::EGL_NATIVE_VISUAL_ID, &mut visual_id);
  egl::get_config_attrib(
    display,
    config,
    egl::EGL_NATIVE_VISUAL_TYPE,
    &mut visual_type,
  );

  egl::get_config_attrib(
    display,
    config,
    egl::EGL_NATIVE_RENDERABLE,
    &mut native_renderable,
  );
  egl::get_config_attrib(
    display,
    config,
    egl::EGL_RENDERABLE_TYPE,
    &mut renderable_type,
  );
  egl::get_config_attrib(display, config, egl::EGL_BUFFER_SIZE, &mut buffer_size);
  egl::get_config_attrib(display, config, egl::EGL_RED_SIZE, &mut red_size);
  egl::get_config_attrib(display, config, egl::EGL_BLUE_SIZE, &mut green_size);
  egl::get_config_attrib(display, config, egl::EGL_GREEN_SIZE, &mut blue_size);
  egl::get_config_attrib(display, config, egl::EGL_ALPHA_SIZE, &mut alpha_size);
  egl::get_config_attrib(display, config, egl::EGL_DEPTH_SIZE, &mut depth_size);
  egl::get_config_attrib(display, config, egl::EGL_SURFACE_TYPE, &mut surface_type);
  egl::get_config_attrib(
    display,
    config,
    egl::EGL_COLOR_BUFFER_TYPE,
    &mut color_buffer_type,
  );
  egl::get_config_attrib(display, config, egl::EGL_BIND_TO_TEXTURE_RGB, &mut rgb_bind);
  egl::get_config_attrib(
    display,
    config,
    egl::EGL_BIND_TO_TEXTURE_RGBA,
    &mut rgba_bind,
  );

  println!(
    "  Renderable : {:#02x} {:#02x}",
    native_renderable, renderable_type
  );
  println!("  Visual ID  : {:#02x} {:#02x}", visual_id, visual_type);
  println!("  Config ID  : {:#02x} {:#02x}", config_id, config_caveat);
  println!(
    "  Sizes      : {}, {}, {}, {}, {}, {}",
    buffer_size, red_size, green_size, blue_size, alpha_size, depth_size
  );
  println!("  Surface    : {:#02x}", surface_type);
  println!(
    "  Color Buff : {}",
    if color_buffer_type == egl::EGL_RGB_BUFFER {
      "RGB"
    } else {
      "LUMINANCE"
    }
  );
  println!(
    "  Textures   : {} {}",
    if rgb_bind == egl::EGL_TRUE as i32 {
      "RGB"
    } else {
      "   "
    },
    if rgba_bind == egl::EGL_TRUE as i32 {
      "RGBA"
    } else {
      ""
    }
  );
}

pub fn print_configs(egl_display: egl::EGLDisplay, egl_configs: &Vec<egl::EGLConfig>) {
  for (i, egl_config) in egl_configs.iter().enumerate() {
    println!("Configuration {} is", i);
    print_config_info(egl_display, *egl_config);
  }
}
