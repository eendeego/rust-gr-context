#![allow(dead_code)]

macro_rules! __gbm_fourcc_code {
  ($a: expr, $b: expr, $c: expr, $d: expr) => {{
    (($a as u32) | (($b as u32) << 8) | (($c as u32) << 16) | (($d as u32) << 24))
  }};
}

pub const GBM_FORMAT_BIG_ENDIAN: u32 = 1 << 31;

/* color index */
pub const GBM_FORMAT_C8: u32 = __gbm_fourcc_code!('C', '8', ' ', ' '); /* [7:0] C */

/* 8 bpp Red */
pub const GBM_FORMAT_R8: u32 = __gbm_fourcc_code!('R', '8', ' ', ' '); /* [7:0] R */

/* 16 bpp RG */
pub const GBM_FORMAT_GR88: u32 = __gbm_fourcc_code!('G', 'R', '8', '8'); /* [15:0] G:R 8:8 little endian */

/* 8 bpp RGB */
pub const GBM_FORMAT_RGB332: u32 = __gbm_fourcc_code!('R', 'G', 'B', '8'); /* [7:0] R:G:B 3:3:2 */
pub const GBM_FORMAT_BGR233: u32 = __gbm_fourcc_code!('B', 'G', 'R', '8'); /* [7:0] B:G:R 2:3:3 */

/* 16 bpp RGB */
pub const GBM_FORMAT_XRGB4444: u32 = __gbm_fourcc_code!('X', 'R', '1', '2'); /* [15:0] x:R:G:B 4:4:4:4 little endian */
pub const GBM_FORMAT_XBGR4444: u32 = __gbm_fourcc_code!('X', 'B', '1', '2'); /* [15:0] x:B:G:R 4:4:4:4 little endian */
pub const GBM_FORMAT_RGBX4444: u32 = __gbm_fourcc_code!('R', 'X', '1', '2'); /* [15:0] R:G:B:x 4:4:4:4 little endian */
pub const GBM_FORMAT_BGRX4444: u32 = __gbm_fourcc_code!('B', 'X', '1', '2'); /* [15:0] B:G:R:x 4:4:4:4 little endian */

pub const GBM_FORMAT_ARGB4444: u32 = __gbm_fourcc_code!('A', 'R', '1', '2'); /* [15:0] A:R:G:B 4:4:4:4 little endian */
pub const GBM_FORMAT_ABGR4444: u32 = __gbm_fourcc_code!('A', 'B', '1', '2'); /* [15:0] A:B:G:R 4:4:4:4 little endian */
pub const GBM_FORMAT_RGBA4444: u32 = __gbm_fourcc_code!('R', 'A', '1', '2'); /* [15:0] R:G:B:A 4:4:4:4 little endian */
pub const GBM_FORMAT_BGRA4444: u32 = __gbm_fourcc_code!('B', 'A', '1', '2'); /* [15:0] B:G:R:A 4:4:4:4 little endian */

pub const GBM_FORMAT_XRGB1555: u32 = __gbm_fourcc_code!('X', 'R', '1', '5'); /* [15:0] x:R:G:B 1:5:5:5 little endian */
pub const GBM_FORMAT_XBGR1555: u32 = __gbm_fourcc_code!('X', 'B', '1', '5'); /* [15:0] x:B:G:R 1:5:5:5 little endian */
pub const GBM_FORMAT_RGBX5551: u32 = __gbm_fourcc_code!('R', 'X', '1', '5'); /* [15:0] R:G:B:x 5:5:5:1 little endian */
pub const GBM_FORMAT_BGRX5551: u32 = __gbm_fourcc_code!('B', 'X', '1', '5'); /* [15:0] B:G:R:x 5:5:5:1 little endian */

pub const GBM_FORMAT_ARGB1555: u32 = __gbm_fourcc_code!('A', 'R', '1', '5'); /* [15:0] A:R:G:B 1:5:5:5 little endian */
pub const GBM_FORMAT_ABGR1555: u32 = __gbm_fourcc_code!('A', 'B', '1', '5'); /* [15:0] A:B:G:R 1:5:5:5 little endian */
pub const GBM_FORMAT_RGBA5551: u32 = __gbm_fourcc_code!('R', 'A', '1', '5'); /* [15:0] R:G:B:A 5:5:5:1 little endian */
pub const GBM_FORMAT_BGRA5551: u32 = __gbm_fourcc_code!('B', 'A', '1', '5'); /* [15:0] B:G:R:A 5:5:5:1 little endian */

