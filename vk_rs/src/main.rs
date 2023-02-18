#![feature(cstr_from_bytes_until_nul)]
#![allow(clippy::single_match)]
#![feature(core_panic)]

extern crate core;

use core::panicking::panic;
use std::borrow::Cow;
use std::error::Error;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::iter::{Map, once};
use std::{ptr, slice};
use std::slice::Iter;
use std::sync::Arc;

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use ash::extensions::ext::DebugUtils;
use ash::vk::DebugUtilsMessengerEXT;
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

mod define;
use define::*;

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
    let severity = format!("{msg_severity:?}");
    let mtype = format!("{msg_type:?}");

    println!("\
        {severity:<10} | {mtype:<10} | {name:?}\n\
        {message:?}\
    ");
    vk::FALSE
}

fn create_instance(entry: &Entry) -> Result<Arc<Instance>, Box<dyn Error>> {

    let layer_names = entry.enumerate_instance_layer_properties()?;
    let extension_names = entry.enumerate_instance_extension_properties(Some(chars(b"\0")))?;

    let required_extension = vec![
        khr::Surface::name().as_ptr(),
        ash::extensions::khr::Win32Surface::name().as_ptr(),
        ash::extensions::khr::GetSurfaceCapabilities2::name().as_ptr(),
        ash::extensions::khr::GetPhysicalDeviceProperties2::name().as_ptr(),
        ext::DebugUtils::name().as_ptr(),
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


    let app_info = vk::ApplicationInfo::default()
        .application_name(app_name)
        .application_version(0)
        .engine_name(app_name)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));

    let instance_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_layer_names(&required_layers)
        .enabled_extension_names(&required_extension)
        .flags(vk::InstanceCreateFlags::default());

    Ok(Arc::new(
        unsafe { entry.create_instance(&instance_info, None)? }
    ))
}

fn debug_init(entry: &Entry, instance: &Instance) -> (DebugUtilsMessengerEXT, DebugUtils) {
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

    let debug_utils = ext::DebugUtils::new(&entry, &instance);
    unsafe {(
        debug_utils
            .create_debug_utils_messenger(&debug_info, None)
            .unwrap(),
        debug_utils,
    )}
}

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

struct VKPhysicalDeviceProperties {
    queues: Vec<vk::QueueFamilyProperties>,
    features: vk::PhysicalDeviceFeatures,
    properties: vk::PhysicalDeviceProperties,
    physical_device: vk::PhysicalDevice,
    layers_properties: Vec<vk::LayerProperties>,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    extensions_properties: Vec<vk::ExtensionProperties>,
    surface_present: Vec<vk::PresentModeKHR>,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    graphic_index: u32
}

fn enumerate_device(instance: &Instance, surface: &vk::SurfaceKHR, surface_handle: &khr::Surface) -> VKPhysicalDeviceProperties {
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
                    memory_properties: instance.get_physical_device_memory_properties(physical_device),
                    layers_properties: instance.enumerate_device_layer_properties(physical_device).unwrap(),
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

fn create_device(
    instance: &Instance,
    device_properties: &VKPhysicalDeviceProperties,
    extensions: &Vec<*const c_char>,
    features: &vk::PhysicalDeviceFeatures
) -> (Device, vk::Queue) {
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
        let device = instance
            .create_device(device_properties.physical_device, &device_create_info, None)
            .expect("create device");
        let queue = device.get_device_queue(device_properties.graphic_index, 0);
        (device, queue)
    }
}

fn create_swapchain(
    swapchain_proc : &khr::Swapchain,
    surface: &vk::SurfaceKHR,
    device_properties: &VKPhysicalDeviceProperties
) -> vk::SwapchainKHR {

    let present = device_properties.surface_present
        .iter()
        .max_by_key(|x| match **x {
            vk::PresentModeKHR::MAILBOX => 2,
            vk::PresentModeKHR::FIFO => 1,
            _ => 0
        })
        .unwrap()
        .clone();

    let extent = device_properties.surface_capabilities.current_extent;
    let format = device_properties.surface_formats[0];
    let image_count = device_properties.surface_capabilities.min_image_count+1;
    let transform = device_properties.surface_capabilities.current_transform;

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
        .surface(*surface)
        .min_image_count(image_count)
        .image_color_space(format.color_space)
        .image_format(format.format)
        .image_extent(extent)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present)
        .clipped(true)
        .image_array_layers(1);
    println!("{:?}", extent);

    unsafe {
        swapchain_proc.create_swapchain(&swapchain_create_info, None).expect("create swapchain")
    }
}

