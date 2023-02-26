
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
use winit::{
    event::{Event, DeviceEvent as DE, WindowEvent as WE},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use smallvec::{smallvec, SmallVec};
use winit::event::VirtualKeyCode::N;
use winit::platform::run_return::EventLoopExtRunReturn;
use crate::device::VKDevice;
use crate::vk_proc::VKProc;

pub struct VKWindow<'a> {
    vkproc: &'a VKProc,
    window: window::Window,
    surface: vk::SurfaceKHR,
    swapchain: vk::SwapchainKHR,
    prop: SurfaceProperty,
}

impl<'a> VKWindow<'a> {

    pub fn new(
        vkproc: &'a VKProc,
        window: window::Window,
        surface: vk::SurfaceKHR,
        device: &VKDevice,
    ) -> Self {
        unsafe {
            let vk_window = Self {
                vkproc,
                window,
                surface,
                prop: SurfaceProperty::new(vkproc, &device.physical_device, &surface),
                swapchain: mem::zeroed(),
            };
            vk_window.create_swapchain()
        }
    }

    pub fn create_swapchain(mut self) -> Self {

        let extent = self.prop.capabilities.current_extent;
        let format = self.prop.formats[0];
        let image_count = self.prop.capabilities.min_image_count+1;
        let transform = self.prop.capabilities.current_transform;
        let present = self.prop.presents
            .iter()
            .max_by_key(|&&x| match x {
                vk::PresentModeKHR::MAILBOX => 2,
                vk::PresentModeKHR::FIFO => 1,
                _ => 0
            })
            .unwrap();

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_color_space(format.color_space)
            .image_format(format.format)
            .image_extent(extent)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(*present)
            .clipped(true)
            .image_array_layers(1);

        unsafe {
            self.swapchain = self.vkproc.swapchain
                .create_swapchain(&swapchain_create_info, None)
                .expect("create swapchain");
            self
        }
    }

    pub fn recreate_swapchain() {

    }
}


// fn create_vk_surface<Handle: HasRawWindowHandle + HasRawDisplayHandle>(
//     vkproc: &VKProc,
//     window: &Handle,
// ) -> VkResult<vk::SurfaceKHR>  {
#[cfg(target_os = "windows")]
pub fn create_vk_surface(vkproc: &VKProc, window: &window::Window) -> VkResult<vk::SurfaceKHR> {
    match window.raw_window_handle() {
        RawWindowHandle::Win32(handle) => unsafe {
            let surface_fn = khr::Win32Surface::new(&vkproc.entry, &vkproc.instance);
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
                .hinstance(handle.hinstance)
                .hwnd(handle.hwnd);
            surface_fn.create_win32_surface(&surface_desc, None)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

pub struct SurfaceProperty {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub presents: Vec<vk::PresentModeKHR>,
    pub formats: Vec<vk::SurfaceFormatKHR>,
}

impl SurfaceProperty {
    pub fn new(vkproc: &VKProc, physical_device: &vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> Self {
        unsafe {
            let mut prop = Self  {
                capabilities: mem::zeroed(),
                presents: mem::zeroed(),
                formats: mem::zeroed(),
            };
            prop.update(vkproc, physical_device, surface);
            prop
        }
    }

    pub unsafe fn update(&mut self, vkproc: &VKProc, physical_device: &vk::PhysicalDevice, surface: &vk::SurfaceKHR) {
        self.formats = vkproc.surface.get_physical_device_surface_formats(*physical_device, *surface).unwrap();
        self.presents = vkproc.surface.get_physical_device_surface_present_modes(*physical_device, *surface).unwrap();
        self.capabilities = vkproc.surface.get_physical_device_surface_capabilities(*physical_device, *surface).unwrap();
    }
}


