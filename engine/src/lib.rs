#![allow(unused_imports)]
extern crate image as img;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate maplit;
extern crate imgui_sys as imgui;
extern crate itertools;
extern crate nalgebra_glm as glm;
extern crate ncollide3d as nc;
extern crate num_traits;
extern crate ordered_float as of;
extern crate rusttype;
extern crate specs;
extern crate specs_derive;
extern crate time as t;
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

pub mod camera;
pub mod drawing;
pub mod fps_counter;
pub mod image;
pub mod mesh;
pub mod shader;
pub mod time;
pub mod transform;
pub mod utils;
pub mod window;

use camera::*;
use drawing::*;
use glm::*;
use shader::*;
use specs::prelude::*;
use time::*;
use window::init_vulkano_window;

use std::sync::Arc;
use vulkano::{
    device::{Device, DeviceExtensions, Features},
    instance::PhysicalDevice,
    swapchain::{ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain},
    sync::SharingMode,
	format::FormatDesc
};
use vulkano_win::VkSurfaceBuild;
use winit::{event::WindowEvent, event_loop::ControlFlow, window::WindowBuilder};

fn viewport(_wl: i32, _h: i32) {
    unimplemented!();
}

fn clear(_a: f32, _b: f32, _c: f32, _d: f32) {
    unimplemented!();
}

pub fn start_event_loop() {
    let window_size = (800, 600);
    let (events_loop, instance) = init_vulkano_window(window_size);
    let surface = WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();

    let device = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    let caps = surface
        .capabilities(device.clone())
        .expect("failed to get surface capabilities");

    let queue_family = device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("coulnd't find a graphical queue family");

    let (device, mut queues) = {
        let deviceExtensions = DeviceExtensions {
			khr_swapchain: true,
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

    let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        caps.min_image_count,
        format,
        dimensions,
        1,
        caps.supported_usage_flags,
        SharingMode::Exclusive,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        FullscreenExclusive::Default,
        false,
        ColorSpace::SrgbNonLinear,
    )
    .expect("failed to create swapchain");

    events_loop.run(
        |event, _event_loop_window_target, control_flow| match event {
            winit::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,
            _ => (),
        },
    );
}