fn create_image_views(
    device: &Device,
    swapchain: &vk::SwapchainKHR,
    swapchain_proc : &khr::Swapchain,
    device_properties: &VKPhysicalDeviceProperties
) -> (Vec<vk::Image>, Vec<vk::ImageView>) {
    let swapchain_image = unsafe { swapchain_proc.get_swapchain_images(*swapchain) }.expect("create swapchain image");
    let image_views = swapchain_image
        .iter()
        .map(|&image| {
            let create_view_info = vk::ImageViewCreateInfo::default()
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(device_properties.surface_formats[0].format)
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
            unsafe { device.create_image_view(&create_view_info, None).expect("create image view") }
        })
        .collect::<Vec<vk::ImageView>>();
    (swapchain_image, image_views)
}

fn spirv_from_bytes(bytes: &[u8]) -> Vec<u32> {

    assert_eq!(bytes.len() % 4, 0, "shader len");
    let bytes_ptr = bytes.as_ptr() as *mut u32;
    let spirv = unsafe { slice::from_raw_parts(bytes_ptr, bytes.len()/4) }.to_vec();

    // let mut spirv: Vec<_> = bytes
    //     .chunks(4)
    //     .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
    //     .collect();

    match spirv[0] {
        SPV_MAGIC_NUMBER_LE => { spirv },
        SPV_MAGIC_NUMBER_BE => { spirv.into_iter().map(|x| x.swap_bytes()).collect() },
        _ => {panic!("spir-v invalid")}
    }
}

// fn shader_module(device: &Device, path: &str) -> ShaderModule {
fn create_shader_module<'a>(device: &Device, bytes: &[u8], stage: vk::ShaderStageFlags) -> vk::PipelineShaderStageCreateInfo<'a> {

    // let mut file = std::fs::File::open(path).unwrap();
    // let spirv_bytes = util::read_spv(&mut file).unwrap();
    let spirv_bytes = spirv_from_bytes(bytes);
    let shader_create_info = vk::ShaderModuleCreateInfo::default().code(&spirv_bytes);
    let shader_module = unsafe {
        device
            .create_shader_module(&shader_create_info, None)
            .expect("create shader create")
    };
    vk::PipelineShaderStageCreateInfo::default()
        .stage(stage)
        .module(shader_module)
        .name(chars(b"main\0"))
}

fn create_pipeline_layout(device: &Device) -> vk::PipelineLayout {
    let layout_create_info = vk::PipelineLayoutCreateInfo::default();

    unsafe {
        device.create_pipeline_layout(&layout_create_info, None).expect("create pipeline layout")
    }
}

fn create_render_pass(device: &Device, device_properties: &VKPhysicalDeviceProperties) -> vk::RenderPass {

    let swapchain_format = device_properties.surface_formats[0].format;
    let attachment_description = [
        vk::AttachmentDescription::default()
            .format(swapchain_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
    ];

    let attachment_references = [
        vk::AttachmentReference::default()
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .attachment(0)
    ];

    let subpass_descriptions = [
        vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&attachment_references)
    ];

    let subpass_dependencies = [
        vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
    ];

    let render_pass_create_info = vk::RenderPassCreateInfo::default()
        .attachments(&attachment_description)
        .subpasses(&subpass_descriptions)
        .dependencies(&subpass_dependencies);

    unsafe {
        device.create_render_pass(&render_pass_create_info, None) .expect("create render pass")
    }
}

