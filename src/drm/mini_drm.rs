#![allow(dead_code)]

use std::ffi::CStr;
use std::fs::File;
use std::os::raw::{c_char, c_int};
use std::os::unix::io::{AsRawFd, RawFd};
use std::slice;

// // This is how we do -ldrm -lgbm -lEGL -lGL
// #[link(name = "drm")]
#[warn(improper_ctypes)]
#[repr(C)]
pub struct RawDRMModeRes {
  pub count_fbs: c_int,
  pub fbs: *const u32,

  pub count_crtcs: c_int,
  pub crtcs: *const u32,

  pub count_connectors: c_int,
  pub connectors: *const u32,

  pub count_encoders: c_int,
  pub encoders: *const u32,

  pub min_width: u32,
  pub max_width: u32,
  pub min_height: u32,
  pub max_height: u32,
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMModeConnection {
  DRM_MODE_CONNECTED = 1,
  DRM_MODE_DISCONNECTED = 2,
  DRM_MODE_UNKNOWNCONNECTION = 3,
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMModeSubPixel {
  DRM_MODE_SUBPIXEL_UNKNOWN = 1,
  DRM_MODE_SUBPIXEL_HORIZONTAL_RGB = 2,
  DRM_MODE_SUBPIXEL_HORIZONTAL_BGR = 3,
  DRM_MODE_SUBPIXEL_VERTICAL_RGB = 4,
  DRM_MODE_SUBPIXEL_VERTICAL_BGR = 5,
  DRM_MODE_SUBPIXEL_NONE = 6,
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawDRMModeModeInfo {
  pub clock: u32,
  pub hdisplay: u16,
  pub hsync_start: u16,
  pub hsync_end: u16,
  pub htotal: u16,
  pub hskew: u16,
  pub vdisplay: u16,
  pub vsync_start: u16,
  pub vsync_end: u16,
  pub vtotal: u16,
  pub vscan: u16,

  pub vrefresh: u32,

  pub flags: u32,
  pub r#type: u32,
  pub name: [c_char; 32],
}

#[derive(Clone, Debug, PartialEq)]
pub struct DRMModeModeInfo {
  pub clock: u32,
  pub hdisplay: u16,
  pub hsync_start: u16,
  pub hsync_end: u16,
  pub htotal: u16,
  pub hskew: u16,
  pub vdisplay: u16,
  pub vsync_start: u16,
  pub vsync_end: u16,
  pub vtotal: u16,
  pub vscan: u16,

  pub vrefresh: u32,

  pub flags: u32,
  pub r#type: u32,
  pub name: String,

  pub raw: RawDRMModeModeInfo,
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawDRMModeConnector {
  pub connector_id: u32,
  pub encoder_id: u32, //< Encoder currently connected to
  pub connector_type: u32,
  pub connector_type_id: u32,
  pub connection: DRMModeConnection,
  pub mmWidth: u32,
  pub mmHeight: u32, //< HxW in millimeters
  pub subpixel: DRMModeSubPixel,
  pub count_modes: c_int,
  pub modes: *const RawDRMModeModeInfo,

  pub count_props: c_int,
  pub props: *const u32,       //< List of property ids
  pub prop_values: *const u64, //< List of property values

  pub count_encoders: c_int,
  pub encoders: *const u32, //< List of encoder ids
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
pub struct DRMModeConnector {
  pub connector_id: u32,
  pub encoder_id: u32, //< Encoder currently connected to
  pub connector_type: u32,
  pub connector_type_id: u32,
  pub connection: DRMModeConnection,
  pub mm_width: u32,
  pub mm_height: u32, //< HxW in millimeters
  pub subpixel: DRMModeSubPixel,
  pub modes: Vec<DRMModeModeInfo>,

  pub props: Vec<u32>,       //< List of property ids
  pub prop_values: Vec<u64>, //< List of property values

  pub encoders: Vec<u32>, //< List of encoder ids

  pub raw: *const RawDRMModeConnector,
}

#[derive(Clone, Debug)]
pub struct DRMModeRes {
  pub fbs: Vec<u32>,
  pub crtcs: Vec<u32>,
  pub connectors: Vec<u32>,
  pub encoders: Vec<u32>,

  pub min_width: u32,
  pub max_width: u32,
  pub min_height: u32,
  pub max_height: u32,

  pub raw: *const RawDRMModeRes,
}

impl Drop for DRMModeRes {
  fn drop(&mut self) {
    unsafe {
      ffi::drmModeFreeResources(self.raw);
    }
  }
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawDRMModeEncoder {
  pub encoder_id: u32,
  pub encoder_type: u32,
  pub crtc_id: u32,
  pub possible_crtcs: u32,
  pub possible_clones: u32,
}

#[derive(Clone, Debug)]
pub struct DRMModeEncoder {
  pub encoder_id: u32,
  pub encoder_type: u32,
  pub crtc_id: u32,
  pub possible_crtcs: u32,
  pub possible_clones: u32,

  pub raw: *const RawDRMModeEncoder,
}

impl Drop for DRMModeEncoder {
  fn drop(&mut self) {
    unsafe {
      ffi::drmModeFreeEncoder(self.raw);
    }
  }
}

#[warn(improper_ctypes)]
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawDRMModeCrtc {
  pub crtc_id: u32,
  pub buffer_id: u32, //< FB id to connect to 0 = disconnect

  pub x: u32, //< Position on the framebuffer
  pub y: u32,
  pub width: u32,
  pub height: u32,
  pub mode_valid: c_int,
  pub mode: RawDRMModeModeInfo,

  pub gamma_size: c_int, //< Number of gamma stops
}

#[derive(Clone, Debug)]
pub struct DRMModeCrtc {
  pub crtc_id: u32,
  pub buffer_id: u32, //< FB id to connect to 0 = disconnect

  pub x: u32, //< Position on the framebuffer
  pub y: u32,
  pub width: u32,
  pub height: u32,
  pub mode_valid: c_int,
  pub mode: DRMModeModeInfo,

  pub gamma_size: c_int, //< Number of gamma stops

  pub raw: *const RawDRMModeCrtc,
}

impl Drop for DRMModeCrtc {
  fn drop(&mut self) {
    unsafe {
      ffi::drmModeFreeCrtc(self.raw);
    }
  }
}

pub mod ffi {
  use super::*;

  extern "C" {
    pub fn drmModeFreeResources(ptr: *const RawDRMModeRes);
    pub fn drmModeFreeCrtc(ptr: *const RawDRMModeCrtc);
    pub fn drmModeFreeConnector(ptr: *const RawDRMModeConnector);
    pub fn drmModeFreeEncoder(ptr: *const RawDRMModeEncoder);

    pub fn drmModeAddFB(
      fd: RawFd,
      width: u32,
      height: u32,
      depth: u8,
      bpp: u8,
      pitch: u32,
      bo_handle: u32,
      buf_id: *const u32,
    ) -> c_int;

    pub fn drmModeRmFB(fd: RawFd, bufferId: u32) -> c_int;

    pub fn drmModeSetCrtc(
      fd: RawFd,
      crtcId: u32,
      bufferId: u32,
      x: u32,
      y: u32,
      connectors: *const u32,
      count: c_int,
      mode: *const RawDRMModeModeInfo,
    ) -> c_int;

    pub fn drmModeGetResources(fd: RawFd) -> *const RawDRMModeRes;
    pub fn drmModeGetCrtc(fd: RawFd, crtcId: u32) -> *const RawDRMModeCrtc;
    pub fn drmModeGetEncoder(fd: RawFd, encoderId: u32) -> *const RawDRMModeEncoder;
    pub fn drmModeGetConnector(fd: RawFd, connectorId: u32) -> *const RawDRMModeConnector;

  }
}

impl DRMModeModeInfo {
  pub fn copy(&self) -> DRMModeModeInfo {
    return DRMModeModeInfo {
      clock: self.clock,
      hdisplay: self.hdisplay,
      hsync_start: self.hsync_start,
      hsync_end: self.hsync_end,
      htotal: self.htotal,
      hskew: self.hskew,
      vdisplay: self.vdisplay,
      vsync_start: self.vsync_start,
      vsync_end: self.vsync_end,
      vtotal: self.vtotal,
      vscan: self.vscan,

      vrefresh: self.vrefresh,

      flags: self.flags,
      r#type: self.r#type,
      name: self.name.clone().to_owned(),
      raw: self.raw,
    };
  }

  pub fn from_raw(raw_mode_info: &RawDRMModeModeInfo) -> DRMModeModeInfo {
    let mut name = unsafe { CStr::from_bytes_with_nul_unchecked(&(*raw_mode_info).name) }
      .to_string_lossy()
      .into_owned();
    let idx = name.find('\0');
    if idx.is_some() {
      name.truncate(idx.unwrap());
    }
    return DRMModeModeInfo {
      clock: (*raw_mode_info).clock,
      hdisplay: (*raw_mode_info).hdisplay,
      hsync_start: (*raw_mode_info).hsync_start,
      hsync_end: (*raw_mode_info).hsync_end,
      htotal: (*raw_mode_info).htotal,
      hskew: (*raw_mode_info).hskew,
      vdisplay: (*raw_mode_info).vdisplay,
      vsync_start: (*raw_mode_info).vsync_start,
      vsync_end: (*raw_mode_info).vsync_end,
      vtotal: (*raw_mode_info).vtotal,
      vscan: (*raw_mode_info).vscan,

      vrefresh: (*raw_mode_info).vrefresh,

      flags: (*raw_mode_info).flags,
      r#type: (*raw_mode_info).r#type,
      name,
      raw: *raw_mode_info,
    };
  }
}

impl Drop for DRMModeConnector {
  fn drop(&mut self) {
    if self.raw != std::ptr::null() {
      unsafe {
        ffi::drmModeFreeConnector(self.raw);
      }
    }
  }
}

impl DRMModeConnector {
  pub fn from_raw(raw_connector: &RawDRMModeConnector) -> DRMModeConnector {
    unsafe {
      let raw_modes = slice::from_raw_parts(
        (*raw_connector).modes as *const RawDRMModeModeInfo,
        (*raw_connector).count_modes as usize,
      );

      let mut modes = Vec::with_capacity((*raw_connector).count_modes as usize);
      for raw_mode in raw_modes {
        modes.push(DRMModeModeInfo::from_raw(raw_mode));
      }

      let result = DRMModeConnector {
        connector_id: (*raw_connector).connector_id,
        encoder_id: (*raw_connector).encoder_id,
        connector_type: (*raw_connector).connector_type,
        connector_type_id: (*raw_connector).connector_type_id,
        connection: (*raw_connector).connection,
        mm_width: (*raw_connector).mmWidth,
        mm_height: (*raw_connector).mmHeight,
        subpixel: (*raw_connector).subpixel,
        modes,
        props: (slice::from_raw_parts(
          (*raw_connector).props as *const u32,
          (*raw_connector).count_props as usize,
        ))
        .to_vec(),

        prop_values: (slice::from_raw_parts(
          (*raw_connector).prop_values as *const u64,
          (*raw_connector).count_props as usize,
        ))
        .to_vec(),

        encoders: (slice::from_raw_parts(
          (*raw_connector).encoders as *const u32,
          (*raw_connector).count_encoders as usize,
        ))
        .to_vec(),

        raw: raw_connector,
      };

      return result;
    }
  }
}

pub fn mode_get_resources(device: &File) -> Option<DRMModeRes> {
  let raw_resources = unsafe { ffi::drmModeGetResources((*device).as_raw_fd()) };
  if raw_resources.is_null() {
    return None;
  }

  unsafe {
    let result = DRMModeRes {
      fbs: (slice::from_raw_parts(
        (*raw_resources).fbs as *const u32,
        (*raw_resources).count_fbs as usize,
      ))
      .to_vec(),
      crtcs: (slice::from_raw_parts(
        (*raw_resources).crtcs as *const u32,
        (*raw_resources).count_crtcs as usize,
      ))
      .to_vec(),
      connectors: (slice::from_raw_parts(
        (*raw_resources).connectors as *const u32,
        (*raw_resources).count_connectors as usize,
      ))
      .to_vec(),
      encoders: (slice::from_raw_parts(
        (*raw_resources).encoders as *const u32,
        (*raw_resources).count_encoders as usize,
      ))
      .to_vec(),
      min_width: (*raw_resources).min_width,
      max_width: (*raw_resources).max_width,
      min_height: (*raw_resources).min_height,
      max_height: (*raw_resources).max_height,

      raw: raw_resources,
    };

    Some(result)
  }
}

pub fn mode_get_crtc(device: &File, crtc_id: u32) -> DRMModeCrtc {
  unsafe {
    let raw_crtc = ffi::drmModeGetCrtc((*device).as_raw_fd(), crtc_id);

    let crtc = DRMModeCrtc {
      crtc_id: (*raw_crtc).crtc_id,
      buffer_id: (*raw_crtc).buffer_id,
      x: (*raw_crtc).x,
      y: (*raw_crtc).y,
      width: (*raw_crtc).width,
      height: (*raw_crtc).height,
      mode_valid: (*raw_crtc).mode_valid,
      mode: DRMModeModeInfo::from_raw(&(*raw_crtc).mode),
      gamma_size: (*raw_crtc).gamma_size,

      raw: raw_crtc,
    };

    return crtc;
  }
}

pub fn mode_free_crtc(crtc: &mut DRMModeCrtc) {
  unsafe { ffi::drmModeFreeCrtc(crtc.raw) };
  crtc.raw = std::ptr::null();
}

pub fn mode_add_fb(
  device: &File,
  width: u32,
  height: u32,
  depth: u8,
  bpp: u8,
  pitch: u32,
  bo_handle: u32,
  buf_id: *const u32,
) -> c_int {
  unsafe {
    return ffi::drmModeAddFB(
      (*device).as_raw_fd(),
      width,
      height,
      depth,
      bpp,
      pitch,
      bo_handle,
      buf_id,
    );
  }
}

pub fn mode_set_crtc(
  device: &File,
  crtc_id: u32,
  buffer_id: u32,
  x: u32,
  y: u32,
  connectors: Vec<u32>,
  mode: *const RawDRMModeModeInfo,
) -> c_int {
  unsafe {
    return ffi::drmModeSetCrtc(
      (*device).as_raw_fd(),
      crtc_id,
      buffer_id,
      x,
      y,
      connectors.as_ptr(),
      connectors.len() as c_int,
      mode,
    );
  }
}

pub fn mode_rm_fb(device: &File, buffer_id: u32) -> c_int {
  return unsafe { ffi::drmModeRmFB((*device).as_raw_fd(), buffer_id) };
}

pub fn find_connector(device: &File, resources: &DRMModeRes) -> Option<DRMModeConnector> {
  unsafe {
    for connector_id in &resources.connectors {
      let connector = ffi::drmModeGetConnector((*device).as_raw_fd(), *connector_id);
      if (*connector).connection == DRMModeConnection::DRM_MODE_CONNECTED {
        return Some(DRMModeConnector::from_raw(&*connector));
      }

      ffi::drmModeFreeConnector(connector);
    }
  }

  None
}

pub fn find_encoder(device: &File, connector: *const DRMModeConnector) -> Option<DRMModeEncoder> {
  if unsafe { (*connector).encoder_id } == 0 {
    return None;
  }
  unsafe {
    let raw_encoder = ffi::drmModeGetEncoder((*device).as_raw_fd(), (*connector).encoder_id);

    let encoder = DRMModeEncoder {
      encoder_id: (*raw_encoder).encoder_id,
      encoder_type: (*raw_encoder).encoder_type,
      crtc_id: (*raw_encoder).crtc_id,
      possible_crtcs: (*raw_encoder).possible_crtcs,
      possible_clones: (*raw_encoder).possible_clones,

      raw: raw_encoder,
    };

    Some(encoder)
  }
}
