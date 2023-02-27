
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter, Pointer};
use std::iter::{Map, once};
use std::{mem, ptr, slice};

use ash::{*, prelude::VkResult};
use ash::extensions::{khr, ext};
use ash::extensions::khr::Swapchain;
use ash::vk::{PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR, SwapchainKHR};
use winit::{
    event::{Event, DeviceEvent as DE, WindowEvent as WE},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::window;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use smallvec::{smallvec, SmallVec};
use winit::event::VirtualKeyCode::N;
use winit::platform::run_return::EventLoopExtRunReturn;
use crate::device::VKDevice;
use crate::vk_proc::proc::VKProc;