fn create_graphic_pipeline(
    device: &Device,
    render_pass: &vk::RenderPass,
    pipeline_layout: &vk::PipelineLayout,
    shader_create_infos: &[vk::PipelineShaderStageCreateInfo],
) -> Vec<vk::Pipeline> {

    let vertex_input_create_info = vk::PipelineVertexInputStateCreateInfo::default();

    let input_assembly_create_info = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    let dynamic_states = [
        vk::DynamicState::VIEWPORT,
        vk::DynamicState::SCISSOR
    ];
    let dynamic_state_create_info = vk::PipelineDynamicStateCreateInfo::default()
        .dynamic_states(&dynamic_states);

    let viewport_create_info = vk::PipelineViewportStateCreateInfo::default()
        .viewport_count(1)
        .scissor_count(1);

    let rasterization_create_info = vk::PipelineRasterizationStateCreateInfo::default()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::CLOCKWISE)
        .depth_clamp_enable(false);

    let multisample_create_info = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);

    let color_blend_attachment = [
        vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(false)
            .color_blend_op(vk::BlendOp::ADD)
            .alpha_blend_op(vk::BlendOp::ADD)
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
        // .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
        // .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        // .src_alpha_blend_factor(vk::BlendFactor::ONE)
        // .dst_alpha_blend_factor(vk::BlendFactor::ONE)
    ];

    let color_blend_create_info = vk::PipelineColorBlendStateCreateInfo::default()
        .logic_op_enable(false)
        .logic_op(vk::LogicOp::COPY)
        .attachments(&color_blend_attachment);

    let pipeline_create_info = [
        vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_create_infos)
            .vertex_input_state(&vertex_input_create_info)
            .input_assembly_state(&input_assembly_create_info)
            .viewport_state(&viewport_create_info)
            .rasterization_state(&rasterization_create_info)
            .multisample_state(&multisample_create_info)
            // .depth_stencil_state(&depth_state_info)
            .color_blend_state(&color_blend_create_info)
            .dynamic_state(&dynamic_state_create_info)
            .layout(*pipeline_layout)
            .render_pass(*render_pass)
            .base_pipeline_index(-1)
    ];

    unsafe {
        (
            device.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_create_info, None)
                .expect("create graphics pipeline")
        )
    }
}

fn create_frame_buffer(
    device: &Device,
    render_pass: &vk::RenderPass,
    image_views: &Vec<vk::ImageView>,
    device_properties: &VKPhysicalDeviceProperties
) -> Vec<vk::Framebuffer> {

    let swapchain_extent = device_properties.surface_capabilities.current_extent;
    image_views.iter().map(|&image_view| {
        let attachments = [image_view];
        let frame_buffer_create_info = vk::FramebufferCreateInfo::default()
            .render_pass(*render_pass)
            .attachments(&attachments)
            .width(swapchain_extent.width)
            .height(swapchain_extent.height)
            .layers(1);

        unsafe { device.create_framebuffer(&frame_buffer_create_info, None).expect("create frame buffer") }
    }).collect()
}

fn create_command_pool(device: &Device, queue_family_index: u32) -> vk::CommandPool {

    let pool_create_info = vk::CommandPoolCreateInfo::default()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(queue_family_index);

    unsafe { device.create_command_pool(&pool_create_info, None).expect("create command pool") }
}

fn command_buffer_allocate(device: &Device, pool: &vk::CommandPool) -> Vec<vk::CommandBuffer> {

    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(2)
        .command_pool(*pool);

    unsafe {
        device.allocate_command_buffers(&command_buffer_allocate_info).expect("create command buffer")
    }
}

