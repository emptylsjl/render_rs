
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::iter::{Map, once};
use std::{mem, ptr, slice};

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use itertools::Itertools;
use winit::{
    event::{Event, DeviceEvent as DE, WindowEvent as WE},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use smallvec::{smallvec, SmallVec};
use windows::s;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode::N;
use winit::platform::run_return::EventLoopExtRunReturn;
use crate::device::VKDevice;
use crate::vk_proc::dvk::*;
use crate::vk_proc::proc::VKProc;


pub fn create_swapchain(
    swapchain_proc: &khr::Swapchain,
    surface: vk::SurfaceKHR,
    prop: &SurfaceProperty
) -> vk::SwapchainKHR {

    let extent = prop.capabilities.current_extent;
    let format = prop.formats[1];
    let image_count = prop.capabilities.min_image_count+1;
    let transform = prop.capabilities.current_transform;

    let present = prop.presents
        .iter()
        .max_by_key(|&&x| match x {
            VK_PRESENT_MODE_MAILBOX_KHR => 2,
            VK_PRESENT_MODE_FIFO_KHR => 1,
            _ => 0
        })
        .unwrap();

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
        .surface(surface)
        .min_image_count(image_count)
        .image_color_space(format.color_space)
        .image_format(format.format)
        .image_extent(extent)
        .image_usage(VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT)
        .image_sharing_mode(VK_SHARING_MODE_EXCLUSIVE)
        .pre_transform(transform)
        .composite_alpha(VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR)
        .present_mode(*present)
        .clipped(true)
        .image_array_layers(1);

    unsafe {
        swapchain_proc
            .create_swapchain(&swapchain_create_info, None)
            .expect("create swapchain")
    }
}

pub fn create_image_views(
    device: &VKDevice,
    format: vk::Format,
    image: vk::Image,
) -> vk::ImageView {

    let create_view_info = vk::ImageViewCreateInfo::default()
        .view_type(VK_IMAGE_VIEW_TYPE_2D)
        .format(format)
        .components(vk::ComponentMapping {
            r: VK_COMPONENT_SWIZZLE_IDENTITY,
            g: VK_COMPONENT_SWIZZLE_IDENTITY,
            b: VK_COMPONENT_SWIZZLE_IDENTITY,
            a: VK_COMPONENT_SWIZZLE_IDENTITY,
        })
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: VK_IMAGE_ASPECT_COLOR_BIT,
            base_array_layer: 0,
            base_mip_level: 0,
            level_count: 1,
            layer_count: 1,
        })
        .image(image);

    unsafe {
        device.device
            .create_image_view(&create_view_info, None)
            .expect("create image view")
    }
}

fn create_frame_buffer(
    device: &VKDevice,
    render_pass: &vk::RenderPass,
    image_views: &Vec<vk::ImageView>,
    extent: vk::Extent2D,
) -> Vec<vk::Framebuffer> {

    image_views.iter().map(|&image_view| {
        let attachments = [image_view];
        let frame_buffer_create_info = vk::FramebufferCreateInfo::default()
            .render_pass(*render_pass)
            .attachments(&attachments)
            .width(extent.width)
            .height(extent.height)
            .layers(1);

        unsafe {
            device.device
                .create_framebuffer(&frame_buffer_create_info, None)
                .expect("create frame buffer")
        }
    }).collect()
}

pub struct VKFramebuffer<'a> {
    device: &'a VKDevice<'a>,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl<'a> VKFramebuffer<'a> {

    pub fn new(
        device: &'a VKDevice,
        window: &VKWindow,
        render_pass: &vk::RenderPass,
    ) -> VKFramebuffer<'a> {
        let images = window.get_swapchain_images();
        let image_views = images.iter().map(|&image|
            create_image_views(device, window.get_format().format, image,)
        ).collect();
        let framebuffers = create_frame_buffer(
            device,
            render_pass,
            &image_views,
            window.get_extent());
        Self {
            device,
            images,
            image_views,
            framebuffers,
        }
    }

    pub fn update_image_views(&mut self, window: &VKWindow) {
        let images = window.get_swapchain_images();
        let image_views = images.iter().map(|&image|
            create_image_views(self.device, window.get_format().format, image)
        ).collect();

        self.images = images;
        self.image_views = image_views;
    }

    pub fn update_framebuffer(&mut self, window: &VKWindow, render_pass: &vk::RenderPass, ) {
        optick::event!();
        self.framebuffers = create_frame_buffer(
            &self.device,
            render_pass,
            &self.image_views,
            window.get_extent()
        );
    }

    pub fn index(&self, i: u32) -> vk::Framebuffer {
        self.framebuffers[i as usize]
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.device.device_wait_idle().expect("wait idle");
            self.image_views.iter().for_each(|&image| self.device.device.destroy_image_view(image, None));
            self.framebuffers.iter().for_each( |&frame| self.device.device.destroy_framebuffer(frame, None));
        }
    }
}

