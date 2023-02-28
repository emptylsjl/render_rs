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
use ash::vk::{DebugUtilsMessengerEXT, Format};
use winit::{
    event::{Event, DeviceEvent as DE, WindowEvent as WE},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::window::Window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use smallvec::{smallvec, SmallVec};
use winit::event::VirtualKeyCode;
use winit::platform::run_return::EventLoopExtRunReturn;

mod define;
mod vk_proc;
mod present;
mod device;
mod swapchain;

use define::*;
use present::SurfaceProperty;
use vk_proc::proc::VKProc;
use crate::device::VKDevice;
use crate::present::{create_swapchain, create_vk_surface, VKPresent, VKWindow};


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

fn create_render_pass(device: &VKDevice, format: Format) -> vk::RenderPass {

    let attachment_description = [
        vk::AttachmentDescription::default()
            .format(format)
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
        device.device
            .create_render_pass(&render_pass_create_info, None)
            .expect("create render pass")
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
        device.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_create_info, None)
            .expect("create graphics pipeline")
    }
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
    device: &VKDevice,
    present: &VKPresent,
    image_index: &u32,
    render_pass: &vk::RenderPass,
    command_buffer: &vk::CommandBuffer,
    graphic_pipeline: &vk::Pipeline,
) {
    let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::default());
    unsafe {
        device.device.begin_command_buffer(*command_buffer, &command_buffer_begin_info).expect("begin command buffer");
    }

    let clear_values = [
        vk::ClearValue {color: vk::ClearColorValue {float32: [0.,0.,0.,0.]}}
    ];
    let swapchain_extent = present.window.get_extent();

    let render_pass_begin_info = vk::RenderPassBeginInfo::default()
        .render_pass(*render_pass)
        .framebuffer(present.framebuffers[*image_index as usize])
        .render_area(swapchain_extent.into())
        .clear_values(&clear_values);

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
        device.device.cmd_begin_render_pass(
            *command_buffer,
            &render_pass_begin_info,
            vk::SubpassContents::INLINE,
        );
        device.device.cmd_bind_pipeline(
            *command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            *graphic_pipeline
        );
        device.device.cmd_set_viewport(*command_buffer, 0, &viewports);
        device.device.cmd_set_scissor(*command_buffer, 0, &scissors);
        device.device.cmd_draw(
            *command_buffer,
            3,
            1,
            0,
            0
        );
        device.device.cmd_end_render_pass(*command_buffer);
        device.device.end_command_buffer(*command_buffer).expect("end command buffer");
    }
}

fn main() {

    let vkproc = VKProc::new(true).init_khr_validation();

    // let entry = &vkproc.entry;
    // let instance = &vkproc.instance;
    // let surface_proc = &vkproc.surface;
    // let b = ;

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("window!")
        .with_inner_size(winit::dpi::LogicalSize::new(1000, 1000))
        .build(&event_loop)
        .unwrap();

    let surface = create_vk_surface(&vkproc, &window).unwrap();

    let vkdevice = VKDevice::new(&vkproc, &surface);

    let vkwindow = VKWindow::new(&vkproc, vkdevice.physical_device(), window, surface);

    let render_pass = create_render_pass(&vkdevice, vkwindow.get_format().format);

    let mut vkpresent = VKPresent::new(&vkproc, &vkdevice, vkwindow, &render_pass);

    let device = &vkdevice.device;
    let queue = vkdevice.graphic_queue.queue;
    let queue_index = vkdevice.graphic_queue.index;


    let frag_create_info = create_shader_module(&device, include_bytes!("shader/frag.spv"), vk::ShaderStageFlags::FRAGMENT);
    let vert_create_info = create_shader_module(&device, include_bytes!("shader/vert.spv"), vk::ShaderStageFlags::VERTEX);
    let shader_create_infos = [
        frag_create_info,
        vert_create_info
    ];
    let pipeline_layout = create_pipeline_layout(&device);
    let graphic_pipelines = create_graphic_pipeline(&device, &render_pass, &pipeline_layout, &shader_create_infos);

    let command_pool = create_command_pool(&device, queue_index);
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
    let mut recreate = false;
    let mut exit = false;

    event_loop.run_return(|event, _, control_flow| {
        // println!("{event:?}");

        control_flow.set_wait();


        match event {
            Event::MainEventsCleared => {
                vkpresent.window.window.request_redraw();
            },
            Event::RedrawEventsCleared => 'draw: {
                unsafe {
                    if !exit {
                        let frame_index = index % 2;
                        device.wait_for_fences(&[draw_end_fences[frame_index]], true, u64::MAX).expect("Wait for fence failed.");
                        let image_index = match vkpresent.proc.acquire_next_image(
                            vkpresent.swapchain,
                            u64::MAX,
                            image_available_semaphores[frame_index],
                            vk::Fence::null()
                        ) {
                            Ok((index, suboptimal)) => {
                                recreate = suboptimal;
                                index
                            }
                            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                                vkpresent.recreate_swapchain(&render_pass);
                                break 'draw
                            }
                            Err(a) => { panic!("miao: {a:?}") }
                        };

                        device.reset_fences(&[draw_end_fences[frame_index]]).expect("Reset fences failed.");
                        let command_buffer = command_buffers[frame_index];
                        let graphic_pipeline = graphic_pipelines[0];
                        device.reset_command_buffer(
                            command_buffer,
                            vk::CommandBufferResetFlags::default(),
                        ).expect("Reset command buffer");
                        record_command_buffer(
                            &vkdevice,
                            &vkpresent,
                            &image_index,
                            &render_pass,
                            &command_buffer,
                            &graphic_pipeline,
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

                        let swapchains = [vkpresent.swapchain];
                        let image_indexes = [image_index];
                        let present_info = vk::PresentInfoKHR::default()
                            .wait_semaphores(&signal_semaphores) // &base.rendering_complete_semaphore)
                            .swapchains(&swapchains)
                            .image_indices(&image_indexes);

                        match vkpresent.proc.queue_present(queue, &present_info) {
                            Ok(false) => {}
                            Ok(true) => { recreate = true }
                            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => { recreate = true }
                            Err(a) => { print!("miao miao miao\n{a:?}"); }
                        };
                        index += 1;
                    }
                    if recreate {
                        vkpresent.recreate_swapchain(&render_pass);
                        recreate = false;
                    }
                }
            }


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

            Event::WindowEvent { event, window_id } => {
                match event {
                    WE::Resized(a) => unsafe {
                        // println!("{a:?}");
                        // vkpresent.recreate_swapchain(&render_pass)
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

                    WE::CloseRequested => {
                        unsafe {
                            device.device_wait_idle().expect("wait idle");

                            draw_end_fences.iter().for_each( |&fence| device.destroy_fence(fence, None));
                            graphic_pipelines.iter().for_each(|&pipeline| device.destroy_pipeline(pipeline, None));
                            shader_create_infos.iter().for_each(|&shader| device.destroy_shader_module(shader.module, None));
                            draw_end_semaphores.iter().for_each( |&semaphore| device.destroy_semaphore(semaphore, None));
                            image_available_semaphores.iter().for_each( |&semaphore| device.destroy_semaphore(semaphore, None));

                            device.destroy_pipeline_layout(pipeline_layout, None);
                            device.destroy_render_pass(render_pass, None);
                            device.destroy_command_pool(command_pool, None);
                        }
                        control_flow.set_exit();
                        exit = true;
                    }
                    _ => {}
                }
            },
            _ => (),
        }
    });
}
