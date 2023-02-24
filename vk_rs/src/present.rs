
use core::panicking::panic;
use std::borrow::Cow;
use std::error::Error;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::iter::{Map, once};
use std::{mem, ptr, slice};
use std::slice::Iter;
use std::sync::Arc;

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use ash::extensions::ext::DebugUtils;
use ash::vk::{DebugUtilsMessengerEXT, PhysicalDevice};
use winit::{
    event::{Event, DeviceEvent as DE, WindowEvent as WE},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::window::Window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use smallvec::{smallvec, SmallVec};
use winit::event::VirtualKeyCode::N;
use winit::platform::run_return::EventLoopExtRunReturn;
use crate::vk_proc::VKProc;


#[cfg(target_os = "windows")]
fn create_vk_surface<Handle: HasRawWindowHandle + HasRawDisplayHandle>(
    entry: &Entry,
    window: &Handle,
    instance: &Instance,
) -> VkResult<vk::SurfaceKHR>  {
    match window.raw_window_handle() {
        RawWindowHandle::Win32(handle) => unsafe {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
                .hinstance(handle.hinstance)
                .hwnd(handle.hwnd);
            let surface_fn = khr::Win32Surface::new(entry, instance);
            surface_fn.create_win32_surface(&surface_desc, None)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

struct SurfaceE {
    capabilities: vk::SurfaceCapabilitiesKHR,
    presents: Vec<vk::PresentModeKHR>,
    formats: Vec<vk::SurfaceFormatKHR>,
    surface: vk::SurfaceKHR,
}

impl SurfaceE {
    fn new(vkproc: &VKProc, physical_device: &PhysicalDevice) -> Self {
        unsafe {
            let prop = Self  {
                capabilities: mem::zeroed(),
                presents: mem::zeroed(),
                formats: mem::zeroed(),
                // surface: create_vk_surface()
                surface: mem::zeroed()
            };
            prop.update(vkproc, physical_device)
        }
    }

    unsafe fn update(mut self, vkproc: &VKProc, physical_device: &PhysicalDevice) -> Self {
        self.formats = vkproc.surface.get_physical_device_surface_formats(*physical_device, self.surface).unwrap();
        self.presents = vkproc.surface.get_physical_device_surface_present_modes(*physical_device, self.surface).unwrap();
        self.capabilities = vkproc.surface.get_physical_device_surface_capabilities(*physical_device, self.surface).unwrap();
        self
    }
}


// fn create_swapchain(
//     swapchain_proc : &khr::Swapchain,
//     surface: &vk::SurfaceKHR,
//     device_properties: &SurfaceE
// ) -> vk::SwapchainKHR {
//
//     let present = device_properties.surface_present
//         .iter()
//         .max_by_key(|x| match **x {
//             vk::PresentModeKHR::MAILBOX => 2,
//             vk::PresentModeKHR::FIFO => 1,
//             _ => 0
//         })
//         .unwrap()
//         .clone();
//
//     let extent = device_properties.surface_capabilities.current_extent;
//     let format = device_properties.surface_formats[0];
//     let image_count = device_properties.surface_capabilities.min_image_count+1;
//     let transform = device_properties.surface_capabilities.current_transform;
//
//     let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
//         .surface(*surface)
//         .min_image_count(image_count)
//         .image_color_space(format.color_space)
//         .image_format(format.format)
//         .image_extent(extent)
//         .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
//         .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
//         .pre_transform(transform)
//         .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
//         .present_mode(present)
//         .clipped(true)
//         .image_array_layers(1);
//     println!("{:?}", extent);
//
//     unsafe {
//         swapchain_proc.create_swapchain(&swapchain_create_info, None).expect("create swapchain")
//     }
// }