impl Drop for VKFramebuffer<'_> {
    fn drop(&mut self) {
        self.destroy();
    }
}

pub struct VKWindow<'a> {
    pub vkproc: &'a VKProc,
    // pub device: &'a VKDevice<'a>,
    pub window: window::Window,
    pub surface: vk::SurfaceKHR,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_proc: khr::Swapchain,
    pub prop: SurfaceProperty,
}

impl<'a> VKWindow<'a> {

    pub fn new(
        vkproc: &'a VKProc,
        device: &VKDevice,
        window: window::Window,
        surface: vk::SurfaceKHR,
    ) -> Self {
        optick::event!();
        unsafe {
            let prop = SurfaceProperty::new(vkproc, &device.physical_device(), &surface);
            let swapchain_proc = khr::Swapchain::new(&vkproc.instance, &device.device);
            let swapchain = create_swapchain(&swapchain_proc, surface, &prop);
            Self {
                prop,
                // device,
                vkproc,
                window,
                surface,
                swapchain,
                swapchain_proc,
            }
        }
    }

    pub fn update_swapchain(&mut self) {
        optick::event!();
        unsafe {
            self.swapchain_proc.destroy_swapchain(self.swapchain, None);
        }
        self.swapchain = create_swapchain(&self.swapchain_proc, self.surface, &self.prop);
    }

    pub fn recreate_swapchain(
        &mut self,
        framebuffer: &mut VKFramebuffer,
        render_pass: &vk::RenderPass
    ) {
        optick::event!();
        framebuffer.destroy();
        self.update_prop(framebuffer.device.physical_device());
        self.update_swapchain();
        framebuffer.update_image_views(self);
        framebuffer.update_framebuffer(self, render_pass);
    }

    // pub fn recreate_swapchain_from_physical_size(&mut self, render_pass: &RenderPass) {
    //     self.destroy();
    //     let size = self.window.get_dim_window();
    //     let extent = Extent2D::default();
    //     self.update_swapchain();
    //     self.update_image_views();
    //     self.update_framebuffer(render_pass);
    // }

    pub fn get_swapchain_images(&self) -> Vec<vk::Image> {
        unsafe {
            self.swapchain_proc
                .get_swapchain_images(self.swapchain)
                .expect("create swapchain image")
        }
    }
    pub fn get_format(&self) -> vk::SurfaceFormatKHR {
        self.prop.formats[1]
    }

    pub fn get_extent(&self) -> vk::Extent2D {
        self.prop.capabilities.current_extent
    }

    pub fn get_dim_window(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn update_prop(&mut self, physical_device: &vk::PhysicalDevice) {
        self.prop.update(self.vkproc, physical_device, &self.surface)
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.swapchain_proc.destroy_swapchain(self.swapchain, None);
            self.vkproc.surface.destroy_surface(self.surface, None)
        }
    }
}

impl Drop for VKWindow<'_> {
    fn drop(&mut self) {
        println!("win");
        self.destroy();
    }
}

// fn create_vk_surface<Handle: HasRawWindowHandle + HasRawDisplayHandle>(
//     vkproc: &VKProc,
//     window: &Handle,
// ) -> VkResult<SurfaceKHR>  {
#[cfg(target_os = "windows")]
pub fn create_vk_surface(vkproc: &VKProc, window: &window::Window) -> VkResult<vk::SurfaceKHR> {
    optick::event!();
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

pub unsafe fn get_surface_prop(
    vkproc: &VKProc,
    physical_device: &vk::PhysicalDevice,
    surface: &vk::SurfaceKHR
) -> (Vec<vk::SurfaceFormatKHR>, Vec<vk::PresentModeKHR>, vk::SurfaceCapabilitiesKHR) {
    (
        vkproc.surface.get_physical_device_surface_formats(*physical_device, *surface).unwrap(),
        vkproc.surface.get_physical_device_surface_present_modes(*physical_device, *surface).unwrap(),
        vkproc.surface.get_physical_device_surface_capabilities(*physical_device, *surface).unwrap(),
    )
}

#[derive(Clone, Default, Debug)]
pub struct SurfaceProperty {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub presents: Vec<vk::PresentModeKHR>,
    pub formats: Vec<vk::SurfaceFormatKHR>,
}

impl SurfaceProperty {
    pub fn new(vkproc: &VKProc, physical_device: &vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> Self {
        unsafe {
            let (
                formats,
                presents,
                capabilities
            ) = get_surface_prop(vkproc, physical_device, surface);

            Self  {
                formats,
                presents,
                capabilities
            }
        }
    }

    pub fn update(&mut self, vkproc: &VKProc, physical_device: &vk::PhysicalDevice, surface: &vk::SurfaceKHR) {
        unsafe {
            let (
                formats,
                presents,
                capabilities
            ) = get_surface_prop(vkproc, physical_device, surface);
            self.formats = formats;
            self.presents = presents;
            self.capabilities = capabilities;
        }
    }

}