pub const GBM_FORMAT_RGB565: u32 = __gbm_fourcc_code!('R', 'G', '1', '6'); /* [15:0] R:G:B 5:6:5 little endian */
pub const GBM_FORMAT_BGR565: u32 = __gbm_fourcc_code!('B', 'G', '1', '6'); /* [15:0] B:G:R 5:6:5 little endian */

/* 24 bpp RGB */
pub const GBM_FORMAT_RGB888: u32 = __gbm_fourcc_code!('R', 'G', '2', '4'); /* [23:0] R:G:B little endian */
pub const GBM_FORMAT_BGR888: u32 = __gbm_fourcc_code!('B', 'G', '2', '4'); /* [23:0] B:G:R little endian */

/* 32 bpp RGB */
pub const GBM_FORMAT_XRGB8888: u32 = __gbm_fourcc_code!('X', 'R', '2', '4'); /* [31:0] x:R:G:B 8:8:8:8 little endian */
pub const GBM_FORMAT_XBGR8888: u32 = __gbm_fourcc_code!('X', 'B', '2', '4'); /* [31:0] x:B:G:R 8:8:8:8 little endian */
pub const GBM_FORMAT_RGBX8888: u32 = __gbm_fourcc_code!('R', 'X', '2', '4'); /* [31:0] R:G:B:x 8:8:8:8 little endian */
pub const GBM_FORMAT_BGRX8888: u32 = __gbm_fourcc_code!('B', 'X', '2', '4'); /* [31:0] B:G:R:x 8:8:8:8 little endian */

pub const GBM_FORMAT_ARGB8888: u32 = __gbm_fourcc_code!('A', 'R', '2', '4'); /* [31:0] A:R:G:B 8:8:8:8 little endian */
pub const GBM_FORMAT_ABGR8888: u32 = __gbm_fourcc_code!('A', 'B', '2', '4'); /* [31:0] A:B:G:R 8:8:8:8 little endian */
pub const GBM_FORMAT_RGBA8888: u32 = __gbm_fourcc_code!('R', 'A', '2', '4'); /* [31:0] R:G:B:A 8:8:8:8 little endian */
pub const GBM_FORMAT_BGRA8888: u32 = __gbm_fourcc_code!('B', 'A', '2', '4'); /* [31:0] B:G:R:A 8:8:8:8 little endian */

pub const GBM_FORMAT_XRGB2101010: u32 = __gbm_fourcc_code!('X', 'R', '3', '0'); /* [31:0] x:R:G:B 2:10:10:10 little endian */
pub const GBM_FORMAT_XBGR2101010: u32 = __gbm_fourcc_code!('X', 'B', '3', '0'); /* [31:0] x:B:G:R 2:10:10:10 little endian */
pub const GBM_FORMAT_RGBX1010102: u32 = __gbm_fourcc_code!('R', 'X', '3', '0'); /* [31:0] R:G:B:x 10:10:10:2 little endian */
pub const GBM_FORMAT_BGRX1010102: u32 = __gbm_fourcc_code!('B', 'X', '3', '0'); /* [31:0] B:G:R:x 10:10:10:2 little endian */

pub const GBM_FORMAT_ARGB2101010: u32 = __gbm_fourcc_code!('A', 'R', '3', '0'); /* [31:0] A:R:G:B 2:10:10:10 little endian */
pub const GBM_FORMAT_ABGR2101010: u32 = __gbm_fourcc_code!('A', 'B', '3', '0'); /* [31:0] A:B:G:R 2:10:10:10 little endian */
pub const GBM_FORMAT_RGBA1010102: u32 = __gbm_fourcc_code!('R', 'A', '3', '0'); /* [31:0] R:G:B:A 10:10:10:2 little endian */
pub const GBM_FORMAT_BGRA1010102: u32 = __gbm_fourcc_code!('B', 'A', '3', '0'); /* [31:0] B:G:R:A 10:10:10:2 little endian */

/*
 * Floating point 64bpp RGB
 * IEEE 754-2008 binary16 half-precision float
 * [15:0] sign:exponent:mantissa 1:5:10
 */
pub const GBM_FORMAT_XBGR16161616F: u32 = __gbm_fourcc_code!('X', 'B', '4', 'H'); /* [63:0] x:B:G:R 16:16:16:16 little endian */

