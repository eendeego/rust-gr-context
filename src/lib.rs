cfg_if::cfg_if! {
  if #[cfg(feature = "vc6")] {
    extern crate mini_drm as drm;
    extern crate mini_gbm as gbm;

    mod egl_utils;

    mod vc6_context;
    pub use vc6_context::Context;
  } else {
    mod egl_utils;

    mod vc4_context;
    pub use vc4_context::Context;
  }
}