fn record_command_buffer(
    device: &Device,
    image_index: &u32,
    render_pass: &vk::RenderPass,
    framebuffers: &Vec<vk::Framebuffer>,
    command_buffer: &vk::CommandBuffer,
    graphic_pipeline: &vk::Pipeline,
    device_properties: &VKPhysicalDeviceProperties
) {
    let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::default());
    unsafe {
        device.begin_command_buffer(*command_buffer, &command_buffer_begin_info).expect("begin command buffer");
    }

    let clear_values = [
        vk::ClearValue {color: vk::ClearColorValue {float32: [0.,0.,0.,0.]}}
    ];
    let render_pass_begin_info = vk::RenderPassBeginInfo::default()
        .render_pass(*render_pass)
        .framebuffer(framebuffers[*image_index as usize])
        .render_area(device_properties.surface_capabilities.current_extent.into())
        .clear_values(&clear_values);

    let swapchain_extent = device_properties.surface_capabilities.current_extent;
    let scissors = [
        vk::Rect2D::from(swapchain_extent)
    ];
    let viewports = [
        vk::Viewport::default()
            .x(0.)
            .y(0.)
            .height(swapchain_extent.height as f32)
            .width(swapchain_extent.width as f32)
            .min_depth(0.)
            .max_depth(1.)
    ];

    unsafe {
        device.cmd_begin_render_pass(
            *command_buffer,
            &render_pass_begin_info,
            vk::SubpassContents::INLINE,
        );
        device.cmd_bind_pipeline(
            *command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            *graphic_pipeline
        );
        device.cmd_set_viewport(*command_buffer, 0, &viewports);
        device.cmd_set_scissor(*command_buffer, 0, &scissors);
        device.cmd_draw(
            *command_buffer,
            3,
            1,
            0,
            0
        );
        device.cmd_end_render_pass(*command_buffer);
        device.end_command_buffer(*command_buffer).expect("end command buffer");
    }
}

fn iter_destory<T: Copy, F: Fn(T)>(objects: Vec<T>, f: F) {
    objects.iter().for_each(|&x| f(x))
}

