#![allow(dead_code)]

use std::fs::File;
// use std::os::raw::c_uint;
use std::os::unix::io::{AsRawFd, RawFd};

use super::gbm_formats;

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawDevice {
  _unused: [u8; 0],
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawSurface {
  _unused: [u8; 0],
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawBO {
  _unused: [u8; 0],
}

pub mod gbm_bo_format {
  pub type Type = std::os::raw::c_uint;
  pub const GBM_BO_FORMAT_XRGB8888: Type = 0;
  pub const GBM_BO_FORMAT_ARGB8888: Type = 1;
}

pub mod gbm_bo_flags {
  pub type Type = std::os::raw::c_uint;
  pub const GBM_BO_USE_SCANOUT: Type = 1;
  pub const GBM_BO_USE_CURSOR: Type = 2;
  pub const GBM_BO_USE_CURSOR_64X64: Type = 2;
  pub const GBM_BO_USE_RENDERING: Type = 4;
  pub const GBM_BO_USE_WRITE: Type = 8;
  pub const GBM_BO_USE_LINEAR: Type = 16;
  pub const GBM_BO_USE_PROTECTED: Type = 32;
}

pub use gbm_formats::*;

// #[repr(C, align(4))]
#[repr(C)]
pub union gbm_bo_handle {
  // pub ptr: *mut c_void,
  pub s32: i32,
  pub u32: u32,
  pub s64: i64,
  pub u64: u64,
}

pub mod ffi {
  use super::*;

  extern "C" {
    pub fn gbm_create_device(fd: RawFd) -> *mut RawDevice;
    pub fn gbm_device_destroy(gbm: *mut RawDevice);

    pub fn gbm_bo_get_stride(bo: *mut RawBO) -> u32;

    pub fn gbm_bo_get_handle(bo: *mut RawBO) -> gbm_bo_handle;

    pub fn gbm_surface_create(
      gbm: *mut RawDevice,
      width: u32,
      height: u32,
      format: u32,
      flags: u32,
    ) -> *mut RawSurface;

    pub fn gbm_surface_lock_front_buffer(surface: *mut RawSurface) -> *mut RawBO;

    pub fn gbm_surface_release_buffer(surface: *mut RawSurface, bo: *mut RawBO);

    pub fn gbm_surface_destroy(surface: *mut RawSurface);
  }
}

pub fn create_device(device: &File) -> *mut RawDevice {
  return unsafe { ffi::gbm_create_device((*device).as_raw_fd()) };
}

pub fn device_destroy(gbm: *mut RawDevice) {
  unsafe { ffi::gbm_device_destroy(gbm) };
}

pub fn bo_get_stride(bo: *mut RawBO) -> u32 {
  return unsafe { ffi::gbm_bo_get_stride(bo) };
}

pub fn bo_get_handle(bo: *mut RawBO) -> gbm_bo_handle {
  return unsafe { ffi::gbm_bo_get_handle(bo) };
}
pub fn bo_get_handle_u32(bo: *mut RawBO) -> u32 {
  return unsafe { ffi::gbm_bo_get_handle(bo).u32 };
}

pub fn surface_create(
  gbm: *mut RawDevice,
  width: u32,
  height: u32,
  format: u32,
  flags: u32,
) -> *mut RawSurface {
  return unsafe { ffi::gbm_surface_create(gbm, width, height, format, flags) };
}

pub fn surface_lock_front_buffer(surface: *mut RawSurface) -> *mut RawBO {
  return unsafe { ffi::gbm_surface_lock_front_buffer(surface) };
}

pub fn surface_destroy(surface: *mut RawSurface) {
  unsafe { ffi::gbm_surface_destroy(surface) };
}

pub fn surface_release_buffer(surface: *mut RawSurface, bo: *mut RawBO) {
  unsafe { ffi::gbm_surface_release_buffer(surface, bo) };
}
