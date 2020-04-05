use glutin::{EventsLoop, PossiblyCurrent, WindowBuilder, WindowedContext};
use vulkano::instance::{
    Instance,
    InstanceExtensions,
    PhysicalDevice,
};
use vulkano::device::{
    Device,
    DeviceExtensions,
    Features,
};
use vulkano::buffer::{
    BufferUsage,
    CpuAccessibleBuffer
};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder,
    CommandBuffer,
};
use vulkano::sync::{
    GpuFuture,
};
use std::sync::Arc;
use vulkano::pipeline::ComputePipeline;
use vulkano::descriptor::{
    PipelineLayoutAbstract,
    descriptor_set::{
        PersistentDescriptorSet,
        UnsafeDescriptorSetLayout,
    }
};

mod cs {
    vulkano_shaders::shader!{
        ty: "compute",
        src: "
        #version 450

        layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

        layout(set = 0, binding = 0) buffer Data {
            uint data[];
        } buf;

        void main() {
            uint idx = gl_GlobalInvocationID.x;
            buf.data[idx] *= 12;
        }"
    }
}

pub fn init_vulkano_window(_window_size: (i32, i32)) -> (EventsLoop, WindowedContext<PossiblyCurrent>) {
    let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("Failed to create vulkan instance");
    println!("Instance {:?}", instance);
    let device = PhysicalDevice::enumerate(&instance).next().expect("no device available");
    println!("PhysicalDevice {:?}", device);
    for family in device.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }

    let queue_family = device.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("coulnd't find a graphical queue family");
    println!("Graphical queue family: {:?}", queue_family);

    let (device, mut queues) = {
        let deviceExtensions = DeviceExtensions {
            khr_storage_buffer_storage_class: true,
            ..DeviceExtensions::none()
        };
        Device::new(device, &Features::none(), &deviceExtensions,
        [(queue_family, 0.5)].iter().cloned()).expect("Failed to create device")
    };
    println!("Created device {:?} with queues", device);
    let queue = queues.next().unwrap();

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data).expect("failed to create buffer");

    let source_content = 0 .. 64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content).expect("failed to create buffer");

    let dest_content = (0 .. 64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, dest_content).expect("failed to create buffer");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .copy_buffer(source.clone(), dest.clone()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();


    let read_dest_buffer = dest.read().unwrap();
    println!("Command finished; destination {:?}", &*read_dest_buffer);

    let data_iter = 0 .. 65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, data_iter).expect("failed to create buffer");

    let shader = cs::Shader::load(device.clone()).expect("failed to create shaader module");
    println!("Shader is done");

    let compute_pipeline = Arc::new(ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).expect("failed to create compute pipeline"));
    println!("Compute pipeline created");

    let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(PersistentDescriptorSet::start(layout.clone())
                       .add_buffer(data_buffer.clone()).unwrap()
                       .build().unwrap());

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ()).unwrap()
        .build().unwrap();
    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything succeeded!");

    panic!();
}
