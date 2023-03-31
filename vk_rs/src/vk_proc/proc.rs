
use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::mem;
use std::mem::MaybeUninit;

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use once_cell::unsync::OnceCell;

pub struct VKProc {
    // instance_hd: vk::Instance,
    // fp_instance: vk::,
    fp_debug_utils: vk::ExtDebugUtilsFn,
    fp_surface_khr: vk::KhrSurfaceFn,
    debug_msg: vk::DebugUtilsMessengerEXT,
    // pub swapchain: khr::Swapchain,
    pub surface: khr::Surface,
    pub debug_util: ext::DebugUtils,
    pub instance: Instance,
    pub entry: Entry,
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

pub fn char2s(chars: &[c_char]) -> String {
    String::from_utf8(chars.into_iter().map_while(|&c|
        if c == 0 { None }
        else { Some(c as u8) }
    ).collect()).unwrap_or_default()
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
                char2s(&layer.layer_name),
                LayerProp {
                    implementation_ver: layer.implementation_version,
                    spec_ver: layer.spec_version,
                    description: char2s(&layer.description),
                }
            )).collect(),

            extensions: extension_props.into_iter().map(|ext| (
                char2s(&ext.extension_name),
                ExtensionProp {
                    spec_ver: ext.spec_version,
                }
            )).collect(),
        }
    }
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

fn instance_create(entry: &Entry) -> Instance {

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
    pub fn new(debug: bool) -> Self {
        let entry = Entry::linked();
        let instance = instance_create(&entry);
        let instance_hd = instance.handle();
        let debug_util = ext::DebugUtils::new(&entry, &instance);
        let surface = khr::Surface::new(&entry, &instance);

        let fp_surface_khr = vk::KhrSurfaceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance_hd, name.as_ptr()))
        });
        let fp_debug_utils = vk::ExtDebugUtilsFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance_hd, name.as_ptr()))
        });
        unsafe {
            // Self {
            //     swapchain: OnceCell::new(),
            //     dbg_util: OnceCell::new(),
            //     surface: OnceCell::new(),
            //     instance,
            //     entry,
            // }
            Self {
                debug_msg: mem::zeroed(),
                fp_debug_utils,
                fp_surface_khr,
                debug_util,
                surface,
                instance,
                entry,
            }
        }
    }

    pub fn validation_callback(mut self) -> Self {
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
                    vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
                    vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
            )
            .pfn_user_callback(Some(debug_callback));

        unsafe {
            self.debug_msg = self.debug_util.create_debug_utils_messenger(&debug_info, None).expect("add debug callback");
            self
        }
    }
}

impl Drop for VKProc {
    fn drop(&mut self) {
        unsafe {

            println!("4");
            self.debug_util.destroy_debug_utils_messenger(self.debug_msg, None);
            println!("5");
            self.instance.destroy_instance(None);
        }
    }
}