pub const GBM_FORMAT_ABGR16161616F: u32 = __gbm_fourcc_code!('A', 'B', '4', 'H'); /* [63:0] A:B:G:R 16:16:16:16 little endian */

/* packed YCbCr */
pub const GBM_FORMAT_YUYV: u32 = __gbm_fourcc_code!('Y', 'U', 'Y', 'V'); /* [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian */
pub const GBM_FORMAT_YVYU: u32 = __gbm_fourcc_code!('Y', 'V', 'Y', 'U'); /* [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian */
pub const GBM_FORMAT_UYVY: u32 = __gbm_fourcc_code!('U', 'Y', 'V', 'Y'); /* [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian */
pub const GBM_FORMAT_VYUY: u32 = __gbm_fourcc_code!('V', 'Y', 'U', 'Y'); /* [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian */

pub const GBM_FORMAT_AYUV: u32 = __gbm_fourcc_code!('A', 'Y', 'U', 'V'); /* [31:0] A:Y:Cb:Cr 8:8:8:8 little endian */

/*
 * 2 plane YCbCr
 * index 0 = Y plane, [7:0] Y
 * index 1 = Cr:Cb plane, [15:0] Cr:Cb little endian
 * or
 * index 1 = Cb:Cr plane, [15:0] Cb:Cr little endian
 */
pub const GBM_FORMAT_NV12: u32 = __gbm_fourcc_code!('N', 'V', '1', '2'); /* 2x2 subsampled Cr:Cb plane */
pub const GBM_FORMAT_NV21: u32 = __gbm_fourcc_code!('N', 'V', '2', '1'); /* 2x2 subsampled Cb:Cr plane */
pub const GBM_FORMAT_NV16: u32 = __gbm_fourcc_code!('N', 'V', '1', '6'); /* 2x1 subsampled Cr:Cb plane */
pub const GBM_FORMAT_NV61: u32 = __gbm_fourcc_code!('N', 'V', '6', '1'); /* 2x1 subsampled Cb:Cr plane */

/*
 * 3 plane YCbCr
 * index 0: Y plane, [7:0] Y
 * index 1: Cb plane, [7:0] Cb
 * index 2: Cr plane, [7:0] Cr
 * or
 * index 1: Cr plane, [7:0] Cr
 * index 2: Cb plane, [7:0] Cb
 */
pub const GBM_FORMAT_YUV410: u32 = __gbm_fourcc_code!('Y', 'U', 'V', '9'); /* 4x4 subsampled Cb (1) and Cr (2) planes */
pub const GBM_FORMAT_YVU410: u32 = __gbm_fourcc_code!('Y', 'V', 'U', '9'); /* 4x4 subsampled Cr (1) and Cb (2) planes */
pub const GBM_FORMAT_YUV411: u32 = __gbm_fourcc_code!('Y', 'U', '1', '1'); /* 4x1 subsampled Cb (1) and Cr (2) planes */
pub const GBM_FORMAT_YVU411: u32 = __gbm_fourcc_code!('Y', 'V', '1', '1'); /* 4x1 subsampled Cr (1) and Cb (2) planes */
pub const GBM_FORMAT_YUV420: u32 = __gbm_fourcc_code!('Y', 'U', '1', '2'); /* 2x2 subsampled Cb (1) and Cr (2) planes */
pub const GBM_FORMAT_YVU420: u32 = __gbm_fourcc_code!('Y', 'V', '1', '2'); /* 2x2 subsampled Cr (1) and Cb (2) planes */
pub const GBM_FORMAT_YUV422: u32 = __gbm_fourcc_code!('Y', 'U', '1', '6'); /* 2x1 subsampled Cb (1) and Cr (2) planes */
pub const GBM_FORMAT_YVU422: u32 = __gbm_fourcc_code!('Y', 'V', '1', '6'); /* 2x1 subsampled Cr (1) and Cb (2) planes */
pub const GBM_FORMAT_YUV444: u32 = __gbm_fourcc_code!('Y', 'U', '2', '4'); /* non-subsampled Cb (1) and Cr (2) planes */
pub const GBM_FORMAT_YVU444: u32 = __gbm_fourcc_code!('Y', 'V', '2', '4'); /* non-subsampled Cr (1) and Cb (2) planes */
