#![feature(cstr_from_bytes_until_nul)]
#![allow(clippy::single_match)]

mod define;

use std::borrow::Cow;
use std::error::Error;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::iter::once;
use std::{ptr, slice};
use std::sync::Arc;

use ash::{Device, Entry, Instance, vk};
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr;
use ash::extensions::khr::{Surface, Swapchain};
use ash::prelude::VkResult;
use ash::vk::{LayerProperties, PhysicalDeviceFeatures, PresentModeKHR, SurfaceKHR, SwapchainKHR};
use winit::{
    event::{Event::*, WindowEvent as WE, DeviceEvent as DE},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::window::Window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use smallvec::{smallvec, SmallVec};

fn chars(s: &[u8]) -> &CStr {
    CStr::from_bytes_with_nul(s).unwrap()
}

unsafe extern "system" fn debug_callback(
    msg_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    msg_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let data = *callback_data;
    let name = CStr::from_ptr(data.p_message_id_name);
    let message = CStr::from_ptr(data.p_message);
    // let severity = msg_severity.fmt()

    println!("\
        {msg_severity:<15?}{msg_type:<15?} - {name:?}\n\
        {message:?}\n\
    ");
    vk::FALSE
}

fn create_instance(entry: &Entry) -> Result<Arc<Instance>, Box<dyn Error>> {

    let layer_names = entry.enumerate_instance_layer_properties()?;
    let extension_names = entry.enumerate_instance_extension_properties(Some(chars(b"\0")))?;

    let required_extension = vec![
        ash::extensions::khr::Surface::name().as_ptr(),
        ash::extensions::khr::Win32Surface::name().as_ptr(),
        ash::extensions::khr::GetSurfaceCapabilities2::name().as_ptr(),
        ash::extensions::khr::GetPhysicalDeviceProperties2::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ];

    let app_name = chars(b"VkTri\0");
    let required_layers = [
        chars(b"VK_LAYER_KHRONOS_validation\0").as_ptr()
    ];

    // assert!(
    //     required_extension.iter().all(|x| extension_names.contains(x)),
    //     "required extension not present"
    // );
    // assert!(
    //     required_layers.iter().all(|x| layer_names.contains(x)),
    //     "required layer not present"
    // );
    // let a = extension_names.contains(&required_extension[0]);
    // let b = layer_names.contains(&required_layers[0]);


    let appinfo = vk::ApplicationInfo::default()
        .application_name(app_name)
        .application_version(0)
        .engine_name(app_name)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));

    let instance_info = vk::InstanceCreateInfo::default()
        .application_info(&appinfo)
        .enabled_layer_names(&required_layers)
        .enabled_extension_names(&required_extension)
        .flags(vk::InstanceCreateFlags::default());

    Ok(Arc::new(
        unsafe { entry.create_instance(&instance_info, None)? }
    ))
}

fn debug_init(entry: &Entry, instance: &Instance) -> vk::DebugUtilsMessengerEXT {
    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(debug_callback));

    let debug_loader = DebugUtils::new(&entry, &instance);
    unsafe {
        debug_loader
            .create_debug_utils_messenger(&debug_info, None)
            .unwrap()
    }
}

