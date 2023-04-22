
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::{mem, ptr, slice};
use std::mem::MaybeUninit;

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use glam::Mat4;
use num_traits::Float;
use smallvec::{smallvec, SmallVec};
use crate::define::*;
use crate::vertex::camera;
use crate::vk_proc::proc::{char2s, VKProc};


pub struct VKDevice<'a> {
    vkproc: &'a VKProc,
    pub props: VKDeviceProperty,
    pub graphic_queue: GraphicQueue,
    pub device: Device,
}

#[derive(Clone, Default)]
pub struct VKDeviceProperty {
    physical_device: vk::PhysicalDevice,
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
        physical_device: vk::PhysicalDevice,
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

    pub fn find_memory_type(&self, memory_type_bits: u32, memory_type_flag: vk::MemoryPropertyFlags) -> Option<usize> {
        let memory_properties = self.props.memory_properties;

        memory_properties.memory_types[..memory_properties.memory_type_count as _]
            .iter()
            .enumerate()
            .position(|(i, memory_type)| {
                (((1 << i) & memory_type_bits) != 0) && ((memory_type.property_flags & memory_type_flag) == memory_type_flag)
            })
    }

    pub fn create_buffer(
        &self,
        buffer_size: u64,
        buffer_usage: vk::BufferUsageFlags,
        memory_type_flag: vk::MemoryPropertyFlags,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        unsafe {
            let buffer_create_info = vk::BufferCreateInfo::default()
                .size(buffer_size)
                .usage(buffer_usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            let buffer = self.device
                .create_buffer(&buffer_create_info, None)
                .expect("create buffer");
            let buffer_memory_requirements = self.device
                .get_buffer_memory_requirements(buffer);

            let Some(memory_type_index) = self.find_memory_type(
                buffer_memory_requirements.memory_type_bits,
                memory_type_flag
            ) else {
                panic!("find memory_type")
            };

            let vertex_buffer_allocate_info = vk::MemoryAllocateInfo::default()
                .allocation_size(buffer_memory_requirements.size)
                .memory_type_index(memory_type_index as u32);

            let device_memory = self.device
                .allocate_memory(&vertex_buffer_allocate_info, None)
                .expect("allocate memory");
            self.device
                .bind_buffer_memory(buffer, device_memory, 0)
                .expect("bind memory");

            (buffer, device_memory)
        }
    }

    pub fn map_memory<T: Copy + Sized>(
        &self,
        src_slice: &[T],
        // memory_size: usize,
        device_memory: &vk::DeviceMemory,
        unmap: bool
    ) -> Option<&'a mut [T]> {
        unsafe {
            let device_memory_ptr = self.device
                .map_memory(*device_memory, 0, (mem::size_of::<T>() * src_slice.len()) as _, vk::MemoryMapFlags::empty())
                .expect("map vertex memory") as *mut T;
            src_slice.as_ptr().copy_to_nonoverlapping(device_memory_ptr, src_slice.len());

            if unmap {
                self.device.unmap_memory(*device_memory);
                None
            } else {
                Some(slice::from_raw_parts_mut(device_memory_ptr, src_slice.len()))
            }
        }
    }


    pub fn copy_memory(
        &self,
        queue: &vk::Queue,
        src_buffer: &vk::Buffer,
        dst_buffer: &vk::Buffer,
        buffer_size: vk::DeviceSize,
        command_pool: &vk::CommandPool,
    ) {
        let temp_command_buffers = self.command_buffer_allocate(command_pool, 1);
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device
                .begin_command_buffer(temp_command_buffers[0], &command_buffer_begin_info)
                .expect("begin Command Buffer");

            let buffer_copy_regions = [
                vk::BufferCopy::default()
                    .size(buffer_size)
            ];

            self.device.cmd_copy_buffer(temp_command_buffers[0], *src_buffer, *dst_buffer, &buffer_copy_regions);

            self.device
                .end_command_buffer(temp_command_buffers[0])
                .expect("Failed to end Command Buffer");

            let submit_info = [
                vk::SubmitInfo::default()
                    .command_buffers(&temp_command_buffers)
            ];
            self.device
                .queue_submit(*queue, &submit_info, vk::Fence::null())
                .expect("Failed to Submit Queue.");
            self.device
                .queue_wait_idle(*queue)
                .expect("Failed to wait Queue idle");

            self.device.free_command_buffers(*command_pool, &temp_command_buffers);
        }
    }

    pub fn update_uniform_buffer<>(&self, mapped_memory: &mut [Mat4], x: f32, y: f32) {

        // mapped_memory[0] = camera([x, y, ]);
    }

    pub fn create_command_pool(&self, queue_family_index: u32) -> vk::CommandPool {

        let pool_create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_index);

        unsafe {
            self.device
            .create_command_pool(&pool_create_info, None)
            .expect("create command pool")
        }
    }

    pub fn command_buffer_allocate(&self, command_pool: &vk::CommandPool, buffer_count: u32) -> Vec<vk::CommandBuffer> {

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(buffer_count)
            .command_pool(*command_pool);

        unsafe {
            self.device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("create command buffer")
        }
    }

    pub fn name(&self) -> String {
        char2s(&self.props.properties.device_name)
    }

    pub fn physical_device(&self) -> &vk::PhysicalDevice {
        &self.props.physical_device
    }

    pub fn memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties { &self.props.memory_properties }
}

impl Drop for VKDevice<'_> {
    fn drop(&mut self) {

        println!("device");
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
