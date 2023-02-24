//
// #![feature(test)]
// extern crate test;
// use test::Bencher;
//
// use std::ffi::CStr;
//
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use ash::{*, prelude::VkResult};
// use ash::extensions::{khr, ext};
// use ash::extensions::ext::DebugUtils;
// use ash::vk::DebugUtilsMessengerEXT;
// use winit::{
//     event::{Event, DeviceEvent as DE, WindowEvent as WE},
//     event_loop::EventLoop,
//     window::WindowBuilder,
// };
// use winit::window::Window;
// use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
// use smallvec::{smallvec, SmallVec};
// use winit::event::VirtualKeyCode::N;
// use winit::platform::run_return::EventLoopExtRunReturn;
// use rand;
// use rand::Rng;
//
// fn chars(s: &[u8]) -> &CStr {
//     CStr::from_bytes_with_nul(s).unwrap()
// }
//
// fn create_instance(entry: &Entry) -> Instance {
//
//     let required_extension = vec![
//         khr::Surface::name().as_ptr(),
//         khr::Win32Surface::name().as_ptr(),
//         khr::GetSurfaceCapabilities2::name().as_ptr(),
//         khr::GetPhysicalDeviceProperties2::name().as_ptr(),
//     ];
//
//     let app_name = chars(b"VkTri\0");
//
//     let app_info = vk::ApplicationInfo::default()
//         .application_name(app_name)
//         .application_version(0)
//         .engine_name(app_name)
//         .engine_version(0)
//         .api_version(vk::make_api_version(0, 1, 0, 0));
//
//     let instance_info = vk::InstanceCreateInfo::default()
//         .application_info(&app_info)
//         .enabled_extension_names(&required_extension)
//         .flags(vk::InstanceCreateFlags::default());
//
//     unsafe { entry.create_instance(&instance_info, None).expect("create instance") }
// }
//
// fn create_vk_surface<Handle: HasRawWindowHandle + HasRawDisplayHandle>(
//     entry: &Entry,
//     window: &Handle,
//     instance: &Instance,
// ) -> VkResult<vk::SurfaceKHR>  {
//     match window.raw_window_handle() {
//         RawWindowHandle::Win32(handle) => unsafe {
//             let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
//                 .hinstance(handle.hinstance)
//                 .hwnd(handle.hwnd);
//             let surface_fn = khr::Win32Surface::new(entry, instance);
//             surface_fn.create_win32_surface(&surface_desc, None)
//         }
//
//         _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
//     }
// }
//
// pub fn criterion_benchmark(c: &mut Criterion) {
//
//     // let entry = Entry::linked();
//     //
//     // let mut event_loop = EventLoop::new();
//     // let window = WindowBuilder::new()
//     //     .with_title("window!")
//     //     .with_inner_size(winit::dpi::LogicalSize::new(1000, 1000))
//     //     .build(&event_loop)
//     //     .unwrap();
//     //
//     // unsafe {
//     //     let instance = create_instance(&entry);
//     //     let device = instance.enumerate_physical_devices().unwrap()[0];
//     //     let surface_proc = khr::Surface::new(&entry, &instance);
//     //     let surface = create_vk_surface(&entry, &window, &instance).unwrap();
//     //
//     //
//     //     c.bench_function("a", |b| b.iter(|| window.inner_size()));
//     //     c.bench_function("b", |b| b.iter(|| surface_proc.get_physical_device_surface_capabilities(device, surface).unwrap()));
//     //
//     //
//     //     // swapchain_proc.destroy_swapchain(swapchain, None);
//     //     surface_proc.destroy_surface(surface, None);
//     //     // device.destroy_pipeline_layout(pipeline_layout, None);
//     //     // device.destroy_render_pass(render_pass, None);
//     //     // device.destroy_command_pool(command_pool, None);
//     //     device.destroy_device(None);
//     //     // debug_utils.destroy_debug_utils_messenger(debug_messenger, None);
//     //     instance.destroy_instance(None);
//     // }
//
//     let mut rng = rand::thread_rng();
//     let cha: Vec<i8> = (0..10000).map(|_| rng.gen()).collect();
//     let chb: Vec<i8> = cha.clone();
//
//     println!("233333---------------------------");
//
//     // #[bench]
//     // fn bench_xor_1000_ints(b: &mut Bencher) {
//     //     b.iter(|| {
//     //         test::black_box(cha.iter().filter(|&&c| c!=0).map(|&c| c as u8).collect::<Vec<_>>());
//     //     });
//     // }
//
//     c.bench_function("a", |b| {
//         let a = black_box(cha.iter().filter(|&&c| c!=0).map(|&c| c as u8).collect::<Vec<_>>());
//     });
//     // println!("---------------------233333---------------------------");
//     // c.bench_function("b", |b| {
//     //     let e: Vec<u8> = chb.iter().filter_map(|&c| {
//     //         if c == 0 {
//     //             None
//     //         } else {
//     //             Some(c as u8)
//     //         }
//     //     }).collect();
//     // });
//     // println!("---------------------233333---------------------------");
// }
//
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
