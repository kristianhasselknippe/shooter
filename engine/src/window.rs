use glutin::{EventsLoop, PossiblyCurrent, WindowBuilder, WindowedContext};
use vulkano::instance::{
    Instance,
    InstanceExtensions
};

pub fn init_vulkano_window(window_size: (i32, i32)) -> (EventsLoop, WindowedContext<PossiblyCurrent>) {
    let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("Failed to create vulkan instance");
    unimplemented!();
}