#[cfg(target_os = "windows")]
fn vk_surface<Handle: HasRawWindowHandle + HasRawDisplayHandle>(
    entry: &Entry,
    window: &Handle,
    instance: &Instance,
) -> VkResult<vk::SurfaceKHR>  {
    match window.raw_window_handle() {
        RawWindowHandle::Win32(handle) => unsafe {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
                .hinstance(handle.hinstance)
                .hwnd(handle.hwnd);
            let surface_fn = ash::extensions::khr::Win32Surface::new(entry, instance);
            surface_fn.create_win32_surface(&surface_desc, None)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

struct VKPhysicalDeviceProperties {
    queues: Vec<vk::QueueFamilyProperties>,
    features: vk::PhysicalDeviceFeatures,
    properties: vk::PhysicalDeviceProperties,
    physical_device: vk::PhysicalDevice,
    layers_properties: Vec<vk::LayerProperties>,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    extensions_properties: Vec<vk::ExtensionProperties>,
    surface_present: Vec<PresentModeKHR>,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    graphic_index: u32
}

fn get_device(instance: &Instance, surface: &SurfaceKHR, surface_handle: &Surface) -> VKPhysicalDeviceProperties {
    unsafe {
        instance.enumerate_physical_devices().unwrap()
            .into_iter()
            .map(|physical_device| {
                let queues = instance.get_physical_device_queue_family_properties(physical_device);
                let graphic_index = queues.iter().enumerate()
                    .find(|(i, queue)| {
                        surface_handle.get_physical_device_surface_support(physical_device, *i as u32, *surface).unwrap() &&
                        queue.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    }).unwrap().0 as u32;
                VKPhysicalDeviceProperties {
                    queues,
                    graphic_index,
                    physical_device,
                    features: instance.get_physical_device_features(physical_device),
                    properties: instance.get_physical_device_properties(physical_device),
                    layers_properties: instance.enumerate_device_layer_properties(physical_device).unwrap(),
                    memory_properties: instance.get_physical_device_memory_properties(physical_device),
                    extensions_properties: instance.enumerate_device_extension_properties(physical_device).unwrap(),
                    surface_formats: surface_handle.get_physical_device_surface_formats(physical_device, *surface).unwrap(),
                    surface_present: surface_handle.get_physical_device_surface_present_modes(physical_device, *surface).unwrap(),
                    surface_capabilities: surface_handle.get_physical_device_surface_capabilities(physical_device, *surface).unwrap()
                }
            })
            .max_by_key(|device| match device.properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => 5,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 4,
                vk::PhysicalDeviceType::VIRTUAL_GPU => 3,
                vk::PhysicalDeviceType::CPU => 2,
                vk::PhysicalDeviceType::OTHER => 1,
                _ => 0
            })
            .unwrap()
    }
}

fn create_device(instance: &Instance, device_properties: &VKPhysicalDeviceProperties, extensions: &Vec<*const c_char>, features: &PhysicalDeviceFeatures) -> Device {
    assert!(device_properties.surface_capabilities.min_image_count > 0, "surface incapable of image displacement");
    // assert!()
    let queue_info = vk::DeviceQueueCreateInfo::default()
        .queue_family_index(device_properties.graphic_index)
        .queue_priorities(&[1.0]);

    let device_create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(std::slice::from_ref(&queue_info))
        .enabled_extension_names(extensions.as_slice())
        .enabled_features(features);

    unsafe {
        instance
            .create_device(device_properties.physical_device, &device_create_info, None)
            .unwrap()
    }
}

fn create_swapchain(
    instance: &Instance,
    device: &Device,
    surface: &SurfaceKHR,
    device_properties: &VKPhysicalDeviceProperties) -> SwapchainKHR {

    let present = device_properties.surface_present
        .iter()
        .max_by_key(|x| match **x {
            PresentModeKHR::MAILBOX => 2,
            PresentModeKHR::FIFO => 1,
            _ => 0
        })
        .unwrap()
        .clone();

    let extent = device_properties.surface_capabilities.current_extent;
    let format = device_properties.surface_formats[0];
    let image_count = 2;
    let transform = device_properties.surface_capabilities.current_transform;

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
        .surface(*surface)
        .min_image_count(image_count)
        .image_color_space(format.color_space)
        .image_format(format.format)
        // .image_extent(extent)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present)
        .clipped(true)
        .image_array_layers(1);


    let swapchain_loader = Swapchain::new(&instance, &device);
    unsafe {
        swapchain_loader.create_swapchain(&swapchain_create_info, None).unwrap()
    }
}

fn main() {
    let entry = Entry::linked();

    let instance = create_instance(&entry).unwrap();
    let debug_messenger = debug_init(&entry, &instance);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 800))
        .build(&event_loop)
        .unwrap();

    let surface_handle = Surface::new(&entry, &instance);
    let surface = vk_surface(&entry, &window, &instance).unwrap();

    let device_properties = get_device(&instance, &surface, &surface_handle);


    let features = vk::PhysicalDeviceFeatures::default();
    let extensions = vec![khr::Swapchain::name().as_ptr()];
    let device = create_device(&instance, &device_properties, &extensions, &features);
    let queue = unsafe { device.get_device_queue(device_properties.graphic_index, 0) };

    let swapchain = create_swapchain(&instance, &device, &surface, &device_properties);



    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        if let WindowEvent { event: WE::CloseRequested, .. } = event {
            control_flow.set_exit()
        }

    //     match event {
    //         WindowEvent { event, window_id } => {
    //             match event {
    //                 WE::CloseRequested => control_flow.set_exit(),
    //                 WE::KeyboardInput { input, is_synthetic, .. } => {
    //                     println!("{input:?}, {is_synthetic:?}")
    //                 }
    //                 WE::MouseInput { state, button, .. } => {
    //                     println!("{button:?}, {state:?}")
    //                 }
    //                 WE::CursorMoved { position, .. } => {
    //                     println!("{position:?}")
    //                 }
    //                 _ => {}
    //             }
    //         },
    //         DeviceEvent {event, device_id} => {
    //             match event {
    //                 DE::MouseMotion { delta } => { println!("{delta:?}"); }
    //                 DE::Button { button, state } => { println!("{button:?}, {state:?}") }
    //                 _ => {}
    //             }
    //         }
    //         MainEventsCleared => {
    //             window.request_redraw();
    //             // println!("233");
    //         }
    //         _ => (),
    //     }
    });
}