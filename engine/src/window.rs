use glutin::{EventsLoop, PossiblyCurrent, WindowBuilder, WindowedContext};

pub fn init_gl_window(window_size: (i32, i32)) -> (EventsLoop, WindowedContext<PossiblyCurrent>) {
    let el = EventsLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(glutin::dpi::LogicalSize::new(
            window_size.0 as f64,
            window_size.1 as f64,
        ));
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    unsafe {
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
    };

    (el, windowed_context)
}
