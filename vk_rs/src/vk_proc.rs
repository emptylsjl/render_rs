use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::mem::MaybeUninit;
use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use ash::vk::DebugUtilsMessengerEXT;

pub struct VKProc {
    pub swapchain: khr::Swapchain,
    pub surface: khr::Surface,
    pub dbg_util: ext::DebugUtils,
    pub instance: Instance,
    pub entry: Entry
}

pub struct LayerProp {
    pub implementation_ver: u32,
    pub spec_ver: u32,
    pub description: String,
}

pub struct ExtensionProp {
    pub spec_ver: u32,
}

pub struct Externals {
    pub layers: HashMap<String, LayerProp>,
    pub extensions: HashMap<String, ExtensionProp>,
}

fn char2s(chars: [c_char; 256]) -> String {
    String::from_utf8(chars.into_iter().filter(|&c| c != 0).map(|c| c as u8).collect()).unwrap_or_default()
}

fn chars(s: &[u8]) -> &CStr {
    CStr::from_bytes_with_nul(s).unwrap()
}

impl Externals {
    fn new(entry: &Entry) -> Self{
        let layer_props = entry.enumerate_instance_layer_properties()
            .expect("iter instance layers");
        let extension_props = entry.enumerate_instance_extension_properties(Some(chars(b"\0")))
            .expect("iter instance extension");

        Externals {
            layers: layer_props.into_iter().map(|layer| (
                char2s(layer.layer_name),
                LayerProp {
                    implementation_ver: layer.implementation_version,
                    spec_ver: layer.spec_version,
                    description: char2s(layer.description),
                }
            )).collect(),

            extensions: extension_props.into_iter().map(|ext| (
                char2s(ext.extension_name),
                ExtensionProp {
                    spec_ver: ext.spec_version,
                }
            )).collect(),
        }
    }
}

fn create_instance(entry: &Entry) -> Instance {

    let required_extension = vec![
        khr::Surface::name().as_ptr(),
        khr::Win32Surface::name().as_ptr(),
        khr::GetSurfaceCapabilities2::name().as_ptr(),
        khr::GetPhysicalDeviceProperties2::name().as_ptr(),
        ext::DebugUtils::name().as_ptr(),
    ];

    let app_name = chars(b"VkTri\0");
    let required_layers = [
        chars(b"VK_LAYER_KHRONOS_validation\0").as_ptr()
    ];


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

    unsafe { entry.create_instance(&instance_info, None).expect("create instance") }
}

impl VKProc {
    pub fn new(entry: Entry, debug: bool) {
        let instance = create_instance(&entry);
        let surface = khr::Surface::new(&entry, &instance);
        // let a = if debug {
        //     ext::DebugUtils::new(&entry, &instance)
        // } else {
        //     // MaybeUninit::<>::uninit()
        // };
        let mut x = MaybeUninit::<&i32>::uninit();
        x.write(&0);
        let x = unsafe { x.assume_init() };
        let debug_utils = ext::DebugUtils::new(&entry, &instance);
    }

}