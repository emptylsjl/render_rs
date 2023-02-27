
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::iter::{Map, once};
use std::{mem, ptr, slice};

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use ash::extensions::khr::Swapchain;
use ash::vk::{Extent2D, Format, Framebuffer, Image, ImageView, PhysicalDevice, PresentModeKHR, RenderPass, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR, SwapchainKHR, Window};
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
use crate::vk_proc::proc::VKProc;

pub struct VKWindow<'a> {
    pub vkproc: &'a VKProc,
    pub window: window::Window,
    pub surface: SurfaceKHR,
    pub prop: SurfaceProperty,
}

impl<'a> VKWindow<'a> {

    pub fn new(
        vkproc: &'a VKProc,
        physical_device: &PhysicalDevice,
        window: window::Window,
        surface: SurfaceKHR,
    ) -> Self {
        unsafe {
            let prop = SurfaceProperty::new(vkproc, physical_device, &surface);
            Self {
                prop,
                vkproc,
                window,
                surface,
            }
        }
    }

    pub fn get_format(&self) -> SurfaceFormatKHR {
        self.prop.formats[1]
    }
    pub fn get_extent(&self) -> Extent2D {
        self.prop.capabilities.current_extent
    }
    pub fn get_dim_window(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn update_prop(&mut self, physical_device: &PhysicalDevice) {
        self.prop.update(self.vkproc, physical_device, &self.surface)
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.vkproc.surface.destroy_surface(self.surface, None)
        }
    }
}

impl Drop for VKWindow<'_> {
    fn drop(&mut self) {
        println!("2");
        self.destroy();
    }
}

// fn create_vk_surface<Handle: HasRawWindowHandle + HasRawDisplayHandle>(
//     vkproc: &VKProc,
//     window: &Handle,
// ) -> VkResult<SurfaceKHR>  {
#[cfg(target_os = "windows")]
pub fn create_vk_surface(vkproc: &VKProc, window: &window::Window) -> VkResult<SurfaceKHR> {
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
    physical_device: &PhysicalDevice,
    surface: &SurfaceKHR
) -> (Vec<SurfaceFormatKHR>, Vec<PresentModeKHR>, SurfaceCapabilitiesKHR) {
    (
        vkproc.surface.get_physical_device_surface_formats(*physical_device, *surface).unwrap(),
        vkproc.surface.get_physical_device_surface_present_modes(*physical_device, *surface).unwrap(),
        vkproc.surface.get_physical_device_surface_capabilities(*physical_device, *surface).unwrap(),
    )
}

#[derive(Clone, Default)]
pub struct SurfaceProperty {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub presents: Vec<PresentModeKHR>,
    pub formats: Vec<SurfaceFormatKHR>,
}

impl SurfaceProperty {
    pub fn new(vkproc: &VKProc, physical_device: &PhysicalDevice, surface: &SurfaceKHR) -> Self {
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

    pub fn update(&mut self, vkproc: &VKProc, physical_device: &PhysicalDevice, surface: &SurfaceKHR) {
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

pub fn create_swapchain(
    window: &VKWindow,
    swapchain_proc: &Swapchain,
) -> SwapchainKHR {

    let extent = window.prop.capabilities.current_extent;
    let format = window.prop.formats[1];
    let image_count = window.prop.capabilities.min_image_count+1;
    let transform = window.prop.capabilities.current_transform;
    println!("{transform:?}");
    let present = window.prop.presents
        .iter()
        .max_by_key(|&&x| match x {
            PresentModeKHR::MAILBOX => 2,
            PresentModeKHR::FIFO => 1,
            _ => 0
        })
        .unwrap();

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
        .surface(window.surface)
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
        swapchain_proc
            .create_swapchain(&swapchain_create_info, None)
            .expect("create swapchain")
    }
}

pub struct VKPresent<'a> {
    pub device: &'a VKDevice<'a>,
    pub images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    pub framebuffers: Vec<Framebuffer>,
    pub swapchain: SwapchainKHR,
    pub proc: Swapchain,
    pub window: VKWindow<'a>,
}

pub fn create_image_views(
    device: &VKDevice,
    window: &VKWindow,
    proc: &Swapchain,
    swapchain: &SwapchainKHR,
) -> (Vec<Image>, Vec<ImageView>) {

    let swapchain_image = unsafe {
        proc.get_swapchain_images(*swapchain).expect("create swapchain image")
    };
    let image_views = swapchain_image
        .iter()
        .map(|&image| {
            let create_view_info = vk::ImageViewCreateInfo::default()
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(window.get_format().format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
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
        })
        .collect::<Vec<ImageView>>();
    (swapchain_image, image_views)
}

fn create_frame_buffer(
    device: &VKDevice,
    render_pass: &RenderPass,
    extent: &Extent2D,
    image_views: &Vec<ImageView>,
) -> Vec<Framebuffer> {

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

impl<'a> VKPresent<'a> {

    pub fn new(
        vkproc: &VKProc,
        device: &'a VKDevice<'_>,
        window: VKWindow<'a>,
        render_pass: &RenderPass,
    ) -> Self {
        let proc = Swapchain::new(&vkproc.instance, &device.device);
        let swapchain = create_swapchain(&window, &proc);
        let (images, image_views) = create_image_views(device, &window, &proc, &swapchain);
        let framebuffers = create_frame_buffer(device, render_pass, &window.get_extent(), &image_views);

        Self {
            device,
            window,
            swapchain,
            images,
            image_views,
            framebuffers,
            proc,
        }
    }

    pub fn update_image_views(&mut self) {
        let (images, image_views) = create_image_views(
            &self.device,
            &self.window,
            &self.proc,
            &self.swapchain
        );
        self.images = images;
        self.image_views = image_views;
    }

    pub fn update_framebuffer(&mut self, render_pass: &RenderPass) {
        self.framebuffers = create_frame_buffer(&self.device, render_pass, &self.window.get_extent(), &self.image_views);
    }

    pub fn update_swapchain(&mut self) {
        self.swapchain = create_swapchain(&self.window, &self.proc);
    }

    pub fn recreate_swapchain(&mut self, render_pass: &RenderPass) {
        self.destroy();
        self.window.update_prop(self.device.physical_device());
        self.update_swapchain();
        self.update_image_views();
        self.update_framebuffer(render_pass);
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.device.device.device_wait_idle().expect("wait idle");
            self.image_views.iter().for_each(|&image| self.device.device.destroy_image_view(image, None));
            self.framebuffers.iter().for_each( |&frame| self.device.device.destroy_framebuffer(frame, None));
            self.proc.destroy_swapchain(self.swapchain, None);
        }
    }
}

impl<'a> Drop for VKPresent<'a> {
    fn drop(&mut self) {
        self.destroy();
    }
}