fn main() {
    let entry = Entry::linked();

    let instance = create_instance(&entry).unwrap();
    let (debug_messenger, debug_utils) = debug_init(&entry, &instance);

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("window!")
        .with_inner_size(winit::dpi::LogicalSize::new(1000, 1000))
        .build(&event_loop)
        .unwrap();

    let surface_proc = khr::Surface::new(&entry, &instance);
    let surface = create_vk_surface(&entry, &window, &instance).unwrap();

    let device_properties = enumerate_device(&instance, &surface, &surface_proc);

    let features = vk::PhysicalDeviceFeatures::default();
    let extensions = vec![khr::Swapchain::name().as_ptr()];
    let (device, queue) = create_device(&instance, &device_properties, &extensions, &features);

    let swapchain_proc = khr::Swapchain::new(&instance, &device);
    let swapchain = create_swapchain(&swapchain_proc, &surface, &device_properties);

    let (swapchain_image, image_views) = create_image_views(&device, &swapchain, &swapchain_proc, &device_properties);

    let render_pass = create_render_pass(&device, &device_properties);


    let frag_create_info = create_shader_module(&device, include_bytes!("shader/frag.spv"), vk::ShaderStageFlags::FRAGMENT);
    let vert_create_info = create_shader_module(&device, include_bytes!("shader/vert.spv"), vk::ShaderStageFlags::VERTEX);
    let shader_create_infos = [
        frag_create_info,
        vert_create_info
    ];
    let pipeline_layout = create_pipeline_layout(&device);
    let graphic_pipelines = create_graphic_pipeline(&device, &render_pass, &pipeline_layout, &shader_create_infos);

    let framebuffers = create_frame_buffer(&device, &render_pass, &image_views, &device_properties);

    let command_pool = create_command_pool(&device, device_properties.graphic_index);
    let command_buffers = command_buffer_allocate(&device, &command_pool);


    let semaphore_create_info = vk::SemaphoreCreateInfo::default();
    let fence_create_info = vk::FenceCreateInfo::default()
        .flags(vk::FenceCreateFlags::SIGNALED);

    // let setup_commands_reuse_fence = device.create_fence(&fence_create_info, None).expect("create fence");

    let (
        draw_end_fences, image_available_semaphores, draw_end_semaphores
    ) = unsafe {(
        // device.create_fence(&fence_create_info, None).expect("create fence"),
        [
            device.create_fence(&fence_create_info, None).expect("create fence"),
            device.create_fence(&fence_create_info, None).expect("create fence")
        ],
        [
            device.create_semaphore(&semaphore_create_info, None).expect("create semaphore"),
            device.create_semaphore(&semaphore_create_info, None).expect("create semaphore")
        ],
        [
            device.create_semaphore(&semaphore_create_info, None).expect("create semaphore"),
            device.create_semaphore(&semaphore_create_info, None).expect("create semaphore")
        ]
    ) };

    let mut index: usize = 0;

    let draw = |frame_index| {
        unsafe {
            device.wait_for_fences(&[draw_end_fences[frame_index]], true, 100000000).expect("Wait for fence failed.");
            device.reset_fences(&[draw_end_fences[frame_index]]).expect("Reset fences failed.");
            let (image_index, suboptimal) = swapchain_proc
                .acquire_next_image(swapchain, 100000000, image_available_semaphores[frame_index], vk::Fence::null())
                .expect("acquire image");
            print!("{suboptimal} - {frame_index}\n");

            let command_buffer = command_buffers[frame_index];
            let graphic_pipeline = graphic_pipelines[0];
            device.reset_command_buffer(
                command_buffer,
                vk::CommandBufferResetFlags::default(),
            ).expect("Reset command buffer");
            record_command_buffer(
                &device,
                &image_index,
                &render_pass,
                &framebuffers,
                &command_buffer,
                &graphic_pipeline,
                &device_properties
            );

            let wait_semaphores = [image_available_semaphores[frame_index]];
            let signal_semaphores = [draw_end_semaphores[frame_index]];
            let command_bufferss = [command_buffer];
            let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let submit_info = vk::SubmitInfo::default()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&wait_stages)
                .command_buffers(&command_bufferss)
                .signal_semaphores(&signal_semaphores);
            device.queue_submit(queue, &[submit_info], draw_end_fences[frame_index]).expect("queue submit");

            let swapchains = [swapchain];
            let image_indexes = [image_index];
            let present_info = vk::PresentInfoKHR::default()
                .wait_semaphores(&signal_semaphores) // &base.rendering_complete_semaphore)
                .swapchains(&swapchains)
                .image_indices(&image_indexes);

            swapchain_proc
                .queue_present(queue, &present_info)
                .unwrap();

            device.device_wait_idle().expect("wait idle");
        }

    };

    event_loop.run_return(|event, _, control_flow| {
        // println!("{event:?}");

        control_flow.set_wait();


        match event {
            Event::WindowEvent { event, window_id } => {
                match event {
                    WE::CloseRequested => {
                        control_flow.set_exit()
                    }
                    // WE::KeyboardInput { input, is_synthetic, .. } => {
                    //     println!("{input:?}, {is_synthetic:?}")
                    // }
                    // WE::MouseInput { state, button, .. } => {
                    //     println!("{button:?}, {state:?}")
                    // }
                    // WE::CursorMoved { position, .. } => {
                    //     println!("{position:?}")
                    // }
                    _ => {}
                }
            },
            // Event::DeviceEvent {event, device_id} => {
            //     match event {
            //         // DE::MouseMotion { delta } => {
            //         //     println!("{delta:?}");
            //         // }
            //         // DE::Button { button, state } => {
            //         //     println!("{button:?}, {state:?}")
            //         // }
            //         _ => {}
            //     }
            // }
            Event::MainEventsCleared => {
                draw(index % 2);
                index += 1;
                // println!("233");
            }
            _ => (),
        }
    });

    unsafe {
        device.device_wait_idle().expect("wait idle");

        image_views.iter().for_each(|&image| device.destroy_image_view(image, None));
        framebuffers.iter().for_each( |&frame| device.destroy_framebuffer(frame, None));
        draw_end_fences.iter().for_each( |&fence| device.destroy_fence(fence, None));
        graphic_pipelines.iter().for_each(|&pipeline| device.destroy_pipeline(pipeline, None));
        shader_create_infos.iter().for_each(|&shader| device.destroy_shader_module(shader.module, None));
        draw_end_semaphores.iter().for_each( |&semaphore| device.destroy_semaphore(semaphore, None));
        image_available_semaphores.iter().for_each( |&semaphore| device.destroy_semaphore(semaphore, None));

        swapchain_proc.destroy_swapchain(swapchain, None);
        surface_proc.destroy_surface(surface, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_render_pass(render_pass, None);
        device.destroy_command_pool(command_pool, None);
        device.destroy_device(None);
        debug_utils.destroy_debug_utils_messenger(debug_messenger, None);
        instance.destroy_instance(None);

    }
}
