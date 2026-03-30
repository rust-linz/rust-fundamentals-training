// `pixels` provides a simple pixel framebuffer rendered via wgpu.
// `winit` provides the cross-platform window and event loop.
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

// Fixed pixel dimensions of the framebuffer. The window is created at this
// size and is not resizable, so these constants can be used as the row stride
// everywhere without passing them around at runtime.
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

// Holds all long-lived application state. Both fields start as `None` and are
// populated in `resumed`, which is the first point at which winit guarantees
// that a window can safely be created.
struct App {
    window: Option<Arc<Window>>,
    // `Pixels` owns the wgpu surface and the CPU-side framebuffer. The
    // `'static` lifetime is required because `Pixels` internally holds a
    // reference to the window; wrapping the window in `Arc` satisfies this.
    pixels: Option<Pixels<'static>>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            pixels: None,
        }
    }
}

// `ApplicationHandler` is winit's trait for the event-driven application
// lifecycle. Implementing it instead of polling the event loop directly is the
// modern winit approach (v0.30+).
impl ApplicationHandler for App {
    // Called when the application is ready to create windows (on first start,
    // or after returning from suspension on mobile platforms).
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let attrs = Window::default_attributes()
            .with_title("Pixels Demo")
            .with_inner_size(size)
            // Prevent the user from shrinking the window below the buffer size,
            // which would cause the surface and buffer to go out of sync.
            .with_min_inner_size(size);

        // `Arc` is needed so the window can be shared between `App` (which
        // drives the event loop) and `SurfaceTexture` (which needs a reference
        // to the window to present rendered frames).
        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        // `SurfaceTexture` wraps the window as a render target for wgpu.
        // `Pixels::new` allocates the CPU-side framebuffer (WIDTH × HEIGHT × 4
        // bytes, RGBA8) and sets up the GPU pipeline that blits it to the
        // surface on every `render()` call.
        let surface = SurfaceTexture::new(WIDTH, HEIGHT, Arc::clone(&window));
        let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    // Dispatched for every window event (input, lifecycle, redraw requests, …).
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            // The user clicked the close button or the OS asked us to quit.
            WindowEvent::CloseRequested => event_loop.exit(),

            // Allow quitting with the Escape key as a convenience.
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }
            }

            // The OS (or winit) is asking us to repaint the window. We fill
            // the CPU framebuffer via `draw`, then hand it to `pixels.render()`
            // which uploads it to the GPU and presents it.
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    draw(pixels.frame_mut());
                    if pixels.render().is_err() {
                        event_loop.exit();
                    }
                }
            }

            _ => {}
        }
    }
}

// Fills a filled rectangle into `frame`.
//
// `frame` is a flat RGBA8 buffer of WIDTH × HEIGHT pixels laid out in row-major
// order. Each pixel occupies 4 consecutive bytes: [R, G, B, A].
//
// `x`/`y`  – top-left corner of the rectangle in pixels
// `width`/`height` – dimensions of the rectangle in pixels
// `color`  – fill color as [R, G, B, A]
fn rect(frame: &mut [u8], x: u32, y: u32, width: u32, height: u32, color: [u8; 4]) {
    for row in y..y + height {
        for col in x..x + width {
            // Convert 2-D (col, row) to the flat byte offset.
            // Each row is WIDTH pixels wide; each pixel is 4 bytes.
            let idx = ((row * WIDTH + col) * 4) as usize;
            frame[idx..idx + 4].copy_from_slice(&color);
        }
    }
}

// Paints the full scene into the framebuffer on every redraw.
fn draw(frame: &mut [u8]) {
    // Clear the entire buffer to opaque black.
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, 255]);
    }

    // Red rectangle in the upper-left area.
    rect(frame, 50, 50, 200, 150, [255, 0, 0, 255]);

    // Blue rectangle in the lower-right area.
    rect(frame, 350, 250, 180, 120, [0, 128, 255, 255]);
}

fn main() {
    // `EventLoop` drives the OS message pump. `run_app` blocks until the
    // application exits and delegates all events to our `App` handler.
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
