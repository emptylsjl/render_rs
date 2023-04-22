use ash::vk;
use ash::vk::*;

pub const FORMAT_R8G8B8A8_SRGB: Format = vk::Format::R8G8B8A8_SRGB;

pub const VK_SHARING_MODE_EXCLUSIVE: SharingMode = vk::SharingMode::EXCLUSIVE;

pub const BUFFER_USAGE_TRANSFER_DST_BIT: BufferUsageFlags = vk::BufferUsageFlags::TRANSFER_DST;
pub const BUFFER_USAGE_TRANSFER_SRC_BIT: BufferUsageFlags = vk::BufferUsageFlags::TRANSFER_SRC;
pub const BUFFER_USAGE_TRANSFER_VERTEX_BUFFER_BIT: BufferUsageFlags = vk::BufferUsageFlags::VERTEX_BUFFER;
pub const BUFFER_USAGE_TRANSFER_INDEX_BUFFER_BIT: BufferUsageFlags = vk::BufferUsageFlags::INDEX_BUFFER;

//https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html
pub const VK_SAMPLE_COUNT_1_BIT: SampleCountFlags = vk::SampleCountFlags::TYPE_1;
pub const VK_SAMPLE_COUNT_2_BIT: SampleCountFlags = vk::SampleCountFlags::TYPE_2;
pub const VK_SAMPLE_COUNT_4_BIT: SampleCountFlags = vk::SampleCountFlags::TYPE_4;
pub const VK_SAMPLE_COUNT_8_BIT: SampleCountFlags = vk::SampleCountFlags::TYPE_8;
pub const VK_SAMPLE_COUNT_16_BIT: SampleCountFlags = vk::SampleCountFlags::TYPE_16;
pub const VK_SAMPLE_COUNT_32_BIT: SampleCountFlags = vk::SampleCountFlags::TYPE_32;
pub const VK_SAMPLE_COUNT_64_BIT: SampleCountFlags = vk::SampleCountFlags::TYPE_64;

pub const IMAGE_TILING_OPTIMAL: ImageTiling = vk::ImageTiling::OPTIMAL;

pub const IMAGE_LAYOUT_UNDEFINED: ImageLayout = vk::ImageLayout::UNDEFINED;

pub const IMAGE_TYPE_2D: ImageType = vk::ImageType::TYPE_2D;

pub const IMAGE_USAGE_TRANSFER_DST_BIT: ImageUsageFlags = vk::ImageUsageFlags::TRANSFER_DST;
pub const IMAGE_USAGE_TRANSFER_SRC_BIT: ImageUsageFlags = vk::ImageUsageFlags::TRANSFER_SRC;
pub const VK_IMAGE_USAGE_SAMPLED_BIT: ImageUsageFlags = vk::ImageUsageFlags::SAMPLED;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: ImageUsageFlags = vk::ImageUsageFlags::COLOR_ATTACHMENT;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;
pub const :  = ;