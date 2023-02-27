

use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::mem;
use std::mem::MaybeUninit;

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use once_cell::unsync::OnceCell;
use crate::vk_proc::proc::VKProc;

impl VKProc {
    pub fn DestroyDebugUtilsMessengerEXT() {

    }
}