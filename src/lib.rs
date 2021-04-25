cfg_if::cfg_if! {
  if #[cfg(feature = "vc6")] {
    // mod mini_drm;
    // mod mini_gbm;
    mod drm {
      pub mod mini_drm;
    }
    mod gbm {
      pub mod mini_gbm;
      pub mod gbm_formats;
    }

    mod egl_utils;

    mod vc6_context;
    pub use vc6_context::Context;
  } else {
    mod egl_utils;

    mod vc4_context;
    pub use vc4_context::Context;
  }
}
