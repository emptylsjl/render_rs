
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::{mem, ptr, slice};
use std::mem::MaybeUninit;

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use smallvec::{smallvec, SmallVec};
use crate::vk_proc::proc::{char2s, VKProc};


pub struct VKDevice<'a> {
    vkproc: &'a VKProc,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,
    queues: Vec<vk::QueueFamilyProperties>,
    features: vk::PhysicalDeviceFeatures,
    properties: vk::PhysicalDeviceProperties,
    layers_properties: Vec<vk::LayerProperties>,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    extensions_properties: Vec<vk::ExtensionProperties>,
    graphic_queue: GraphicQueue
}

struct GraphicQueue {
    index: u32,
    queue: vk::Queue
}

impl<'a> VKDevice<'a> {

    pub fn new(vkproc: &'a VKProc, surface: &vk::SurfaceKHR) -> Self {
        unsafe {
            // let devices = enumerate_device(vkproc).collect::<Vec<MaybeUninit<VKDevice>>>();
            // let a = devices.into_iter().filter_map(|device| {
            //     (&*device.as_ptr()).present_queue(surface)
            // });
            let (devices, queue_index) = enumerate_device(vkproc)
                .into_iter()
                .filter_map(|device| {
                    device.present_queue(surface).map(|x| (device, x))
                }).max_by_key(|(device, index)| match device.properties.device_type {
                    vk::PhysicalDeviceType::DISCRETE_GPU => 5,
                    vk::PhysicalDeviceType::INTEGRATED_GPU => 4,
                    vk::PhysicalDeviceType::VIRTUAL_GPU => 3,
                    vk::PhysicalDeviceType::CPU => 2,
                    vk::PhysicalDeviceType::OTHER => 1,
                    _ => 0
                })
                .expect("select device");

            // (*devices.as_mut_ptr()).graphic_index = index;
            // devices.assume_init()
            devices.create_graphical_device(queue_index)
        }
    }

    pub fn create_graphical_device(mut self, index: u32) -> Self {

        let queue_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(index)
            .queue_priorities(&[1.0]);

        let features = vk::PhysicalDeviceFeatures::default();
        let extensions = vec![khr::Swapchain::name().as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(slice::from_ref(&queue_info))
            .enabled_extension_names(extensions.as_slice())
            .enabled_features(&features);

        unsafe {
            let device = self.vkproc.instance
                .create_device(self.physical_device, &device_create_info, None)
                .expect("create device");
            self.graphic_queue = GraphicQueue {
                index,
                queue: device.get_device_queue(index, 0),
            };
            self
        }
    }

    pub fn present_queue(&self, surface: &vk::SurfaceKHR) -> Option<u32> {
        self.queues.iter().enumerate()
            .find(|(i, queue)| unsafe {
                self.vkproc.surface
                    .get_physical_device_surface_support(self.physical_device, *i as u32, *surface)
                    .expect("get surface support") &&
                queue.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            })
            .map(|x| {
                x.0 as u32
            })
    }

    pub fn device_name(&self) -> String {
        char2s(&self.properties.device_name)
    }
}

pub fn enumerate_device(vkproc: &VKProc) -> impl Iterator<Item=VKDevice> {
    unsafe {
        vkproc.instance.enumerate_physical_devices().expect("enumerate physical devices")
            .into_iter()
            .map(|physical_device| {
                // let mut x = MaybeUninit::<VKDevice>::uninit();
                // x.write(
                VKDevice {
                    vkproc,
                    physical_device,
                    device: mem::zeroed(),
                    graphic_queue: mem::zeroed(),
                    features: vkproc.instance.get_physical_device_features(physical_device),
                    properties: vkproc.instance.get_physical_device_properties(physical_device),
                    queues: vkproc.instance.get_physical_device_queue_family_properties(physical_device),
                    memory_properties: vkproc.instance.get_physical_device_memory_properties(physical_device),
                    layers_properties: vkproc.instance.enumerate_device_layer_properties(physical_device).unwrap(),
                    extensions_properties: vkproc.instance.enumerate_device_extension_properties(physical_device).unwrap(),
                }
                // );
                // x
            })
    }
}
