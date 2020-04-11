use img::ImageBuffer;
use img::Rgba;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::DynamicState;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};
use vulkano::descriptor::{
    descriptor_set::{PersistentDescriptorSet, UnsafeDescriptorSetLayout},
    PipelineLayoutAbstract,
};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::format::{ClearValue, Format};
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::ComputePipeline;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::{swapchain::Surface, sync::GpuFuture};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn init_vulkano_window(window_size: (i32, i32)) -> (EventLoop<()>, Arc<Instance>) {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };

    let mut events_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();
    (events_loop, instance)
}

mod cs {
    vulkano_shaders::shader! {
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

#[derive(Default, Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position);

fn graphics_pipeline(device: Arc<Device>, queue: Arc<Queue>) {
    mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: "
#version 450

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}"
        }
    }

    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}"
        }
    }

    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue.family()),
    )
    .unwrap();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        vec![vertex1, vertex2, vertex3].into_iter(),
    )
    .unwrap();

    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: Format::R8G8B8A8Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    let framebuffer = Arc::new(
        Framebuffer::start(render_pass.clone())
            .add(image.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let buf = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            // Defines what kind of vertex input is expected.
            .vertex_input_single_buffer::<Vertex>()
            // The vertex shader.
            .vertex_shader(vs.main_entry_point(), ())
            // Defines the viewport (explanations below).
            .viewports_dynamic_scissors_irrelevant(1)
            // The fragment shader.
            .fragment_shader(fs.main_entry_point(), ())
            // This graphics pipeline object concerns the first pass of the render pass.
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            // Now that everything is specified, we call `build`.
            .build(device.clone())
            .unwrap(),
    );

    let dynamic_state = DynamicState {
        viewports: Some(vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [1024.0, 1024.0],
            depth_range: 0.0..1.0,
        }]),
        ..DynamicState::none()
    };

    let command_buffer =
        AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
            .unwrap()
            .begin_render_pass(
                framebuffer.clone(),
                false,
                vec![[0.0, 0.0, 1.0, 1.0].into()],
            )
            .unwrap()
            .draw(
                pipeline.clone(),
                &dynamic_state,
                vertex_buffer.clone(),
                (),
                (),
            )
            .unwrap()
            .end_render_pass()
            .unwrap()
            // Render to image
            .copy_image_to_buffer(image.clone(), buf.clone())
            .unwrap()
            .build()
            .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("triangle.png").unwrap();
}

fn make_fractal(device: Arc<Device>, queue: Arc<Queue>) {
    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue.family()),
    )
    .unwrap();

    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            src: "
#version 450
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;
void main() {
    vec2 norm_coordinates = (gl_GlobalInvocationID.xy + vec2(0.5)) / vec2(imageSize(img));
    vec2 c = (norm_coordinates - vec2(0.5)) * 2.0 - vec2(1.0, 0.0);
    vec2 z = vec2(0.0, 0.0);
    float i;
    for (i = 0.0; i < 1.0; i += 0.005) {
        z = vec2(
            z.x * z.x - z.y * z.y + c.x,
            z.y * z.x + z.x * z.y + c.y
        );
        if (length(z) > 4.0) {
            break;
        }
    }
    vec4 to_write = vec4(vec3(i), 1.0);
    imageStore(img, ivec2(gl_GlobalInvocationID.xy), to_write);
}"
        }
    }

    let shader = cs::Shader::load(device.clone()).expect("failed to create shader module");

    let compute_pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
            .expect("failed to create compute pipeline"),
    );

    let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_image(image.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let buf = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .dispatch(
            [1024 / 8, 1024 / 8, 1],
            compute_pipeline.clone(),
            set.clone(),
            (),
        )
        .unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone())
        .unwrap()
        .build()
        .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();
}

pub fn vulkano_tests() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("Failed to create vulkan instance");
    println!("Instance {:?}", instance);
    let device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");
    println!("PhysicalDevice {:?}", device);
    for family in device.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    let queue_family = device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("coulnd't find a graphical queue family");
    println!("Graphical queue family: {:?}", queue_family);

    let (device, mut queues) = {
        let deviceExtensions = DeviceExtensions {
            khr_storage_buffer_storage_class: true,
            ..DeviceExtensions::none()
        };
        Device::new(
            device,
            &Features::none(),
            &deviceExtensions,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("Failed to create device")
    };
    println!("Created device {:?} with queues", device);
    let queue = queues.next().unwrap();

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data)
        .expect("failed to create buffer");

    let source_content = 0..64;
    let source =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
            .expect("failed to create buffer");

    let dest_content = (0..64).map(|_| 0);
    let dest =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, dest_content)
            .expect("failed to create buffer");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .copy_buffer(source.clone(), dest.clone())
        .unwrap()
        .build()
        .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let read_dest_buffer = dest.read().unwrap();
    println!("Command finished; destination {:?}", &*read_dest_buffer);

    let data_iter = 0..65536;
    let data_buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, data_iter)
            .expect("failed to create buffer");

    let shader = cs::Shader::load(device.clone()).expect("failed to create shaader module");
    println!("Shader is done");

    let compute_pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
            .expect("failed to create compute pipeline"),
    );
    println!("Compute pipeline created");

    let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_buffer(data_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ())
        .unwrap()
        .build()
        .unwrap();
    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything succeeded!");

    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue.family()),
    )
    .unwrap();
    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0]))
        .unwrap()
        .build()
        .unwrap();

    //make_fractal(device.clone(), queue.clone());
    graphics_pipeline(device.clone(), queue.clone());
}
