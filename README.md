# Gr-Context

Gr-Context is a Raspberry Pi Video Code IV/VI Graphics Context provider for running Open GL/ES directly on hardware.

## Gr-Context in action

```rust
use gr_context::Context;
use opengles::glesv2 as gl;
use std::thread;
use std::time::{Duration, Instant};

const STEPS: u32 = 180;
const MILLIS_PER_FRAME: Duration = Duration::from_millis((1000_f64 / 60_f64) as u64);

pub fn draw(context: &mut Context, progress: f32) {
  gl::clear_color(1.0_f32 - progress, progress, 0.0, 1.0);
  gl::clear(gl::GL_COLOR_BUFFER_BIT);
  context.swap_buffers();
}

fn main() {
  let mut context = Context::new();

  for i in 0..STEPS {
    let start = Instant::now();
    draw(&mut context, i as f32 / STEPS as f32);

    let end = Instant::now();

    match start
      .checked_add(MILLIS_PER_FRAME)
      .expect("Can always add 16ms")
      .checked_duration_since(end)
    {
      Some(sleep) => thread::sleep(sleep),
      None => {}
    };
  }
}
```

## License

(The MIT License)

Copyright (c) 2021 Luis Reis

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
