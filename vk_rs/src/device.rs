
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::{mem, ptr, slice};
use std::mem::MaybeUninit;

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use ash::vk::PhysicalDevice;
use smallvec::{smallvec, SmallVec};
use crate::vk_proc::proc::{char2s, VKProc};


pub struct VKDevice<'a> {
    vkproc: &'a VKProc,
    pub props: VKDeviceProperty,
    pub graphic_queue: GraphicQueue,
    pub device: Device,
}

#[derive(Clone, Default)]
pub struct VKDeviceProperty {
    pub physical_device: PhysicalDevice,
    queues: Vec<vk::QueueFamilyProperties>,
    features: vk::PhysicalDeviceFeatures,
    properties: vk::PhysicalDeviceProperties,
    layers_properties: Vec<vk::LayerProperties>,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    extensions_properties: Vec<vk::ExtensionProperties>,
}

#[derive(Clone, Default)]
pub struct GraphicQueue {
    pub index: u32,
    pub queue: vk::Queue
}

pub fn find_present_queue(
    vkproc: &VKProc,
    surface: &vk::SurfaceKHR,
    device_property: &VKDeviceProperty,
) -> Option<u32> {
    device_property.queues.iter().enumerate()
        .find(|(i, queue)| unsafe {
            vkproc.surface
                .get_physical_device_surface_support(device_property.physical_device, *i as u32, *surface)
                .expect("get surface support") &&
                queue.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        })
        .map(|x| {
            x.0 as u32
        })
}

impl<'a> VKDevice<'a> {

    pub fn new(vkproc: &'a VKProc, surface: &vk::SurfaceKHR) -> Self {
        unsafe {
            // let devices = enumerate_device(vkproc).collect::<Vec<MaybeUninit<VKDevice>>>();
            // let a = devices.into_iter().filter_map(|device| {
            //     (&*device.as_ptr()).present_queue(surface)
            // });
            let (props, queue_index) = enumerate_device(vkproc)
                .into_iter()
                .filter_map(|device_property| {
                    find_present_queue(vkproc, surface, &device_property).map(|x| (device_property, x))
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
            let (device, graphic_queue) = Self::create_graphical_device(vkproc, props.physical_device, queue_index);
            Self {
                props,
                vkproc,
                device,
                graphic_queue,
            }
        }
    }

    pub fn create_graphical_device(
        vkproc: &VKProc,
        physical_device: PhysicalDevice,
        index: u32
    ) -> (Device, GraphicQueue) {

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(index)
            .queue_priorities(&[1.0]);

        let features = vk::PhysicalDeviceFeatures::default();
        let extensions = vec![khr::Swapchain::name().as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(slice::from_ref(&queue_info))
            .enabled_extension_names(extensions.as_slice())
            .enabled_features(&features);

        unsafe {
            let device = vkproc.instance
                .create_device(physical_device, &device_create_info, None)
                .expect("create device");
            let graphical_queue = GraphicQueue {
                index,
                queue: device.get_device_queue(index, 0),
            };
            (device, graphical_queue)
        }
    }

    pub fn name(&self) -> String {
        char2s(&self.props.properties.device_name)
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.props.physical_device
    }
}

impl Drop for VKDevice<'_> {
    fn drop(&mut self) {

        println!("3");
        unsafe { self.device.destroy_device(None); }
    }
}

pub fn enumerate_device(vkproc: &VKProc) -> impl Iterator<Item=VKDeviceProperty> + '_ {
    unsafe {
        vkproc.instance.enumerate_physical_devices().expect("enumerate physical devices")
            .into_iter()
            .map(|physical_device|
                VKDeviceProperty {
                    physical_device,
                    features: vkproc.instance.get_physical_device_features(physical_device),
                    properties: vkproc.instance.get_physical_device_properties(physical_device),
                    queues: vkproc.instance.get_physical_device_queue_family_properties(physical_device),
                    memory_properties: vkproc.instance.get_physical_device_memory_properties(physical_device),
                    layers_properties: vkproc.instance.enumerate_device_layer_properties(physical_device).unwrap_or_default(),
                    extensions_properties: vkproc.instance.enumerate_device_extension_properties(physical_device).unwrap_or_default(),
                }
            )
    }
}
