use ash::vk;

// ImageLayout-VkImageLayout:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html
// Provided by VK_VERSION_1_0
// enum VkImageLayout {
pub const VK_IMAGE_LAYOUT_UNDEFINED: vk::ImageLayout                                                 = vk::ImageLayout::UNDEFINED;
pub const VK_IMAGE_LAYOUT_GENERAL: vk::ImageLayout                                                   = vk::ImageLayout::GENERAL;
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: vk::ImageLayout                                  = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: vk::ImageLayout                          = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: vk::ImageLayout                           = vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL;
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: vk::ImageLayout                                  = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: vk::ImageLayout                                      = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: vk::ImageLayout                                      = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: vk::ImageLayout                                            = vk::ImageLayout::PREINITIALIZED;
// Provided by VK_VERSION_1_1
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: vk::ImageLayout                = vk::ImageLayout::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
// Provided by VK_VERSION_1_1
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: vk::ImageLayout                = vk::ImageLayout::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
// Provided by VK_VERSION_1_2
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL: vk::ImageLayout                                  = vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL;
// Provided by VK_VERSION_1_2
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL: vk::ImageLayout                                   = vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL;
// Provided by VK_VERSION_1_2
pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL: vk::ImageLayout                                = vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL;
// Provided by VK_VERSION_1_2
pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL: vk::ImageLayout                                 = vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL;
// Provided by VK_VERSION_1_3
pub const VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL: vk::ImageLayout                                         = vk::ImageLayout::READ_ONLY_OPTIMAL;
// Provided by VK_VERSION_1_3
pub const VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL: vk::ImageLayout                                        = vk::ImageLayout::ATTACHMENT_OPTIMAL;
// Provided by VK_KHR_swapchain
pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: vk::ImageLayout                                           = vk::ImageLayout::PRESENT_SRC_KHR;
// Provided by VK_KHR_video_decode_queue
pub const VK_IMAGE_LAYOUT_VIDEO_DECODE_DST_KHR: vk::ImageLayout                                      = vk::ImageLayout::VIDEO_DECODE_DST_KHR;
// Provided by VK_KHR_video_decode_queue
pub const VK_IMAGE_LAYOUT_VIDEO_DECODE_SRC_KHR: vk::ImageLayout                                      = vk::ImageLayout::VIDEO_DECODE_SRC_KHR;
// Provided by VK_KHR_video_decode_queue
pub const VK_IMAGE_LAYOUT_VIDEO_DECODE_DPB_KHR: vk::ImageLayout                                      = vk::ImageLayout::VIDEO_DECODE_DPB_KHR;
// Provided by VK_KHR_shared_presentable_image
pub const VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR: vk::ImageLayout                                        = vk::ImageLayout::SHARED_PRESENT_KHR;
// Provided by VK_EXT_fragment_density_map
pub const VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: vk::ImageLayout                          = vk::ImageLayout::FRAGMENT_DENSITY_MAP_OPTIMAL_EXT;
// Provided by VK_KHR_fragment_shading_rate
pub const VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: vk::ImageLayout              = vk::ImageLayout::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_IMAGE_LAYOUT_VIDEO_ENCODE_DST_KHR: vk::ImageLayout                                      = vk::ImageLayout::VIDEO_ENCODE_DST_KHR;
// #endif
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_IMAGE_LAYOUT_VIDEO_ENCODE_SRC_KHR: vk::ImageLayout                                      = vk::ImageLayout::VIDEO_ENCODE_SRC_KHR;
// #endif
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_IMAGE_LAYOUT_VIDEO_ENCODE_DPB_KHR: vk::ImageLayout                                      = vk::ImageLayout::VIDEO_ENCODE_DPB_KHR;
// #endif
// Provided by VK_EXT_attachment_feedback_loop_layout
pub const VK_IMAGE_LAYOUT_ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: vk::ImageLayout                      = vk::ImageLayout::ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT;
// Provided by VK_KHR_maintenance2
//? IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL
// Provided by VK_KHR_maintenance2
//? IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL
// Provided by VK_NV_shading_rate_image
//? IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV = VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR
// Provided by VK_KHR_separate_depth_stencil_layouts
//? IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL
// Provided by VK_KHR_separate_depth_stencil_layouts
//? IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL
// Provided by VK_KHR_separate_depth_stencil_layouts
//? IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL
// Provided by VK_KHR_separate_depth_stencil_layouts
//? IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL
// Provided by VK_KHR_synchronization2
//? IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL
// Provided by VK_KHR_synchronization2
//? IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL
// } VkImageLayout;

// ImageUsageFlags-VkImageUsageFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkImageUsageFlagBits {
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: vk::ImageUsageFlags                                       = vk::ImageUsageFlags::TRANSFER_SRC;
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: vk::ImageUsageFlags                                       = vk::ImageUsageFlags::TRANSFER_DST;
pub const VK_IMAGE_USAGE_SAMPLED_BIT: vk::ImageUsageFlags                                            = vk::ImageUsageFlags::SAMPLED;
pub const VK_IMAGE_USAGE_STORAGE_BIT: vk::ImageUsageFlags                                            = vk::ImageUsageFlags::STORAGE;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: vk::ImageUsageFlags                                   = vk::ImageUsageFlags::COLOR_ATTACHMENT;
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: vk::ImageUsageFlags                           = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: vk::ImageUsageFlags                               = vk::ImageUsageFlags::TRANSIENT_ATTACHMENT;
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: vk::ImageUsageFlags                                   = vk::ImageUsageFlags::INPUT_ATTACHMENT;
// Provided by VK_KHR_video_decode_queue
pub const VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR: vk::ImageUsageFlags                               = vk::ImageUsageFlags::VIDEO_DECODE_DST_KHR;
// Provided by VK_KHR_video_decode_queue
pub const VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR: vk::ImageUsageFlags                               = vk::ImageUsageFlags::VIDEO_DECODE_SRC_KHR;
// Provided by VK_KHR_video_decode_queue
pub const VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR: vk::ImageUsageFlags                               = vk::ImageUsageFlags::VIDEO_DECODE_DPB_KHR;
// Provided by VK_EXT_fragment_density_map
pub const VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT: vk::ImageUsageFlags                           = vk::ImageUsageFlags::FRAGMENT_DENSITY_MAP_EXT;
// Provided by VK_KHR_fragment_shading_rate
pub const VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: vk::ImageUsageFlags               = vk::ImageUsageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR: vk::ImageUsageFlags                               = vk::ImageUsageFlags::VIDEO_ENCODE_DST_KHR;
// #endif
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR: vk::ImageUsageFlags                               = vk::ImageUsageFlags::VIDEO_ENCODE_SRC_KHR;
// #endif
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR: vk::ImageUsageFlags                               = vk::ImageUsageFlags::VIDEO_ENCODE_DPB_KHR;
// #endif
// Provided by VK_EXT_attachment_feedback_loop_layout
pub const VK_IMAGE_USAGE_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: vk::ImageUsageFlags                       = vk::ImageUsageFlags::ATTACHMENT_FEEDBACK_LOOP_EXT;
// Provided by VK_HUAWEI_invocation_mask
pub const VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI: vk::ImageUsageFlags                             = vk::ImageUsageFlags::INVOCATION_MASK_HUAWEI;
// Provided by VK_QCOM_image_processing
pub const VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM: vk::ImageUsageFlags                                 = vk::ImageUsageFlags::SAMPLE_WEIGHT_QCOM;
// Provided by VK_QCOM_image_processing
pub const VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM: vk::ImageUsageFlags                            = vk::ImageUsageFlags::SAMPLE_BLOCK_MATCH_QCOM;
// Provided by VK_NV_shading_rate_image
pub const VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV: vk::ImageUsageFlags                              = VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
// } VkImageUsageFlagBits;

// Format-VkFormat:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFormat.html
// Provided by VK_VERSION_1_0
// enum VkFormat {
pub const VK_FORMAT_UNDEFINED: vk::Format                                                            = vk::Format::UNDEFINED;
pub const VK_FORMAT_R4G4_UNORM_PACK8: vk::Format                                                     = vk::Format::R4G4_UNORM_PACK8;
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: vk::Format                                                = vk::Format::R4G4B4A4_UNORM_PACK16;
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: vk::Format                                                = vk::Format::B4G4R4A4_UNORM_PACK16;
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: vk::Format                                                  = vk::Format::R5G6B5_UNORM_PACK16;
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: vk::Format                                                  = vk::Format::B5G6R5_UNORM_PACK16;
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: vk::Format                                                = vk::Format::R5G5B5A1_UNORM_PACK16;
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: vk::Format                                                = vk::Format::B5G5R5A1_UNORM_PACK16;
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: vk::Format                                                = vk::Format::A1R5G5B5_UNORM_PACK16;
pub const VK_FORMAT_R8_UNORM: vk::Format                                                             = vk::Format::R8_UNORM;
pub const VK_FORMAT_R8_SNORM: vk::Format                                                             = vk::Format::R8_SNORM;
pub const VK_FORMAT_R8_USCALED: vk::Format                                                           = vk::Format::R8_USCALED;
pub const VK_FORMAT_R8_SSCALED: vk::Format                                                           = vk::Format::R8_SSCALED;
pub const VK_FORMAT_R8_UINT: vk::Format                                                              = vk::Format::R8_UINT;
pub const VK_FORMAT_R8_SINT: vk::Format                                                              = vk::Format::R8_SINT;
pub const VK_FORMAT_R8_SRGB: vk::Format                                                              = vk::Format::R8_SRGB;
pub const VK_FORMAT_R8G8_UNORM: vk::Format                                                           = vk::Format::R8G8_UNORM;
pub const VK_FORMAT_R8G8_SNORM: vk::Format                                                           = vk::Format::R8G8_SNORM;
pub const VK_FORMAT_R8G8_USCALED: vk::Format                                                         = vk::Format::R8G8_USCALED;
pub const VK_FORMAT_R8G8_SSCALED: vk::Format                                                         = vk::Format::R8G8_SSCALED;
pub const VK_FORMAT_R8G8_UINT: vk::Format                                                            = vk::Format::R8G8_UINT;
pub const VK_FORMAT_R8G8_SINT: vk::Format                                                            = vk::Format::R8G8_SINT;
pub const VK_FORMAT_R8G8_SRGB: vk::Format                                                            = vk::Format::R8G8_SRGB;
pub const VK_FORMAT_R8G8B8_UNORM: vk::Format                                                         = vk::Format::R8G8B8_UNORM;
pub const VK_FORMAT_R8G8B8_SNORM: vk::Format                                                         = vk::Format::R8G8B8_SNORM;
pub const VK_FORMAT_R8G8B8_USCALED: vk::Format                                                       = vk::Format::R8G8B8_USCALED;
pub const VK_FORMAT_R8G8B8_SSCALED: vk::Format                                                       = vk::Format::R8G8B8_SSCALED;
pub const VK_FORMAT_R8G8B8_UINT: vk::Format                                                          = vk::Format::R8G8B8_UINT;
pub const VK_FORMAT_R8G8B8_SINT: vk::Format                                                          = vk::Format::R8G8B8_SINT;
pub const VK_FORMAT_R8G8B8_SRGB: vk::Format                                                          = vk::Format::R8G8B8_SRGB;
pub const VK_FORMAT_B8G8R8_UNORM: vk::Format                                                         = vk::Format::B8G8R8_UNORM;
pub const VK_FORMAT_B8G8R8_SNORM: vk::Format                                                         = vk::Format::B8G8R8_SNORM;
pub const VK_FORMAT_B8G8R8_USCALED: vk::Format                                                       = vk::Format::B8G8R8_USCALED;
pub const VK_FORMAT_B8G8R8_SSCALED: vk::Format                                                       = vk::Format::B8G8R8_SSCALED;
pub const VK_FORMAT_B8G8R8_UINT: vk::Format                                                          = vk::Format::B8G8R8_UINT;
pub const VK_FORMAT_B8G8R8_SINT: vk::Format                                                          = vk::Format::B8G8R8_SINT;
pub const VK_FORMAT_B8G8R8_SRGB: vk::Format                                                          = vk::Format::B8G8R8_SRGB;
pub const VK_FORMAT_R8G8B8A8_UNORM: vk::Format                                                       = vk::Format::R8G8B8A8_UNORM;
pub const VK_FORMAT_R8G8B8A8_SNORM: vk::Format                                                       = vk::Format::R8G8B8A8_SNORM;
pub const VK_FORMAT_R8G8B8A8_USCALED: vk::Format                                                     = vk::Format::R8G8B8A8_USCALED;
pub const VK_FORMAT_R8G8B8A8_SSCALED: vk::Format                                                     = vk::Format::R8G8B8A8_SSCALED;
pub const VK_FORMAT_R8G8B8A8_UINT: vk::Format                                                        = vk::Format::R8G8B8A8_UINT;
pub const VK_FORMAT_R8G8B8A8_SINT: vk::Format                                                        = vk::Format::R8G8B8A8_SINT;
pub const VK_FORMAT_R8G8B8A8_SRGB: vk::Format                                                        = vk::Format::R8G8B8A8_SRGB;
pub const VK_FORMAT_B8G8R8A8_UNORM: vk::Format                                                       = vk::Format::B8G8R8A8_UNORM;
pub const VK_FORMAT_B8G8R8A8_SNORM: vk::Format                                                       = vk::Format::B8G8R8A8_SNORM;
pub const VK_FORMAT_B8G8R8A8_USCALED: vk::Format                                                     = vk::Format::B8G8R8A8_USCALED;
pub const VK_FORMAT_B8G8R8A8_SSCALED: vk::Format                                                     = vk::Format::B8G8R8A8_SSCALED;
pub const VK_FORMAT_B8G8R8A8_UINT: vk::Format                                                        = vk::Format::B8G8R8A8_UINT;
pub const VK_FORMAT_B8G8R8A8_SINT: vk::Format                                                        = vk::Format::B8G8R8A8_SINT;
pub const VK_FORMAT_B8G8R8A8_SRGB: vk::Format                                                        = vk::Format::B8G8R8A8_SRGB;
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: vk::Format                                                = vk::Format::A8B8G8R8_UNORM_PACK32;
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: vk::Format                                                = vk::Format::A8B8G8R8_SNORM_PACK32;
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: vk::Format                                              = vk::Format::A8B8G8R8_USCALED_PACK32;
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: vk::Format                                              = vk::Format::A8B8G8R8_SSCALED_PACK32;
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: vk::Format                                                 = vk::Format::A8B8G8R8_UINT_PACK32;
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: vk::Format                                                 = vk::Format::A8B8G8R8_SINT_PACK32;
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: vk::Format                                                 = vk::Format::A8B8G8R8_SRGB_PACK32;
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: vk::Format                                             = vk::Format::A2R10G10B10_UNORM_PACK32;
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: vk::Format                                             = vk::Format::A2R10G10B10_SNORM_PACK32;
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: vk::Format                                           = vk::Format::A2R10G10B10_USCALED_PACK32;
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: vk::Format                                           = vk::Format::A2R10G10B10_SSCALED_PACK32;
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: vk::Format                                              = vk::Format::A2R10G10B10_UINT_PACK32;
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: vk::Format                                              = vk::Format::A2R10G10B10_SINT_PACK32;
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: vk::Format                                             = vk::Format::A2B10G10R10_UNORM_PACK32;
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: vk::Format                                             = vk::Format::A2B10G10R10_SNORM_PACK32;
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: vk::Format                                           = vk::Format::A2B10G10R10_USCALED_PACK32;
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: vk::Format                                           = vk::Format::A2B10G10R10_SSCALED_PACK32;
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: vk::Format                                              = vk::Format::A2B10G10R10_UINT_PACK32;
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: vk::Format                                              = vk::Format::A2B10G10R10_SINT_PACK32;
pub const VK_FORMAT_R16_UNORM: vk::Format                                                            = vk::Format::R16_UNORM;
pub const VK_FORMAT_R16_SNORM: vk::Format                                                            = vk::Format::R16_SNORM;
pub const VK_FORMAT_R16_USCALED: vk::Format                                                          = vk::Format::R16_USCALED;
pub const VK_FORMAT_R16_SSCALED: vk::Format                                                          = vk::Format::R16_SSCALED;
pub const VK_FORMAT_R16_UINT: vk::Format                                                             = vk::Format::R16_UINT;
pub const VK_FORMAT_R16_SINT: vk::Format                                                             = vk::Format::R16_SINT;
pub const VK_FORMAT_R16_SFLOAT: vk::Format                                                           = vk::Format::R16_SFLOAT;
pub const VK_FORMAT_R16G16_UNORM: vk::Format                                                         = vk::Format::R16G16_UNORM;
pub const VK_FORMAT_R16G16_SNORM: vk::Format                                                         = vk::Format::R16G16_SNORM;
pub const VK_FORMAT_R16G16_USCALED: vk::Format                                                       = vk::Format::R16G16_USCALED;
pub const VK_FORMAT_R16G16_SSCALED: vk::Format                                                       = vk::Format::R16G16_SSCALED;
pub const VK_FORMAT_R16G16_UINT: vk::Format                                                          = vk::Format::R16G16_UINT;
pub const VK_FORMAT_R16G16_SINT: vk::Format                                                          = vk::Format::R16G16_SINT;
pub const VK_FORMAT_R16G16_SFLOAT: vk::Format                                                        = vk::Format::R16G16_SFLOAT;
pub const VK_FORMAT_R16G16B16_UNORM: vk::Format                                                      = vk::Format::R16G16B16_UNORM;
pub const VK_FORMAT_R16G16B16_SNORM: vk::Format                                                      = vk::Format::R16G16B16_SNORM;
pub const VK_FORMAT_R16G16B16_USCALED: vk::Format                                                    = vk::Format::R16G16B16_USCALED;
pub const VK_FORMAT_R16G16B16_SSCALED: vk::Format                                                    = vk::Format::R16G16B16_SSCALED;
pub const VK_FORMAT_R16G16B16_UINT: vk::Format                                                       = vk::Format::R16G16B16_UINT;
pub const VK_FORMAT_R16G16B16_SINT: vk::Format                                                       = vk::Format::R16G16B16_SINT;
pub const VK_FORMAT_R16G16B16_SFLOAT: vk::Format                                                     = vk::Format::R16G16B16_SFLOAT;
pub const VK_FORMAT_R16G16B16A16_UNORM: vk::Format                                                   = vk::Format::R16G16B16A16_UNORM;
pub const VK_FORMAT_R16G16B16A16_SNORM: vk::Format                                                   = vk::Format::R16G16B16A16_SNORM;
pub const VK_FORMAT_R16G16B16A16_USCALED: vk::Format                                                 = vk::Format::R16G16B16A16_USCALED;
pub const VK_FORMAT_R16G16B16A16_SSCALED: vk::Format                                                 = vk::Format::R16G16B16A16_SSCALED;
pub const VK_FORMAT_R16G16B16A16_UINT: vk::Format                                                    = vk::Format::R16G16B16A16_UINT;
pub const VK_FORMAT_R16G16B16A16_SINT: vk::Format                                                    = vk::Format::R16G16B16A16_SINT;
pub const VK_FORMAT_R16G16B16A16_SFLOAT: vk::Format                                                  = vk::Format::R16G16B16A16_SFLOAT;
pub const VK_FORMAT_R32_UINT: vk::Format                                                             = vk::Format::R32_UINT;
pub const VK_FORMAT_R32_SINT: vk::Format                                                             = vk::Format::R32_SINT;
pub const VK_FORMAT_R32_SFLOAT: vk::Format                                                           = vk::Format::R32_SFLOAT;
pub const VK_FORMAT_R32G32_UINT: vk::Format                                                          = vk::Format::R32G32_UINT;
pub const VK_FORMAT_R32G32_SINT: vk::Format                                                          = vk::Format::R32G32_SINT;
pub const VK_FORMAT_R32G32_SFLOAT: vk::Format                                                        = vk::Format::R32G32_SFLOAT;
pub const VK_FORMAT_R32G32B32_UINT: vk::Format                                                       = vk::Format::R32G32B32_UINT;
pub const VK_FORMAT_R32G32B32_SINT: vk::Format                                                       = vk::Format::R32G32B32_SINT;
pub const VK_FORMAT_R32G32B32_SFLOAT: vk::Format                                                     = vk::Format::R32G32B32_SFLOAT;
pub const VK_FORMAT_R32G32B32A32_UINT: vk::Format                                                    = vk::Format::R32G32B32A32_UINT;
pub const VK_FORMAT_R32G32B32A32_SINT: vk::Format                                                    = vk::Format::R32G32B32A32_SINT;
pub const VK_FORMAT_R32G32B32A32_SFLOAT: vk::Format                                                  = vk::Format::R32G32B32A32_SFLOAT;
pub const VK_FORMAT_R64_UINT: vk::Format                                                             = vk::Format::R64_UINT;
pub const VK_FORMAT_R64_SINT: vk::Format                                                             = vk::Format::R64_SINT;
pub const VK_FORMAT_R64_SFLOAT: vk::Format                                                           = vk::Format::R64_SFLOAT;
pub const VK_FORMAT_R64G64_UINT: vk::Format                                                          = vk::Format::R64G64_UINT;
pub const VK_FORMAT_R64G64_SINT: vk::Format                                                          = vk::Format::R64G64_SINT;
pub const VK_FORMAT_R64G64_SFLOAT: vk::Format                                                        = vk::Format::R64G64_SFLOAT;
pub const VK_FORMAT_R64G64B64_UINT: vk::Format                                                       = vk::Format::R64G64B64_UINT;
pub const VK_FORMAT_R64G64B64_SINT: vk::Format                                                       = vk::Format::R64G64B64_SINT;
pub const VK_FORMAT_R64G64B64_SFLOAT: vk::Format                                                     = vk::Format::R64G64B64_SFLOAT;
pub const VK_FORMAT_R64G64B64A64_UINT: vk::Format                                                    = vk::Format::R64G64B64A64_UINT;
pub const VK_FORMAT_R64G64B64A64_SINT: vk::Format                                                    = vk::Format::R64G64B64A64_SINT;
pub const VK_FORMAT_R64G64B64A64_SFLOAT: vk::Format                                                  = vk::Format::R64G64B64A64_SFLOAT;
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: vk::Format                                              = vk::Format::B10G11R11_UFLOAT_PACK32;
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: vk::Format                                               = vk::Format::E5B9G9R9_UFLOAT_PACK32;
pub const VK_FORMAT_D16_UNORM: vk::Format                                                            = vk::Format::D16_UNORM;
pub const VK_FORMAT_X8_D24_UNORM_PACK32: vk::Format                                                  = vk::Format::X8_D24_UNORM_PACK32;
pub const VK_FORMAT_D32_SFLOAT: vk::Format                                                           = vk::Format::D32_SFLOAT;
pub const VK_FORMAT_S8_UINT: vk::Format                                                              = vk::Format::S8_UINT;
pub const VK_FORMAT_D16_UNORM_S8_UINT: vk::Format                                                    = vk::Format::D16_UNORM_S8_UINT;
pub const VK_FORMAT_D24_UNORM_S8_UINT: vk::Format                                                    = vk::Format::D24_UNORM_S8_UINT;
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: vk::Format                                                   = vk::Format::D32_SFLOAT_S8_UINT;
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: vk::Format                                                  = vk::Format::BC1_RGB_UNORM_BLOCK;
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: vk::Format                                                   = vk::Format::BC1_RGB_SRGB_BLOCK;
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: vk::Format                                                 = vk::Format::BC1_RGBA_UNORM_BLOCK;
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: vk::Format                                                  = vk::Format::BC1_RGBA_SRGB_BLOCK;
pub const VK_FORMAT_BC2_UNORM_BLOCK: vk::Format                                                      = vk::Format::BC2_UNORM_BLOCK;
pub const VK_FORMAT_BC2_SRGB_BLOCK: vk::Format                                                       = vk::Format::BC2_SRGB_BLOCK;
pub const VK_FORMAT_BC3_UNORM_BLOCK: vk::Format                                                      = vk::Format::BC3_UNORM_BLOCK;
pub const VK_FORMAT_BC3_SRGB_BLOCK: vk::Format                                                       = vk::Format::BC3_SRGB_BLOCK;
pub const VK_FORMAT_BC4_UNORM_BLOCK: vk::Format                                                      = vk::Format::BC4_UNORM_BLOCK;
pub const VK_FORMAT_BC4_SNORM_BLOCK: vk::Format                                                      = vk::Format::BC4_SNORM_BLOCK;
pub const VK_FORMAT_BC5_UNORM_BLOCK: vk::Format                                                      = vk::Format::BC5_UNORM_BLOCK;
pub const VK_FORMAT_BC5_SNORM_BLOCK: vk::Format                                                      = vk::Format::BC5_SNORM_BLOCK;
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: vk::Format                                                    = vk::Format::BC6H_UFLOAT_BLOCK;
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: vk::Format                                                    = vk::Format::BC6H_SFLOAT_BLOCK;
pub const VK_FORMAT_BC7_UNORM_BLOCK: vk::Format                                                      = vk::Format::BC7_UNORM_BLOCK;
pub const VK_FORMAT_BC7_SRGB_BLOCK: vk::Format                                                       = vk::Format::BC7_SRGB_BLOCK;
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: vk::Format                                              = vk::Format::ETC2_R8G8B8_UNORM_BLOCK;
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: vk::Format                                               = vk::Format::ETC2_R8G8B8_SRGB_BLOCK;
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: vk::Format                                            = vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK;
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: vk::Format                                             = vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK;
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: vk::Format                                            = vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK;
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: vk::Format                                             = vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK;
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: vk::Format                                                  = vk::Format::EAC_R11_UNORM_BLOCK;
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: vk::Format                                                  = vk::Format::EAC_R11_SNORM_BLOCK;
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: vk::Format                                               = vk::Format::EAC_R11G11_UNORM_BLOCK;
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: vk::Format                                               = vk::Format::EAC_R11G11_SNORM_BLOCK;
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_4X4_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_4X4_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_5X4_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_5X4_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_5X5_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_5X5_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_6X5_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_6X5_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_6X6_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_6X6_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_8X5_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_8X5_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_8X6_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_8X6_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: vk::Format                                                 = vk::Format::ASTC_8X8_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: vk::Format                                                  = vk::Format::ASTC_8X8_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: vk::Format                                                = vk::Format::ASTC_10X5_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: vk::Format                                                 = vk::Format::ASTC_10X5_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: vk::Format                                                = vk::Format::ASTC_10X6_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: vk::Format                                                 = vk::Format::ASTC_10X6_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: vk::Format                                                = vk::Format::ASTC_10X8_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: vk::Format                                                 = vk::Format::ASTC_10X8_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: vk::Format                                               = vk::Format::ASTC_10X10_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: vk::Format                                                = vk::Format::ASTC_10X10_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: vk::Format                                               = vk::Format::ASTC_12X10_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: vk::Format                                                = vk::Format::ASTC_12X10_SRGB_BLOCK;
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: vk::Format                                               = vk::Format::ASTC_12X12_UNORM_BLOCK;
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: vk::Format                                                = vk::Format::ASTC_12X12_SRGB_BLOCK;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G8B8G8R8_422_UNORM: vk::Format                                                   = vk::Format::G8B8G8R8_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_B8G8R8G8_422_UNORM: vk::Format                                                   = vk::Format::B8G8R8G8_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: vk::Format                                            = vk::Format::G8_B8_R8_3PLANE_420_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: vk::Format                                             = vk::Format::G8_B8R8_2PLANE_420_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: vk::Format                                            = vk::Format::G8_B8_R8_3PLANE_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: vk::Format                                             = vk::Format::G8_B8R8_2PLANE_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: vk::Format                                            = vk::Format::G8_B8_R8_3PLANE_444_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_R10X6_UNORM_PACK16: vk::Format                                                   = vk::Format::R10X6_UNORM_PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: vk::Format                                             = vk::Format::R10X6G10X6_UNORM_2PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: vk::Format                                   = vk::Format::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: vk::Format                               = vk::Format::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: vk::Format                               = vk::Format::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: vk::Format                           = vk::Format::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: vk::Format                            = vk::Format::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: vk::Format                           = vk::Format::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: vk::Format                            = vk::Format::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: vk::Format                           = vk::Format::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_R12X4_UNORM_PACK16: vk::Format                                                   = vk::Format::R12X4_UNORM_PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: vk::Format                                             = vk::Format::R12X4G12X4_UNORM_2PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: vk::Format                                   = vk::Format::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: vk::Format                               = vk::Format::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: vk::Format                               = vk::Format::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: vk::Format                           = vk::Format::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: vk::Format                            = vk::Format::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: vk::Format                           = vk::Format::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: vk::Format                            = vk::Format::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: vk::Format                           = vk::Format::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G16B16G16R16_422_UNORM: vk::Format                                               = vk::Format::G16B16G16R16_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_B16G16R16G16_422_UNORM: vk::Format                                               = vk::Format::B16G16R16G16_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: vk::Format                                         = vk::Format::G16_B16_R16_3PLANE_420_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: vk::Format                                          = vk::Format::G16_B16R16_2PLANE_420_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: vk::Format                                         = vk::Format::G16_B16_R16_3PLANE_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: vk::Format                                          = vk::Format::G16_B16R16_2PLANE_422_UNORM;
// Provided by VK_VERSION_1_1
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: vk::Format                                         = vk::Format::G16_B16_R16_3PLANE_444_UNORM;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM: vk::Format                                             = vk::Format::G8_B8R8_2PLANE_444_UNORM;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: vk::Format                            = vk::Format::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: vk::Format                            = vk::Format::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM: vk::Format                                          = vk::Format::G16_B16R16_2PLANE_444_UNORM;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16: vk::Format                                                = vk::Format::A4R4G4B4_UNORM_PACK16;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16: vk::Format                                                = vk::Format::A4B4G4R4_UNORM_PACK16;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_4X4_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_5X4_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_5X5_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_6X5_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_6X6_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_8X5_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_8X6_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK: vk::Format                                                = vk::Format::ASTC_8X8_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK: vk::Format                                               = vk::Format::ASTC_10X5_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK: vk::Format                                               = vk::Format::ASTC_10X6_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK: vk::Format                                               = vk::Format::ASTC_10X8_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK: vk::Format                                              = vk::Format::ASTC_10X10_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK: vk::Format                                              = vk::Format::ASTC_12X10_SFLOAT_BLOCK;
// Provided by VK_VERSION_1_3
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK: vk::Format                                              = vk::Format::ASTC_12X12_SFLOAT_BLOCK;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: vk::Format                                          = vk::Format::PVRTC1_2BPP_UNORM_BLOCK_IMG;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: vk::Format                                          = vk::Format::PVRTC1_4BPP_UNORM_BLOCK_IMG;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: vk::Format                                          = vk::Format::PVRTC2_2BPP_UNORM_BLOCK_IMG;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: vk::Format                                          = vk::Format::PVRTC2_4BPP_UNORM_BLOCK_IMG;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: vk::Format                                           = vk::Format::PVRTC1_2BPP_SRGB_BLOCK_IMG;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: vk::Format                                           = vk::Format::PVRTC1_4BPP_SRGB_BLOCK_IMG;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: vk::Format                                           = vk::Format::PVRTC2_2BPP_SRGB_BLOCK_IMG;
// Provided by VK_IMG_format_pvrtc
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: vk::Format                                           = vk::Format::PVRTC2_4BPP_SRGB_BLOCK_IMG;
// Provided by VK_NV_optical_flow
pub const VK_FORMAT_R16G16_S10_5_NV: vk::Format                                                      = vk::Format::R16G16_S10_5_NV;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT: vk::Format                                            = VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT: vk::Format                                           = VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT: vk::Format                                           = VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT: vk::Format                                           = VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT: vk::Format                                          = VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT: vk::Format                                          = VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK;
// Provided by VK_EXT_texture_compression_astc_hdr
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT: vk::Format                                          = VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G8B8G8R8_422_UNORM_KHR: vk::Format                                               = VK_FORMAT_G8B8G8R8_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_B8G8R8G8_422_UNORM_KHR: vk::Format                                               = VK_FORMAT_B8G8R8G8_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR: vk::Format                                        = VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR: vk::Format                                         = VK_FORMAT_G8_B8R8_2PLANE_420_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR: vk::Format                                        = VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR: vk::Format                                         = VK_FORMAT_G8_B8R8_2PLANE_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR: vk::Format                                        = VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_R10X6_UNORM_PACK16_KHR: vk::Format                                               = VK_FORMAT_R10X6_UNORM_PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR: vk::Format                                         = VK_FORMAT_R10X6G10X6_UNORM_2PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: vk::Format                               = VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: vk::Format                           = VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: vk::Format                           = VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: vk::Format                       = VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: vk::Format                        = VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: vk::Format                       = VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: vk::Format                        = VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: vk::Format                       = VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_R12X4_UNORM_PACK16_KHR: vk::Format                                               = VK_FORMAT_R12X4_UNORM_PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR: vk::Format                                         = VK_FORMAT_R12X4G12X4_UNORM_2PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: vk::Format                               = VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: vk::Format                           = VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: vk::Format                           = VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: vk::Format                       = VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: vk::Format                        = VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: vk::Format                       = VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: vk::Format                        = VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: vk::Format                       = VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G16B16G16R16_422_UNORM_KHR: vk::Format                                           = VK_FORMAT_G16B16G16R16_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_B16G16R16G16_422_UNORM_KHR: vk::Format                                           = VK_FORMAT_B16G16R16G16_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR: vk::Format                                     = VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR: vk::Format                                      = VK_FORMAT_G16_B16R16_2PLANE_420_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR: vk::Format                                     = VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR: vk::Format                                      = VK_FORMAT_G16_B16R16_2PLANE_422_UNORM;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR: vk::Format                                     = VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM;
// Provided by VK_EXT_ycbcr_2plane_444_formats
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT: vk::Format                                         = VK_FORMAT_G8_B8R8_2PLANE_444_UNORM;
// Provided by VK_EXT_ycbcr_2plane_444_formats
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: vk::Format                        = VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
// Provided by VK_EXT_ycbcr_2plane_444_formats
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: vk::Format                        = VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
// Provided by VK_EXT_ycbcr_2plane_444_formats
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT: vk::Format                                      = VK_FORMAT_G16_B16R16_2PLANE_444_UNORM;
// Provided by VK_EXT_4444_formats
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT: vk::Format                                            = VK_FORMAT_A4R4G4B4_UNORM_PACK16;
// Provided by VK_EXT_4444_formats
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT: vk::Format                                            = VK_FORMAT_A4B4G4R4_UNORM_PACK16;
// } VkFormat;

// ImageTiling-VkImageTiling:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html
// Provided by VK_VERSION_1_0
// enum VkImageTiling {
pub const VK_IMAGE_TILING_OPTIMAL: vk::ImageTiling                                                   = vk::ImageTiling::OPTIMAL;
pub const VK_IMAGE_TILING_LINEAR: vk::ImageTiling                                                    = vk::ImageTiling::LINEAR;
// Provided by VK_EXT_image_drm_format_modifier
pub const VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT: vk::ImageTiling                                   = vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT;
// } VkImageTiling;

// MemoryPropertyFlags-VkMemoryPropertyFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkMemoryPropertyFlagBits {
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: vk::MemoryPropertyFlags                               = vk::MemoryPropertyFlags::DEVICE_LOCAL;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: vk::MemoryPropertyFlags                               = vk::MemoryPropertyFlags::HOST_VISIBLE;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: vk::MemoryPropertyFlags                              = vk::MemoryPropertyFlags::HOST_COHERENT;
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: vk::MemoryPropertyFlags                                = vk::MemoryPropertyFlags::HOST_CACHED;
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: vk::MemoryPropertyFlags                           = vk::MemoryPropertyFlags::LAZILY_ALLOCATED;
// Provided by VK_VERSION_1_1
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: vk::MemoryPropertyFlags                                  = vk::MemoryPropertyFlags::PROTECTED;
// Provided by VK_AMD_device_coherent_memory
pub const VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD: vk::MemoryPropertyFlags                        = vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD;
// Provided by VK_AMD_device_coherent_memory
pub const VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD: vk::MemoryPropertyFlags                        = vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD;
// Provided by VK_NV_external_memory_rdma
pub const VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV: vk::MemoryPropertyFlags                            = vk::MemoryPropertyFlags::RDMA_CAPABLE_NV;
// } VkMemoryPropertyFlagBits;

// ImageAspectFlags-VkImageAspectFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkImageAspectFlagBits {
pub const VK_IMAGE_ASPECT_COLOR_BIT: vk::ImageAspectFlags                                            = vk::ImageAspectFlags::COLOR;
pub const VK_IMAGE_ASPECT_DEPTH_BIT: vk::ImageAspectFlags                                            = vk::ImageAspectFlags::DEPTH;
pub const VK_IMAGE_ASPECT_STENCIL_BIT: vk::ImageAspectFlags                                          = vk::ImageAspectFlags::STENCIL;
pub const VK_IMAGE_ASPECT_METADATA_BIT: vk::ImageAspectFlags                                         = vk::ImageAspectFlags::METADATA;
// Provided by VK_VERSION_1_1
pub const VK_IMAGE_ASPECT_PLANE_0_BIT: vk::ImageAspectFlags                                          = vk::ImageAspectFlags::PLANE_0;
// Provided by VK_VERSION_1_1
pub const VK_IMAGE_ASPECT_PLANE_1_BIT: vk::ImageAspectFlags                                          = vk::ImageAspectFlags::PLANE_1;
// Provided by VK_VERSION_1_1
pub const VK_IMAGE_ASPECT_PLANE_2_BIT: vk::ImageAspectFlags                                          = vk::ImageAspectFlags::PLANE_2;
// Provided by VK_VERSION_1_3
pub const VK_IMAGE_ASPECT_NONE: vk::ImageAspectFlags                                                 = vk::ImageAspectFlags::NONE;
// Provided by VK_EXT_image_drm_format_modifier
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT: vk::ImageAspectFlags                               = vk::ImageAspectFlags::MEMORY_PLANE_0_EXT;
// Provided by VK_EXT_image_drm_format_modifier
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT: vk::ImageAspectFlags                               = vk::ImageAspectFlags::MEMORY_PLANE_1_EXT;
// Provided by VK_EXT_image_drm_format_modifier
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT: vk::ImageAspectFlags                               = vk::ImageAspectFlags::MEMORY_PLANE_2_EXT;
// Provided by VK_EXT_image_drm_format_modifier
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT: vk::ImageAspectFlags                               = vk::ImageAspectFlags::MEMORY_PLANE_3_EXT;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_IMAGE_ASPECT_PLANE_0_BIT_KHR: vk::ImageAspectFlags                                      = VK_IMAGE_ASPECT_PLANE_0_BIT;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_IMAGE_ASPECT_PLANE_1_BIT_KHR: vk::ImageAspectFlags                                      = VK_IMAGE_ASPECT_PLANE_1_BIT;
// Provided by VK_KHR_sampler_ycbcr_conversion
pub const VK_IMAGE_ASPECT_PLANE_2_BIT_KHR: vk::ImageAspectFlags                                      = VK_IMAGE_ASPECT_PLANE_2_BIT;
// Provided by VK_KHR_maintenance4
pub const VK_IMAGE_ASPECT_NONE_KHR: vk::ImageAspectFlags                                             = VK_IMAGE_ASPECT_NONE;
// } VkImageAspectFlagBits;

// CommandBufferUsageFlags-VkCommandBufferUsageFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkCommandBufferUsageFlagBits {
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: vk::CommandBufferUsageFlags                   = vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT;
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: vk::CommandBufferUsageFlags              = vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE;
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: vk::CommandBufferUsageFlags                  = vk::CommandBufferUsageFlags::SIMULTANEOUS_USE;
// } VkCommandBufferUsageFlagBits;

// BufferUsageFlags-VkBufferUsageFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkBufferUsageFlagBits {
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: vk::BufferUsageFlags                                     = vk::BufferUsageFlags::TRANSFER_SRC;
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: vk::BufferUsageFlags                                     = vk::BufferUsageFlags::TRANSFER_DST;
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: vk::BufferUsageFlags                             = vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER;
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: vk::BufferUsageFlags                             = vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER;
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: vk::BufferUsageFlags                                   = vk::BufferUsageFlags::UNIFORM_BUFFER;
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: vk::BufferUsageFlags                                   = vk::BufferUsageFlags::STORAGE_BUFFER;
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: vk::BufferUsageFlags                                     = vk::BufferUsageFlags::INDEX_BUFFER;
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: vk::BufferUsageFlags                                    = vk::BufferUsageFlags::VERTEX_BUFFER;
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: vk::BufferUsageFlags                                  = vk::BufferUsageFlags::INDIRECT_BUFFER;
// Provided by VK_VERSION_1_2
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT: vk::BufferUsageFlags                            = vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;
// Provided by VK_KHR_video_decode_queue
pub const VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR: vk::BufferUsageFlags                             = vk::BufferUsageFlags::VIDEO_DECODE_SRC_KHR;
// Provided by VK_KHR_video_decode_queue
pub const VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR: vk::BufferUsageFlags                             = vk::BufferUsageFlags::VIDEO_DECODE_DST_KHR;
// Provided by VK_EXT_transform_feedback
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: vk::BufferUsageFlags                    = vk::BufferUsageFlags::TRANSFORM_FEEDBACK_BUFFER_EXT;
// Provided by VK_EXT_transform_feedback
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT: vk::BufferUsageFlags            = vk::BufferUsageFlags::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT;
// Provided by VK_EXT_conditional_rendering
pub const VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT: vk::BufferUsageFlags                        = vk::BufferUsageFlags::CONDITIONAL_RENDERING_EXT;
// Provided by VK_KHR_acceleration_structure
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR: vk::BufferUsageFlags = vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR;
// Provided by VK_KHR_acceleration_structure
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: vk::BufferUsageFlags               = vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR: vk::BufferUsageFlags                         = vk::BufferUsageFlags::SHADER_BINDING_TABLE_KHR;
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR: vk::BufferUsageFlags                             = vk::BufferUsageFlags::VIDEO_ENCODE_DST_KHR;
// #endif
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR: vk::BufferUsageFlags                             = vk::BufferUsageFlags::VIDEO_ENCODE_SRC_KHR;
// #endif
// Provided by VK_EXT_descriptor_buffer
pub const VK_BUFFER_USAGE_SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT: vk::BufferUsageFlags                    = vk::BufferUsageFlags::SAMPLER_DESCRIPTOR_BUFFER_EXT;
// Provided by VK_EXT_descriptor_buffer
pub const VK_BUFFER_USAGE_RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT: vk::BufferUsageFlags                   = vk::BufferUsageFlags::RESOURCE_DESCRIPTOR_BUFFER_EXT;
// Provided by VK_EXT_descriptor_buffer
pub const VK_BUFFER_USAGE_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT: vk::BufferUsageFlags           = vk::BufferUsageFlags::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT;
// Provided by VK_EXT_opacity_micromap
pub const VK_BUFFER_USAGE_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT: vk::BufferUsageFlags               = vk::BufferUsageFlags::MICROMAP_BUILD_INPUT_READ_ONLY_EXT;
// Provided by VK_EXT_opacity_micromap
pub const VK_BUFFER_USAGE_MICROMAP_STORAGE_BIT_EXT: vk::BufferUsageFlags                             = vk::BufferUsageFlags::MICROMAP_STORAGE_EXT;
// Provided by VK_NV_ray_tracing
pub const VK_BUFFER_USAGE_RAY_TRACING_BIT_NV: vk::BufferUsageFlags                                   = VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR;
// Provided by VK_EXT_buffer_device_address
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT: vk::BufferUsageFlags                        = VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
// Provided by VK_KHR_buffer_device_address
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR: vk::BufferUsageFlags                        = VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
// } VkBufferUsageFlagBits;

// AccessFlags-VkAccessFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkAccessFlagBits {
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: vk::AccessFlags                                       = vk::AccessFlags::INDIRECT_COMMAND_READ;
pub const VK_ACCESS_INDEX_READ_BIT: vk::AccessFlags                                                  = vk::AccessFlags::INDEX_READ;
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: vk::AccessFlags                                       = vk::AccessFlags::VERTEX_ATTRIBUTE_READ;
pub const VK_ACCESS_UNIFORM_READ_BIT: vk::AccessFlags                                                = vk::AccessFlags::UNIFORM_READ;
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: vk::AccessFlags                                       = vk::AccessFlags::INPUT_ATTACHMENT_READ;
pub const VK_ACCESS_SHADER_READ_BIT: vk::AccessFlags                                                 = vk::AccessFlags::SHADER_READ;
pub const VK_ACCESS_SHADER_WRITE_BIT: vk::AccessFlags                                                = vk::AccessFlags::SHADER_WRITE;
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: vk::AccessFlags                                       = vk::AccessFlags::COLOR_ATTACHMENT_READ;
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: vk::AccessFlags                                      = vk::AccessFlags::COLOR_ATTACHMENT_WRITE;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: vk::AccessFlags                               = vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: vk::AccessFlags                              = vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
pub const VK_ACCESS_TRANSFER_READ_BIT: vk::AccessFlags                                               = vk::AccessFlags::TRANSFER_READ;
pub const VK_ACCESS_TRANSFER_WRITE_BIT: vk::AccessFlags                                              = vk::AccessFlags::TRANSFER_WRITE;
pub const VK_ACCESS_HOST_READ_BIT: vk::AccessFlags                                                   = vk::AccessFlags::HOST_READ;
pub const VK_ACCESS_HOST_WRITE_BIT: vk::AccessFlags                                                  = vk::AccessFlags::HOST_WRITE;
pub const VK_ACCESS_MEMORY_READ_BIT: vk::AccessFlags                                                 = vk::AccessFlags::MEMORY_READ;
pub const VK_ACCESS_MEMORY_WRITE_BIT: vk::AccessFlags                                                = vk::AccessFlags::MEMORY_WRITE;
// Provided by VK_VERSION_1_3
pub const VK_ACCESS_NONE: vk::AccessFlags                                                            = vk::AccessFlags::NONE;
// Provided by VK_EXT_transform_feedback
pub const VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT: vk::AccessFlags                                = vk::AccessFlags::TRANSFORM_FEEDBACK_WRITE_EXT;
// Provided by VK_EXT_transform_feedback
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: vk::AccessFlags                         = vk::AccessFlags::TRANSFORM_FEEDBACK_COUNTER_READ_EXT;
// Provided by VK_EXT_transform_feedback
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: vk::AccessFlags                        = vk::AccessFlags::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT;
// Provided by VK_EXT_conditional_rendering
pub const VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT: vk::AccessFlags                              = vk::AccessFlags::CONDITIONAL_RENDERING_READ_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: vk::AccessFlags                       = vk::AccessFlags::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT;
// Provided by VK_KHR_acceleration_structure
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR: vk::AccessFlags                             = vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR;
// Provided by VK_KHR_acceleration_structure
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR: vk::AccessFlags                            = vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR;
// Provided by VK_EXT_fragment_density_map
pub const VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT: vk::AccessFlags                               = vk::AccessFlags::FRAGMENT_DENSITY_MAP_READ_EXT;
// Provided by VK_KHR_fragment_shading_rate
pub const VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: vk::AccessFlags                   = vk::AccessFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
// Provided by VK_NV_device_generated_commands
pub const VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV: vk::AccessFlags                                  = vk::AccessFlags::COMMAND_PREPROCESS_READ_NV;
// Provided by VK_NV_device_generated_commands
pub const VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV: vk::AccessFlags                                 = vk::AccessFlags::COMMAND_PREPROCESS_WRITE_NV;
// Provided by VK_NV_shading_rate_image
pub const VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV: vk::AccessFlags                                  = VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV: vk::AccessFlags                              = VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV: vk::AccessFlags                             = VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR;
// Provided by VK_KHR_synchronization2
pub const VK_ACCESS_NONE_KHR: vk::AccessFlags                                                        = VK_ACCESS_NONE;
// } VkAccessFlagBits;

// PipelineStageFlags-VkPipelineStageFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkPipelineStageFlagBits {
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: vk::PipelineStageFlags                                  = vk::PipelineStageFlags::TOP_OF_PIPE;
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: vk::PipelineStageFlags                                = vk::PipelineStageFlags::DRAW_INDIRECT;
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: vk::PipelineStageFlags                                 = vk::PipelineStageFlags::VERTEX_INPUT;
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: vk::PipelineStageFlags                                = vk::PipelineStageFlags::VERTEX_SHADER;
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: vk::PipelineStageFlags                  = vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER;
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: vk::PipelineStageFlags               = vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER;
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: vk::PipelineStageFlags                              = vk::PipelineStageFlags::GEOMETRY_SHADER;
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: vk::PipelineStageFlags                              = vk::PipelineStageFlags::FRAGMENT_SHADER;
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: vk::PipelineStageFlags                         = vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS;
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: vk::PipelineStageFlags                          = vk::PipelineStageFlags::LATE_FRAGMENT_TESTS;
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: vk::PipelineStageFlags                      = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: vk::PipelineStageFlags                               = vk::PipelineStageFlags::COMPUTE_SHADER;
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: vk::PipelineStageFlags                                     = vk::PipelineStageFlags::TRANSFER;
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: vk::PipelineStageFlags                               = vk::PipelineStageFlags::BOTTOM_OF_PIPE;
pub const VK_PIPELINE_STAGE_HOST_BIT: vk::PipelineStageFlags                                         = vk::PipelineStageFlags::HOST;
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: vk::PipelineStageFlags                                 = vk::PipelineStageFlags::ALL_GRAPHICS;
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: vk::PipelineStageFlags                                 = vk::PipelineStageFlags::ALL_COMMANDS;
// Provided by VK_VERSION_1_3
pub const VK_PIPELINE_STAGE_NONE: vk::PipelineStageFlags                                             = vk::PipelineStageFlags::NONE;
// Provided by VK_EXT_transform_feedback
pub const VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT: vk::PipelineStageFlags                       = vk::PipelineStageFlags::TRANSFORM_FEEDBACK_EXT;
// Provided by VK_EXT_conditional_rendering
pub const VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT: vk::PipelineStageFlags                    = vk::PipelineStageFlags::CONDITIONAL_RENDERING_EXT;
// Provided by VK_KHR_acceleration_structure
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR: vk::PipelineStageFlags             = vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR: vk::PipelineStageFlags                       = vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR;
// Provided by VK_EXT_fragment_density_map
pub const VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT: vk::PipelineStageFlags                 = vk::PipelineStageFlags::FRAGMENT_DENSITY_PROCESS_EXT;
// Provided by VK_KHR_fragment_shading_rate
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: vk::PipelineStageFlags         = vk::PipelineStageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
// Provided by VK_NV_device_generated_commands
pub const VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV: vk::PipelineStageFlags                        = vk::PipelineStageFlags::COMMAND_PREPROCESS_NV;
// Provided by VK_EXT_mesh_shader
pub const VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT: vk::PipelineStageFlags                              = vk::PipelineStageFlags::TASK_SHADER_EXT;
// Provided by VK_EXT_mesh_shader
pub const VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT: vk::PipelineStageFlags                              = vk::PipelineStageFlags::MESH_SHADER_EXT;
// Provided by VK_NV_shading_rate_image
pub const VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV: vk::PipelineStageFlags                        = VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV: vk::PipelineStageFlags                        = VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV: vk::PipelineStageFlags              = VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR;
// Provided by VK_NV_mesh_shader
pub const VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV: vk::PipelineStageFlags                               = VK_PIPELINE_STAGE_TASK_SHADER_BIT_EXT;
// Provided by VK_NV_mesh_shader
pub const VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV: vk::PipelineStageFlags                               = VK_PIPELINE_STAGE_MESH_SHADER_BIT_EXT;
// Provided by VK_KHR_synchronization2
pub const VK_PIPELINE_STAGE_NONE_KHR: vk::PipelineStageFlags                                         = VK_PIPELINE_STAGE_NONE;
// } VkPipelineStageFlagBits;

// DependencyFlags-VkDependencyFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkDependencyFlagBits {
pub const VK_DEPENDENCY_BY_REGION_BIT: vk::DependencyFlags                                           = vk::DependencyFlags::BY_REGION;
// Provided by VK_VERSION_1_1
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT: vk::DependencyFlags                                        = vk::DependencyFlags::DEVICE_GROUP;
// Provided by VK_VERSION_1_1
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: vk::DependencyFlags                                          = vk::DependencyFlags::VIEW_LOCAL;
// Provided by VK_EXT_attachment_feedback_loop_layout
pub const VK_DEPENDENCY_FEEDBACK_LOOP_BIT_EXT: vk::DependencyFlags                                   = vk::DependencyFlags::FEEDBACK_LOOP_EXT;
// Provided by VK_KHR_multiview
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR: vk::DependencyFlags                                      = VK_DEPENDENCY_VIEW_LOCAL_BIT;
// Provided by VK_KHR_device_group
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR: vk::DependencyFlags                                    = VK_DEPENDENCY_DEVICE_GROUP_BIT;
// } VkDependencyFlagBits;

// ComponentSwizzle-VkComponentSwizzle:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html
// Provided by VK_VERSION_1_0
// enum VkComponentSwizzle {
pub const VK_COMPONENT_SWIZZLE_IDENTITY: vk::ComponentSwizzle                                        = vk::ComponentSwizzle::IDENTITY;
pub const VK_COMPONENT_SWIZZLE_ZERO: vk::ComponentSwizzle                                            = vk::ComponentSwizzle::ZERO;
pub const VK_COMPONENT_SWIZZLE_ONE: vk::ComponentSwizzle                                             = vk::ComponentSwizzle::ONE;
pub const VK_COMPONENT_SWIZZLE_R: vk::ComponentSwizzle                                               = vk::ComponentSwizzle::R;
pub const VK_COMPONENT_SWIZZLE_G: vk::ComponentSwizzle                                               = vk::ComponentSwizzle::G;
pub const VK_COMPONENT_SWIZZLE_B: vk::ComponentSwizzle                                               = vk::ComponentSwizzle::B;
pub const VK_COMPONENT_SWIZZLE_A: vk::ComponentSwizzle                                               = vk::ComponentSwizzle::A;
// } VkComponentSwizzle;

// SharingMode-VkSharingMode:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html
// Provided by VK_VERSION_1_0
// enum VkSharingMode {
pub const VK_SHARING_MODE_EXCLUSIVE: vk::SharingMode                                                 = vk::SharingMode::EXCLUSIVE;
pub const VK_SHARING_MODE_CONCURRENT: vk::SharingMode                                                = vk::SharingMode::CONCURRENT;
// } VkSharingMode;

// CompositeAlphaFlagsKHR-VkCompositeAlphaFlagBitsKHR:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html
// Provided by VK_KHR_surface
// enum VkCompositeAlphaFlagBitsKHR {
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: vk::CompositeAlphaFlagsKHR                              = vk::CompositeAlphaFlagsKHR::OPAQUE;
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: vk::CompositeAlphaFlagsKHR                      = vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED;
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: vk::CompositeAlphaFlagsKHR                     = vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED;
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: vk::CompositeAlphaFlagsKHR                             = vk::CompositeAlphaFlagsKHR::INHERIT;
// } VkCompositeAlphaFlagBitsKHR;

// PresentModeKHR-VkPresentModeKHR:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html
// Provided by VK_KHR_surface
// enum VkPresentModeKHR {
pub const VK_PRESENT_MODE_IMMEDIATE_KHR: vk::PresentModeKHR                                          = vk::PresentModeKHR::IMMEDIATE;
pub const VK_PRESENT_MODE_MAILBOX_KHR: vk::PresentModeKHR                                            = vk::PresentModeKHR::MAILBOX;
pub const VK_PRESENT_MODE_FIFO_KHR: vk::PresentModeKHR                                               = vk::PresentModeKHR::FIFO;
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: vk::PresentModeKHR                                       = vk::PresentModeKHR::FIFO_RELAXED;
// Provided by VK_KHR_shared_presentable_image
pub const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: vk::PresentModeKHR                              = vk::PresentModeKHR::SHARED_DEMAND_REFRESH;
// Provided by VK_KHR_shared_presentable_image
pub const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: vk::PresentModeKHR                          = vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH;
// } VkPresentModeKHR;

// ImageViewType-VkImageViewType:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html
// Provided by VK_VERSION_1_0
// enum VkImageViewType {
pub const VK_IMAGE_VIEW_TYPE_1D: vk::ImageViewType                                                   = vk::ImageViewType::TYPE_1D;
pub const VK_IMAGE_VIEW_TYPE_2D: vk::ImageViewType                                                   = vk::ImageViewType::TYPE_2D;
pub const VK_IMAGE_VIEW_TYPE_3D: vk::ImageViewType                                                   = vk::ImageViewType::TYPE_3D;
pub const VK_IMAGE_VIEW_TYPE_CUBE: vk::ImageViewType                                                 = vk::ImageViewType::CUBE;
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: vk::ImageViewType                                             = vk::ImageViewType::TYPE_1D_ARRAY;
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: vk::ImageViewType                                             = vk::ImageViewType::TYPE_2D_ARRAY;
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: vk::ImageViewType                                           = vk::ImageViewType::CUBE_ARRAY;
// } VkImageViewType;

// ShaderStageFlags-VkShaderStageFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkShaderStageFlagBits {
pub const VK_SHADER_STAGE_VERTEX_BIT: vk::ShaderStageFlags                                           = vk::ShaderStageFlags::VERTEX;
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: vk::ShaderStageFlags                             = vk::ShaderStageFlags::TESSELLATION_CONTROL;
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: vk::ShaderStageFlags                          = vk::ShaderStageFlags::TESSELLATION_EVALUATION;
pub const VK_SHADER_STAGE_GEOMETRY_BIT: vk::ShaderStageFlags                                         = vk::ShaderStageFlags::GEOMETRY;
pub const VK_SHADER_STAGE_FRAGMENT_BIT: vk::ShaderStageFlags                                         = vk::ShaderStageFlags::FRAGMENT;
pub const VK_SHADER_STAGE_COMPUTE_BIT: vk::ShaderStageFlags                                          = vk::ShaderStageFlags::COMPUTE;
pub const VK_SHADER_STAGE_ALL_GRAPHICS: vk::ShaderStageFlags                                         = vk::ShaderStageFlags::ALL_GRAPHICS;
pub const VK_SHADER_STAGE_ALL: vk::ShaderStageFlags                                                  = vk::ShaderStageFlags::ALL;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_RAYGEN_BIT_KHR: vk::ShaderStageFlags                                       = vk::ShaderStageFlags::RAYGEN_KHR;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_ANY_HIT_BIT_KHR: vk::ShaderStageFlags                                      = vk::ShaderStageFlags::ANY_HIT_KHR;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR: vk::ShaderStageFlags                                  = vk::ShaderStageFlags::CLOSEST_HIT_KHR;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_MISS_BIT_KHR: vk::ShaderStageFlags                                         = vk::ShaderStageFlags::MISS_KHR;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_INTERSECTION_BIT_KHR: vk::ShaderStageFlags                                 = vk::ShaderStageFlags::INTERSECTION_KHR;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_CALLABLE_BIT_KHR: vk::ShaderStageFlags                                     = vk::ShaderStageFlags::CALLABLE_KHR;
// Provided by VK_EXT_mesh_shader
pub const VK_SHADER_STAGE_TASK_BIT_EXT: vk::ShaderStageFlags                                         = vk::ShaderStageFlags::TASK_EXT;
// Provided by VK_EXT_mesh_shader
pub const VK_SHADER_STAGE_MESH_BIT_EXT: vk::ShaderStageFlags                                         = vk::ShaderStageFlags::MESH_EXT;
// Provided by VK_HUAWEI_subpass_shading
pub const VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI: vk::ShaderStageFlags                           = vk::ShaderStageFlags::SUBPASS_SHADING_HUAWEI;
// Provided by VK_HUAWEI_cluster_culling_shader
//? VK_SHADER_STAGE_CLUSTER_CULLING_BIT_HUAWEI = 0x00080000
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_RAYGEN_BIT_NV: vk::ShaderStageFlags                                        = VK_SHADER_STAGE_RAYGEN_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_ANY_HIT_BIT_NV: vk::ShaderStageFlags                                       = VK_SHADER_STAGE_ANY_HIT_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV: vk::ShaderStageFlags                                   = VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_MISS_BIT_NV: vk::ShaderStageFlags                                          = VK_SHADER_STAGE_MISS_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_INTERSECTION_BIT_NV: vk::ShaderStageFlags                                  = VK_SHADER_STAGE_INTERSECTION_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_CALLABLE_BIT_NV: vk::ShaderStageFlags                                      = VK_SHADER_STAGE_CALLABLE_BIT_KHR;
// Provided by VK_NV_mesh_shader
pub const VK_SHADER_STAGE_TASK_BIT_NV: vk::ShaderStageFlags                                          = VK_SHADER_STAGE_TASK_BIT_EXT;
// Provided by VK_NV_mesh_shader
pub const VK_SHADER_STAGE_MESH_BIT_NV: vk::ShaderStageFlags                                          = VK_SHADER_STAGE_MESH_BIT_EXT;
// } VkShaderStageFlagBits;

// FenceCreateFlags-VkFenceCreateFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkFenceCreateFlagBits {
pub const VK_FENCE_CREATE_SIGNALED_BIT: vk::FenceCreateFlags                                         = vk::FenceCreateFlags::SIGNALED;
// } VkFenceCreateFlagBits;

// Filter-VkFilter:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilter.html
// Provided by VK_VERSION_1_0
// enum VkFilter {
pub const VK_FILTER_NEAREST: vk::Filter                                                              = vk::Filter::NEAREST;
pub const VK_FILTER_LINEAR: vk::Filter                                                               = vk::Filter::LINEAR;
// Provided by VK_EXT_filter_cubic
pub const VK_FILTER_CUBIC_EXT: vk::Filter                                                            = vk::Filter::CUBIC_EXT;
// Provided by VK_IMG_filter_cubic
pub const VK_FILTER_CUBIC_IMG: vk::Filter                                                            = VK_FILTER_CUBIC_EXT;
// } VkFilter;

// SamplerAddressMode-VkSamplerAddressMode:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html
// Provided by VK_VERSION_1_0
// enum VkSamplerAddressMode {
pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: vk::SamplerAddressMode                                     = vk::SamplerAddressMode::REPEAT;
pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: vk::SamplerAddressMode                            = vk::SamplerAddressMode::MIRRORED_REPEAT;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: vk::SamplerAddressMode                              = vk::SamplerAddressMode::CLAMP_TO_EDGE;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: vk::SamplerAddressMode                            = vk::SamplerAddressMode::CLAMP_TO_BORDER;
// Provided by VK_VERSION_1_2, VK_KHR_sampler_mirror_clamp_to_edge
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: vk::SamplerAddressMode                       = vk::SamplerAddressMode::MIRROR_CLAMP_TO_EDGE;
// Provided by VK_KHR_sampler_mirror_clamp_to_edge
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR: vk::SamplerAddressMode                   = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE;
// } VkSamplerAddressMode;

// BorderColor-VkBorderColor:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html
// Provided by VK_VERSION_1_0
// enum VkBorderColor {
pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: vk::BorderColor                                   = vk::BorderColor::FLOAT_TRANSPARENT_BLACK;
pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: vk::BorderColor                                     = vk::BorderColor::INT_TRANSPARENT_BLACK;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: vk::BorderColor                                        = vk::BorderColor::FLOAT_OPAQUE_BLACK;
pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: vk::BorderColor                                          = vk::BorderColor::INT_OPAQUE_BLACK;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: vk::BorderColor                                        = vk::BorderColor::FLOAT_OPAQUE_WHITE;
pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: vk::BorderColor                                          = vk::BorderColor::INT_OPAQUE_WHITE;
// Provided by VK_EXT_custom_border_color
pub const VK_BORDER_COLOR_FLOAT_CUSTOM_EXT: vk::BorderColor                                          = vk::BorderColor::FLOAT_CUSTOM_EXT;
// Provided by VK_EXT_custom_border_color
pub const VK_BORDER_COLOR_INT_CUSTOM_EXT: vk::BorderColor                                            = vk::BorderColor::INT_CUSTOM_EXT;
// } VkBorderColor;

// CompareOp-VkCompareOp:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html
// Provided by VK_VERSION_1_0
// enum VkCompareOp {
pub const VK_COMPARE_OP_NEVER: vk::CompareOp                                                         = vk::CompareOp::NEVER;
pub const VK_COMPARE_OP_LESS: vk::CompareOp                                                          = vk::CompareOp::LESS;
pub const VK_COMPARE_OP_EQUAL: vk::CompareOp                                                         = vk::CompareOp::EQUAL;
pub const VK_COMPARE_OP_LESS_OR_EQUAL: vk::CompareOp                                                 = vk::CompareOp::LESS_OR_EQUAL;
pub const VK_COMPARE_OP_GREATER: vk::CompareOp                                                       = vk::CompareOp::GREATER;
pub const VK_COMPARE_OP_NOT_EQUAL: vk::CompareOp                                                     = vk::CompareOp::NOT_EQUAL;
pub const VK_COMPARE_OP_GREATER_OR_EQUAL: vk::CompareOp                                              = vk::CompareOp::GREATER_OR_EQUAL;
pub const VK_COMPARE_OP_ALWAYS: vk::CompareOp                                                        = vk::CompareOp::ALWAYS;
// } VkCompareOp;

// SamplerMipmapMode-VkSamplerMipmapMode:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html
// Provided by VK_VERSION_1_0
// enum VkSamplerMipmapMode {
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: vk::SamplerMipmapMode                                      = vk::SamplerMipmapMode::NEAREST;
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: vk::SamplerMipmapMode                                       = vk::SamplerMipmapMode::LINEAR;
// } VkSamplerMipmapMode;

// DescriptorType-VkDescriptorType:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html
// Provided by VK_VERSION_1_0
// enum VkDescriptorType {
pub const VK_DESCRIPTOR_TYPE_SAMPLER: vk::DescriptorType                                             = vk::DescriptorType::SAMPLER;
pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: vk::DescriptorType                              = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;
pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: vk::DescriptorType                                       = vk::DescriptorType::SAMPLED_IMAGE;
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: vk::DescriptorType                                       = vk::DescriptorType::STORAGE_IMAGE;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: vk::DescriptorType                                = vk::DescriptorType::UNIFORM_TEXEL_BUFFER;
pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: vk::DescriptorType                                = vk::DescriptorType::STORAGE_TEXEL_BUFFER;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: vk::DescriptorType                                      = vk::DescriptorType::UNIFORM_BUFFER;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: vk::DescriptorType                                      = vk::DescriptorType::STORAGE_BUFFER;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: vk::DescriptorType                              = vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: vk::DescriptorType                              = vk::DescriptorType::STORAGE_BUFFER_DYNAMIC;
pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: vk::DescriptorType                                    = vk::DescriptorType::INPUT_ATTACHMENT;
// Provided by VK_VERSION_1_3
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK: vk::DescriptorType                                = vk::DescriptorType::INLINE_UNIFORM_BLOCK;
// Provided by VK_KHR_acceleration_structure
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR: vk::DescriptorType                          = vk::DescriptorType::ACCELERATION_STRUCTURE_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV: vk::DescriptorType                           = vk::DescriptorType::ACCELERATION_STRUCTURE_NV;
// Provided by VK_QCOM_image_processing
pub const VK_DESCRIPTOR_TYPE_SAMPLE_WEIGHT_IMAGE_QCOM: vk::DescriptorType                            = vk::DescriptorType::SAMPLE_WEIGHT_IMAGE_QCOM;
// Provided by VK_QCOM_image_processing
pub const VK_DESCRIPTOR_TYPE_BLOCK_MATCH_IMAGE_QCOM: vk::DescriptorType                              = vk::DescriptorType::BLOCK_MATCH_IMAGE_QCOM;
// Provided by VK_EXT_mutable_descriptor_type
pub const VK_DESCRIPTOR_TYPE_MUTABLE_EXT: vk::DescriptorType                                         = vk::DescriptorType::MUTABLE_EXT;
// Provided by VK_EXT_inline_uniform_block
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT: vk::DescriptorType                            = VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK;
// Provided by VK_VALVE_mutable_descriptor_type
pub const VK_DESCRIPTOR_TYPE_MUTABLE_VALVE: vk::DescriptorType                                       = VK_DESCRIPTOR_TYPE_MUTABLE_EXT;
// } VkDescriptorType;

// VertexInputRate-VkVertexInputRate:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVertexInputRate.html
// Provided by VK_VERSION_1_0
// enum VkVertexInputRate {
pub const VK_VERTEX_INPUT_RATE_VERTEX: vk::VertexInputRate                                           = vk::VertexInputRate::VERTEX;
pub const VK_VERTEX_INPUT_RATE_INSTANCE: vk::VertexInputRate                                         = vk::VertexInputRate::INSTANCE;
// } VkVertexInputRate;

// FrontFace-VkFrontFace:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html
// Provided by VK_VERSION_1_0
// enum VkFrontFace {
pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: vk::FrontFace                                             = vk::FrontFace::COUNTER_CLOCKWISE;
pub const VK_FRONT_FACE_CLOCKWISE: vk::FrontFace                                                     = vk::FrontFace::CLOCKWISE;
// } VkFrontFace;

// CullModeFlags-VkCullModeFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkCullModeFlagBits {
pub const VK_CULL_MODE_NONE: vk::CullModeFlags                                                       = vk::CullModeFlags::NONE;
pub const VK_CULL_MODE_FRONT_BIT: vk::CullModeFlags                                                  = vk::CullModeFlags::FRONT;
pub const VK_CULL_MODE_BACK_BIT: vk::CullModeFlags                                                   = vk::CullModeFlags::BACK;
pub const VK_CULL_MODE_FRONT_AND_BACK: vk::CullModeFlags                                             = vk::CullModeFlags::FRONT_AND_BACK;
// } VkCullModeFlagBits;

// PolygonMode-VkPolygonMode:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPolygonMode.html
// Provided by VK_VERSION_1_0
// enum VkPolygonMode {
pub const VK_POLYGON_MODE_FILL: vk::PolygonMode                                                      = vk::PolygonMode::FILL;
pub const VK_POLYGON_MODE_LINE: vk::PolygonMode                                                      = vk::PolygonMode::LINE;
pub const VK_POLYGON_MODE_POINT: vk::PolygonMode                                                     = vk::PolygonMode::POINT;
// Provided by VK_NV_fill_rectangle
pub const VK_POLYGON_MODE_FILL_RECTANGLE_NV: vk::PolygonMode                                         = vk::PolygonMode::FILL_RECTANGLE_NV;
// } VkPolygonMode;

// SampleCountFlags-VkSampleCountFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkSampleCountFlagBits {
pub const VK_SAMPLE_COUNT_1_BIT: vk::SampleCountFlags                                                = vk::SampleCountFlags::TYPE_1;
pub const VK_SAMPLE_COUNT_2_BIT: vk::SampleCountFlags                                                = vk::SampleCountFlags::TYPE_2;
pub const VK_SAMPLE_COUNT_4_BIT: vk::SampleCountFlags                                                = vk::SampleCountFlags::TYPE_4;
pub const VK_SAMPLE_COUNT_8_BIT: vk::SampleCountFlags                                                = vk::SampleCountFlags::TYPE_8;
pub const VK_SAMPLE_COUNT_16_BIT: vk::SampleCountFlags                                               = vk::SampleCountFlags::TYPE_16;
pub const VK_SAMPLE_COUNT_32_BIT: vk::SampleCountFlags                                               = vk::SampleCountFlags::TYPE_32;
pub const VK_SAMPLE_COUNT_64_BIT: vk::SampleCountFlags                                               = vk::SampleCountFlags::TYPE_64;
// } VkSampleCountFlagBits;

// BlendOp-VkBlendOp:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html
// Provided by VK_VERSION_1_0
// enum VkBlendOp {
pub const VK_BLEND_OP_ADD: vk::BlendOp                                                               = vk::BlendOp::ADD;
pub const VK_BLEND_OP_SUBTRACT: vk::BlendOp                                                          = vk::BlendOp::SUBTRACT;
pub const VK_BLEND_OP_REVERSE_SUBTRACT: vk::BlendOp                                                  = vk::BlendOp::REVERSE_SUBTRACT;
pub const VK_BLEND_OP_MIN: vk::BlendOp                                                               = vk::BlendOp::MIN;
pub const VK_BLEND_OP_MAX: vk::BlendOp                                                               = vk::BlendOp::MAX;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_ZERO_EXT: vk::BlendOp                                                          = vk::BlendOp::ZERO_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_SRC_EXT: vk::BlendOp                                                           = vk::BlendOp::SRC_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_DST_EXT: vk::BlendOp                                                           = vk::BlendOp::DST_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_SRC_OVER_EXT: vk::BlendOp                                                      = vk::BlendOp::SRC_OVER_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_DST_OVER_EXT: vk::BlendOp                                                      = vk::BlendOp::DST_OVER_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_SRC_IN_EXT: vk::BlendOp                                                        = vk::BlendOp::SRC_IN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_DST_IN_EXT: vk::BlendOp                                                        = vk::BlendOp::DST_IN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_SRC_OUT_EXT: vk::BlendOp                                                       = vk::BlendOp::SRC_OUT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_DST_OUT_EXT: vk::BlendOp                                                       = vk::BlendOp::DST_OUT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_SRC_ATOP_EXT: vk::BlendOp                                                      = vk::BlendOp::SRC_ATOP_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_DST_ATOP_EXT: vk::BlendOp                                                      = vk::BlendOp::DST_ATOP_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_XOR_EXT: vk::BlendOp                                                           = vk::BlendOp::XOR_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_MULTIPLY_EXT: vk::BlendOp                                                      = vk::BlendOp::MULTIPLY_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_SCREEN_EXT: vk::BlendOp                                                        = vk::BlendOp::SCREEN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_OVERLAY_EXT: vk::BlendOp                                                       = vk::BlendOp::OVERLAY_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_DARKEN_EXT: vk::BlendOp                                                        = vk::BlendOp::DARKEN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_LIGHTEN_EXT: vk::BlendOp                                                       = vk::BlendOp::LIGHTEN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_COLORDODGE_EXT: vk::BlendOp                                                    = vk::BlendOp::COLORDODGE_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_COLORBURN_EXT: vk::BlendOp                                                     = vk::BlendOp::COLORBURN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_HARDLIGHT_EXT: vk::BlendOp                                                     = vk::BlendOp::HARDLIGHT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_SOFTLIGHT_EXT: vk::BlendOp                                                     = vk::BlendOp::SOFTLIGHT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_DIFFERENCE_EXT: vk::BlendOp                                                    = vk::BlendOp::DIFFERENCE_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_EXCLUSION_EXT: vk::BlendOp                                                     = vk::BlendOp::EXCLUSION_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_INVERT_EXT: vk::BlendOp                                                        = vk::BlendOp::INVERT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_INVERT_RGB_EXT: vk::BlendOp                                                    = vk::BlendOp::INVERT_RGB_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_LINEARDODGE_EXT: vk::BlendOp                                                   = vk::BlendOp::LINEARDODGE_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_LINEARBURN_EXT: vk::BlendOp                                                    = vk::BlendOp::LINEARBURN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_VIVIDLIGHT_EXT: vk::BlendOp                                                    = vk::BlendOp::VIVIDLIGHT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_LINEARLIGHT_EXT: vk::BlendOp                                                   = vk::BlendOp::LINEARLIGHT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_PINLIGHT_EXT: vk::BlendOp                                                      = vk::BlendOp::PINLIGHT_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_HARDMIX_EXT: vk::BlendOp                                                       = vk::BlendOp::HARDMIX_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_HSL_HUE_EXT: vk::BlendOp                                                       = vk::BlendOp::HSL_HUE_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_HSL_SATURATION_EXT: vk::BlendOp                                                = vk::BlendOp::HSL_SATURATION_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_HSL_COLOR_EXT: vk::BlendOp                                                     = vk::BlendOp::HSL_COLOR_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_HSL_LUMINOSITY_EXT: vk::BlendOp                                                = vk::BlendOp::HSL_LUMINOSITY_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_PLUS_EXT: vk::BlendOp                                                          = vk::BlendOp::PLUS_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_PLUS_CLAMPED_EXT: vk::BlendOp                                                  = vk::BlendOp::PLUS_CLAMPED_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT: vk::BlendOp                                            = vk::BlendOp::PLUS_CLAMPED_ALPHA_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_PLUS_DARKER_EXT: vk::BlendOp                                                   = vk::BlendOp::PLUS_DARKER_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_MINUS_EXT: vk::BlendOp                                                         = vk::BlendOp::MINUS_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_MINUS_CLAMPED_EXT: vk::BlendOp                                                 = vk::BlendOp::MINUS_CLAMPED_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_CONTRAST_EXT: vk::BlendOp                                                      = vk::BlendOp::CONTRAST_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_INVERT_OVG_EXT: vk::BlendOp                                                    = vk::BlendOp::INVERT_OVG_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_RED_EXT: vk::BlendOp                                                           = vk::BlendOp::RED_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_GREEN_EXT: vk::BlendOp                                                         = vk::BlendOp::GREEN_EXT;
// Provided by VK_EXT_blend_operation_advanced
pub const VK_BLEND_OP_BLUE_EXT: vk::BlendOp                                                          = vk::BlendOp::BLUE_EXT;
// } VkBlendOp;

// BlendFactor-VkBlendFactor:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html
// Provided by VK_VERSION_1_0
// enum VkBlendFactor {
pub const VK_BLEND_FACTOR_ZERO: vk::BlendFactor                                                      = vk::BlendFactor::ZERO;
pub const VK_BLEND_FACTOR_ONE: vk::BlendFactor                                                       = vk::BlendFactor::ONE;
pub const VK_BLEND_FACTOR_SRC_COLOR: vk::BlendFactor                                                 = vk::BlendFactor::SRC_COLOR;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: vk::BlendFactor                                       = vk::BlendFactor::ONE_MINUS_SRC_COLOR;
pub const VK_BLEND_FACTOR_DST_COLOR: vk::BlendFactor                                                 = vk::BlendFactor::DST_COLOR;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: vk::BlendFactor                                       = vk::BlendFactor::ONE_MINUS_DST_COLOR;
pub const VK_BLEND_FACTOR_SRC_ALPHA: vk::BlendFactor                                                 = vk::BlendFactor::SRC_ALPHA;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: vk::BlendFactor                                       = vk::BlendFactor::ONE_MINUS_SRC_ALPHA;
pub const VK_BLEND_FACTOR_DST_ALPHA: vk::BlendFactor                                                 = vk::BlendFactor::DST_ALPHA;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: vk::BlendFactor                                       = vk::BlendFactor::ONE_MINUS_DST_ALPHA;
pub const VK_BLEND_FACTOR_CONSTANT_COLOR: vk::BlendFactor                                            = vk::BlendFactor::CONSTANT_COLOR;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: vk::BlendFactor                                  = vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR;
pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: vk::BlendFactor                                            = vk::BlendFactor::CONSTANT_ALPHA;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: vk::BlendFactor                                  = vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA;
pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: vk::BlendFactor                                        = vk::BlendFactor::SRC_ALPHA_SATURATE;
pub const VK_BLEND_FACTOR_SRC1_COLOR: vk::BlendFactor                                                = vk::BlendFactor::SRC1_COLOR;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: vk::BlendFactor                                      = vk::BlendFactor::ONE_MINUS_SRC1_COLOR;
pub const VK_BLEND_FACTOR_SRC1_ALPHA: vk::BlendFactor                                                = vk::BlendFactor::SRC1_ALPHA;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: vk::BlendFactor                                      = vk::BlendFactor::ONE_MINUS_SRC1_ALPHA;
// } VkBlendFactor;

// ColorComponentFlags-VkColorComponentFlagBits:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html
// Provided by VK_VERSION_1_0
// enum VkColorComponentFlagBits {
pub const VK_COLOR_COMPONENT_R_BIT: vk::ColorComponentFlags                                          = vk::ColorComponentFlags::R;
pub const VK_COLOR_COMPONENT_G_BIT: vk::ColorComponentFlags                                          = vk::ColorComponentFlags::G;
pub const VK_COLOR_COMPONENT_B_BIT: vk::ColorComponentFlags                                          = vk::ColorComponentFlags::B;
pub const VK_COLOR_COMPONENT_A_BIT: vk::ColorComponentFlags                                          = vk::ColorComponentFlags::A;
// } VkColorComponentFlagBits;

// LogicOp-VkLogicOp:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkLogicOp.html
// Provided by VK_VERSION_1_0
// enum VkLogicOp {
pub const VK_LOGIC_OP_CLEAR: vk::LogicOp                                                             = vk::LogicOp::CLEAR;
pub const VK_LOGIC_OP_AND: vk::LogicOp                                                               = vk::LogicOp::AND;
pub const VK_LOGIC_OP_AND_REVERSE: vk::LogicOp                                                       = vk::LogicOp::AND_REVERSE;
pub const VK_LOGIC_OP_COPY: vk::LogicOp                                                              = vk::LogicOp::COPY;
pub const VK_LOGIC_OP_AND_INVERTED: vk::LogicOp                                                      = vk::LogicOp::AND_INVERTED;
pub const VK_LOGIC_OP_NO_OP: vk::LogicOp                                                             = vk::LogicOp::NO_OP;
pub const VK_LOGIC_OP_XOR: vk::LogicOp                                                               = vk::LogicOp::XOR;
pub const VK_LOGIC_OP_OR: vk::LogicOp                                                                = vk::LogicOp::OR;
pub const VK_LOGIC_OP_NOR: vk::LogicOp                                                               = vk::LogicOp::NOR;
pub const VK_LOGIC_OP_EQUIVALENT: vk::LogicOp                                                        = vk::LogicOp::EQUIVALENT;
pub const VK_LOGIC_OP_INVERT: vk::LogicOp                                                            = vk::LogicOp::INVERT;
pub const VK_LOGIC_OP_OR_REVERSE: vk::LogicOp                                                        = vk::LogicOp::OR_REVERSE;
pub const VK_LOGIC_OP_COPY_INVERTED: vk::LogicOp                                                     = vk::LogicOp::COPY_INVERTED;
pub const VK_LOGIC_OP_OR_INVERTED: vk::LogicOp                                                       = vk::LogicOp::OR_INVERTED;
pub const VK_LOGIC_OP_NAND: vk::LogicOp                                                              = vk::LogicOp::NAND;
pub const VK_LOGIC_OP_SET: vk::LogicOp                                                               = vk::LogicOp::SET;
// } VkLogicOp;

// DynamicState-VkDynamicState:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDynamicState.html
// Provided by VK_VERSION_1_0
// enum VkDynamicState {
pub const VK_DYNAMIC_STATE_VIEWPORT: vk::DynamicState                                                = vk::DynamicState::VIEWPORT;
pub const VK_DYNAMIC_STATE_SCISSOR: vk::DynamicState                                                 = vk::DynamicState::SCISSOR;
pub const VK_DYNAMIC_STATE_LINE_WIDTH: vk::DynamicState                                              = vk::DynamicState::LINE_WIDTH;
pub const VK_DYNAMIC_STATE_DEPTH_BIAS: vk::DynamicState                                              = vk::DynamicState::DEPTH_BIAS;
pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: vk::DynamicState                                         = vk::DynamicState::BLEND_CONSTANTS;
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: vk::DynamicState                                            = vk::DynamicState::DEPTH_BOUNDS;
pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: vk::DynamicState                                    = vk::DynamicState::STENCIL_COMPARE_MASK;
pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: vk::DynamicState                                      = vk::DynamicState::STENCIL_WRITE_MASK;
pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: vk::DynamicState                                       = vk::DynamicState::STENCIL_REFERENCE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_CULL_MODE: vk::DynamicState                                               = vk::DynamicState::CULL_MODE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_FRONT_FACE: vk::DynamicState                                              = vk::DynamicState::FRONT_FACE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY: vk::DynamicState                                      = vk::DynamicState::PRIMITIVE_TOPOLOGY;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT: vk::DynamicState                                     = vk::DynamicState::VIEWPORT_WITH_COUNT;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT: vk::DynamicState                                      = vk::DynamicState::SCISSOR_WITH_COUNT;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE: vk::DynamicState                             = vk::DynamicState::VERTEX_INPUT_BINDING_STRIDE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE: vk::DynamicState                                       = vk::DynamicState::DEPTH_TEST_ENABLE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE: vk::DynamicState                                      = vk::DynamicState::DEPTH_WRITE_ENABLE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP: vk::DynamicState                                        = vk::DynamicState::DEPTH_COMPARE_OP;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE: vk::DynamicState                                = vk::DynamicState::DEPTH_BOUNDS_TEST_ENABLE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE: vk::DynamicState                                     = vk::DynamicState::STENCIL_TEST_ENABLE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_STENCIL_OP: vk::DynamicState                                              = vk::DynamicState::STENCIL_OP;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE: vk::DynamicState                               = vk::DynamicState::RASTERIZER_DISCARD_ENABLE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE: vk::DynamicState                                       = vk::DynamicState::DEPTH_BIAS_ENABLE;
// Provided by VK_VERSION_1_3
pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE: vk::DynamicState                                = vk::DynamicState::PRIMITIVE_RESTART_ENABLE;
// Provided by VK_NV_clip_space_w_scaling
pub const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV: vk::DynamicState                                   = vk::DynamicState::VIEWPORT_W_SCALING_NV;
// Provided by VK_EXT_discard_rectangles
pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT: vk::DynamicState                                   = vk::DynamicState::DISCARD_RECTANGLE_EXT;
// Provided by VK_EXT_discard_rectangles
//? VK_DYNAMIC_STATE_DISCARD_RECTANGLE_ENABLE_EXT = 1000099001
// Provided by VK_EXT_discard_rectangles
//? VK_DYNAMIC_STATE_DISCARD_RECTANGLE_MODE_EXT = 1000099002
// Provided by VK_EXT_sample_locations
pub const VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT: vk::DynamicState                                    = vk::DynamicState::SAMPLE_LOCATIONS_EXT;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR: vk::DynamicState                     = vk::DynamicState::RAY_TRACING_PIPELINE_STACK_SIZE_KHR;
// Provided by VK_NV_shading_rate_image
pub const VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV: vk::DynamicState                        = vk::DynamicState::VIEWPORT_SHADING_RATE_PALETTE_NV;
// Provided by VK_NV_shading_rate_image
pub const VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV: vk::DynamicState                         = vk::DynamicState::VIEWPORT_COARSE_SAMPLE_ORDER_NV;
// Provided by VK_NV_scissor_exclusive
//? VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_ENABLE_NV = 1000205000
// Provided by VK_NV_scissor_exclusive
pub const VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV: vk::DynamicState                                    = vk::DynamicState::EXCLUSIVE_SCISSOR_NV;
// Provided by VK_KHR_fragment_shading_rate
pub const VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR: vk::DynamicState                               = vk::DynamicState::FRAGMENT_SHADING_RATE_KHR;
// Provided by VK_EXT_line_rasterization
pub const VK_DYNAMIC_STATE_LINE_STIPPLE_EXT: vk::DynamicState                                        = vk::DynamicState::LINE_STIPPLE_EXT;
// Provided by VK_EXT_vertex_input_dynamic_state
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_EXT: vk::DynamicState                                        = vk::DynamicState::VERTEX_INPUT_EXT;
// Provided by VK_EXT_extended_dynamic_state2
pub const VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT: vk::DynamicState                                = vk::DynamicState::PATCH_CONTROL_POINTS_EXT;
// Provided by VK_EXT_extended_dynamic_state2
pub const VK_DYNAMIC_STATE_LOGIC_OP_EXT: vk::DynamicState                                            = vk::DynamicState::LOGIC_OP_EXT;
// Provided by VK_EXT_color_write_enable
pub const VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT: vk::DynamicState                                  = vk::DynamicState::COLOR_WRITE_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_TESSELLATION_DOMAIN_ORIGIN_EXT: vk::DynamicState                          = vk::DynamicState::TESSELLATION_DOMAIN_ORIGIN_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_DEPTH_CLAMP_ENABLE_EXT: vk::DynamicState                                  = vk::DynamicState::DEPTH_CLAMP_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_POLYGON_MODE_EXT: vk::DynamicState                                        = vk::DynamicState::POLYGON_MODE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_RASTERIZATION_SAMPLES_EXT: vk::DynamicState                               = vk::DynamicState::RASTERIZATION_SAMPLES_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_SAMPLE_MASK_EXT: vk::DynamicState                                         = vk::DynamicState::SAMPLE_MASK_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_ALPHA_TO_COVERAGE_ENABLE_EXT: vk::DynamicState                            = vk::DynamicState::ALPHA_TO_COVERAGE_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_ALPHA_TO_ONE_ENABLE_EXT: vk::DynamicState                                 = vk::DynamicState::ALPHA_TO_ONE_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_LOGIC_OP_ENABLE_EXT: vk::DynamicState                                     = vk::DynamicState::LOGIC_OP_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COLOR_BLEND_ENABLE_EXT: vk::DynamicState                                  = vk::DynamicState::COLOR_BLEND_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COLOR_BLEND_EQUATION_EXT: vk::DynamicState                                = vk::DynamicState::COLOR_BLEND_EQUATION_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COLOR_WRITE_MASK_EXT: vk::DynamicState                                    = vk::DynamicState::COLOR_WRITE_MASK_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_RASTERIZATION_STREAM_EXT: vk::DynamicState                                = vk::DynamicState::RASTERIZATION_STREAM_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_CONSERVATIVE_RASTERIZATION_MODE_EXT: vk::DynamicState                     = vk::DynamicState::CONSERVATIVE_RASTERIZATION_MODE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: vk::DynamicState                 = vk::DynamicState::EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_DEPTH_CLIP_ENABLE_EXT: vk::DynamicState                                   = vk::DynamicState::DEPTH_CLIP_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_ENABLE_EXT: vk::DynamicState                             = vk::DynamicState::SAMPLE_LOCATIONS_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COLOR_BLEND_ADVANCED_EXT: vk::DynamicState                                = vk::DynamicState::COLOR_BLEND_ADVANCED_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_PROVOKING_VERTEX_MODE_EXT: vk::DynamicState                               = vk::DynamicState::PROVOKING_VERTEX_MODE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_LINE_RASTERIZATION_MODE_EXT: vk::DynamicState                             = vk::DynamicState::LINE_RASTERIZATION_MODE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_LINE_STIPPLE_ENABLE_EXT: vk::DynamicState                                 = vk::DynamicState::LINE_STIPPLE_ENABLE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: vk::DynamicState                      = vk::DynamicState::DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_ENABLE_NV: vk::DynamicState                            = vk::DynamicState::VIEWPORT_W_SCALING_ENABLE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_VIEWPORT_SWIZZLE_NV: vk::DynamicState                                     = vk::DynamicState::VIEWPORT_SWIZZLE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_ENABLE_NV: vk::DynamicState                             = vk::DynamicState::COVERAGE_TO_COLOR_ENABLE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COVERAGE_TO_COLOR_LOCATION_NV: vk::DynamicState                           = vk::DynamicState::COVERAGE_TO_COLOR_LOCATION_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COVERAGE_MODULATION_MODE_NV: vk::DynamicState                             = vk::DynamicState::COVERAGE_MODULATION_MODE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_ENABLE_NV: vk::DynamicState                     = vk::DynamicState::COVERAGE_MODULATION_TABLE_ENABLE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_NV: vk::DynamicState                            = vk::DynamicState::COVERAGE_MODULATION_TABLE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_SHADING_RATE_IMAGE_ENABLE_NV: vk::DynamicState                            = vk::DynamicState::SHADING_RATE_IMAGE_ENABLE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: vk::DynamicState                  = vk::DynamicState::REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV;
// Provided by VK_EXT_extended_dynamic_state3
pub const VK_DYNAMIC_STATE_COVERAGE_REDUCTION_MODE_NV: vk::DynamicState                              = vk::DynamicState::COVERAGE_REDUCTION_MODE_NV;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_CULL_MODE_EXT: vk::DynamicState                                           = VK_DYNAMIC_STATE_CULL_MODE;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_FRONT_FACE_EXT: vk::DynamicState                                          = VK_DYNAMIC_STATE_FRONT_FACE;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT: vk::DynamicState                                  = VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT: vk::DynamicState                                 = VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT: vk::DynamicState                                  = VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT: vk::DynamicState                         = VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT: vk::DynamicState                                   = VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT: vk::DynamicState                                  = VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT: vk::DynamicState                                    = VK_DYNAMIC_STATE_DEPTH_COMPARE_OP;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT: vk::DynamicState                            = VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT: vk::DynamicState                                 = VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE;
// Provided by VK_EXT_extended_dynamic_state
pub const VK_DYNAMIC_STATE_STENCIL_OP_EXT: vk::DynamicState                                          = VK_DYNAMIC_STATE_STENCIL_OP;
// Provided by VK_EXT_extended_dynamic_state2
pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE_EXT: vk::DynamicState                           = VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE;
// Provided by VK_EXT_extended_dynamic_state2
pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE_EXT: vk::DynamicState                                   = VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE;
// Provided by VK_EXT_extended_dynamic_state2
pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT: vk::DynamicState                            = VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE;
// } VkDynamicState;

// PrimitiveTopology-VkPrimitiveTopology:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html
// Provided by VK_VERSION_1_0
// enum VkPrimitiveTopology {
pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: vk::PrimitiveTopology                                    = vk::PrimitiveTopology::POINT_LIST;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: vk::PrimitiveTopology                                     = vk::PrimitiveTopology::LINE_LIST;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: vk::PrimitiveTopology                                    = vk::PrimitiveTopology::LINE_STRIP;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: vk::PrimitiveTopology                                 = vk::PrimitiveTopology::TRIANGLE_LIST;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: vk::PrimitiveTopology                                = vk::PrimitiveTopology::TRIANGLE_STRIP;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: vk::PrimitiveTopology                                  = vk::PrimitiveTopology::TRIANGLE_FAN;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: vk::PrimitiveTopology                      = vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: vk::PrimitiveTopology                     = vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: vk::PrimitiveTopology                  = vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: vk::PrimitiveTopology                 = vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY;
pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: vk::PrimitiveTopology                                    = vk::PrimitiveTopology::PATCH_LIST;
// } VkPrimitiveTopology;

// AttachmentLoadOp-VkAttachmentLoadOp:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentLoadOp.html
// Provided by VK_VERSION_1_0
// enum VkAttachmentLoadOp {
pub const VK_ATTACHMENT_LOAD_OP_LOAD: vk::AttachmentLoadOp                                           = vk::AttachmentLoadOp::LOAD;
pub const VK_ATTACHMENT_LOAD_OP_CLEAR: vk::AttachmentLoadOp                                          = vk::AttachmentLoadOp::CLEAR;
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: vk::AttachmentLoadOp                                      = vk::AttachmentLoadOp::DONT_CARE;
// Provided by VK_EXT_load_store_op_none
pub const VK_ATTACHMENT_LOAD_OP_NONE_EXT: vk::AttachmentLoadOp                                       = vk::AttachmentLoadOp::NONE_EXT;
// } VkAttachmentLoadOp;

// AttachmentStoreOp-VkAttachmentStoreOp:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentStoreOp.html
// Provided by VK_VERSION_1_0
// enum VkAttachmentStoreOp {
pub const VK_ATTACHMENT_STORE_OP_STORE: vk::AttachmentStoreOp                                        = vk::AttachmentStoreOp::STORE;
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: vk::AttachmentStoreOp                                    = vk::AttachmentStoreOp::DONT_CARE;
// Provided by VK_VERSION_1_3
pub const VK_ATTACHMENT_STORE_OP_NONE: vk::AttachmentStoreOp                                         = vk::AttachmentStoreOp::NONE;
// Provided by VK_KHR_dynamic_rendering
pub const VK_ATTACHMENT_STORE_OP_NONE_KHR: vk::AttachmentStoreOp                                     = VK_ATTACHMENT_STORE_OP_NONE;
// Provided by VK_QCOM_render_pass_store_ops
pub const VK_ATTACHMENT_STORE_OP_NONE_QCOM: vk::AttachmentStoreOp                                    = VK_ATTACHMENT_STORE_OP_NONE;
// Provided by VK_EXT_load_store_op_none
pub const VK_ATTACHMENT_STORE_OP_NONE_EXT: vk::AttachmentStoreOp                                     = VK_ATTACHMENT_STORE_OP_NONE;
// } VkAttachmentStoreOp;

// PipelineBindPoint-VkPipelineBindPoint:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html
// Provided by VK_VERSION_1_0
// enum VkPipelineBindPoint {
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: vk::PipelineBindPoint                                     = vk::PipelineBindPoint::GRAPHICS;
pub const VK_PIPELINE_BIND_POINT_COMPUTE: vk::PipelineBindPoint                                      = vk::PipelineBindPoint::COMPUTE;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR: vk::PipelineBindPoint                              = vk::PipelineBindPoint::RAY_TRACING_KHR;
// Provided by VK_HUAWEI_subpass_shading
pub const VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI: vk::PipelineBindPoint                       = vk::PipelineBindPoint::SUBPASS_SHADING_HUAWEI;
// Provided by VK_NV_ray_tracing
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_NV: vk::PipelineBindPoint                               = VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR;
// } VkPipelineBindPoint;

// Result-VkResult:https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkResult.html
// Provided by VK_VERSION_1_0
// enum VkResult {
pub const VK_SUCCESS: vk::Result                                                                     = vk::Result::SUCCESS;
pub const VK_NOT_READY: vk::Result                                                                   = vk::Result::NOT_READY;
pub const VK_TIMEOUT: vk::Result                                                                     = vk::Result::TIMEOUT;
pub const VK_EVENT_SET: vk::Result                                                                   = vk::Result::EVENT_SET;
pub const VK_EVENT_RESET: vk::Result                                                                 = vk::Result::EVENT_RESET;
pub const VK_INCOMPLETE: vk::Result                                                                  = vk::Result::INCOMPLETE;
pub const VK_ERROR_OUT_OF_HOST_MEMORY: vk::Result                                                    = -1;
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: vk::Result                                                  = -2;
pub const VK_ERROR_INITIALIZATION_FAILED: vk::Result                                                 = -3;
pub const VK_ERROR_DEVICE_LOST: vk::Result                                                           = -4;
pub const VK_ERROR_MEMORY_MAP_FAILED: vk::Result                                                     = -5;
pub const VK_ERROR_LAYER_NOT_PRESENT: vk::Result                                                     = -6;
pub const VK_ERROR_EXTENSION_NOT_PRESENT: vk::Result                                                 = -7;
pub const VK_ERROR_FEATURE_NOT_PRESENT: vk::Result                                                   = -8;
pub const VK_ERROR_INCOMPATIBLE_DRIVER: vk::Result                                                   = -9;
pub const VK_ERROR_TOO_MANY_OBJECTS: vk::Result                                                      = -10;
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: vk::Result                                                  = -11;
pub const VK_ERROR_FRAGMENTED_POOL: vk::Result                                                       = -12;
pub const VK_ERROR_UNKNOWN: vk::Result                                                               = -13;
// Provided by VK_VERSION_1_1
pub const VK_ERROR_OUT_OF_POOL_MEMORY: vk::Result                                                    = -1000069000;
// Provided by VK_VERSION_1_1
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: vk::Result                                               = -1000072003;
// Provided by VK_VERSION_1_2
pub const VK_ERROR_FRAGMENTATION: vk::Result                                                         = -1000161000;
// Provided by VK_VERSION_1_2
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: vk::Result                                        = -1000257000;
// Provided by VK_VERSION_1_3
pub const VK_PIPELINE_COMPILE_REQUIRED: vk::Result                                                   = vk::Result::PIPELINE_COMPILE_REQUIRED;
// Provided by VK_KHR_surface
pub const VK_ERROR_SURFACE_LOST_KHR: vk::Result                                                      = -1000000000;
// Provided by VK_KHR_surface
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: vk::Result                                              = -1000000001;
// Provided by VK_KHR_swapchain
pub const VK_SUBOPTIMAL_KHR: vk::Result                                                              = vk::Result::SUBOPTIMAL_KHR;
// Provided by VK_KHR_swapchain
pub const VK_ERROR_OUT_OF_DATE_KHR: vk::Result                                                       = -1000001004;
// Provided by VK_KHR_display_swapchain
pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: vk::Result                                              = -1000003001;
// Provided by VK_EXT_debug_report
pub const VK_ERROR_VALIDATION_FAILED_EXT: vk::Result                                                 = -1000011001;
// Provided by VK_NV_glsl_shader
pub const VK_ERROR_INVALID_SHADER_NV: vk::Result                                                     = -1000012000;
// Provided by VK_KHR_video_queue
pub const VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: vk::Result                                         = -1000023000;
// Provided by VK_KHR_video_queue
pub const VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: vk::Result                                = -1000023001;
// Provided by VK_KHR_video_queue
pub const VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: vk::Result                             = -1000023002;
// Provided by VK_KHR_video_queue
pub const VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: vk::Result                                = -1000023003;
// Provided by VK_KHR_video_queue
pub const VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: vk::Result                                 = -1000023004;
// Provided by VK_KHR_video_queue
pub const VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: vk::Result                                   = -1000023005;
// Provided by VK_EXT_image_drm_format_modifier
pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: vk::Result                          = -1000158000;
// Provided by VK_KHR_global_priority
pub const VK_ERROR_NOT_PERMITTED_KHR: vk::Result                                                     = -1000174001;
// Provided by VK_EXT_full_screen_exclusive
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: vk::Result                                   = -1000255000;
// Provided by VK_KHR_deferred_host_operations
pub const VK_THREAD_IDLE_KHR: vk::Result                                                             = vk::Result::THREAD_IDLE_KHR;
// Provided by VK_KHR_deferred_host_operations
pub const VK_THREAD_DONE_KHR: vk::Result                                                             = vk::Result::THREAD_DONE_KHR;
// Provided by VK_KHR_deferred_host_operations
pub const VK_OPERATION_DEFERRED_KHR: vk::Result                                                      = vk::Result::OPERATION_DEFERRED_KHR;
// Provided by VK_KHR_deferred_host_operations
pub const VK_OPERATION_NOT_DEFERRED_KHR: vk::Result                                                  = vk::Result::OPERATION_NOT_DEFERRED_KHR;
// #ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
pub const VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: vk::Result                                      = -1000299000;
// #endif
// Provided by VK_EXT_image_compression_control
pub const VK_ERROR_COMPRESSION_EXHAUSTED_EXT: vk::Result                                             = -1000338000;
// Provided by VK_EXT_shader_object
//? VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT = 1000482000
// Provided by VK_KHR_maintenance1
pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: vk::Result                                                = VK_ERROR_OUT_OF_POOL_MEMORY;
// Provided by VK_KHR_external_memory
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: vk::Result                                           = VK_ERROR_INVALID_EXTERNAL_HANDLE;
// Provided by VK_EXT_descriptor_indexing
pub const VK_ERROR_FRAGMENTATION_EXT: vk::Result                                                     = VK_ERROR_FRAGMENTATION;
// Provided by VK_EXT_global_priority
pub const VK_ERROR_NOT_PERMITTED_EXT: vk::Result                                                     = VK_ERROR_NOT_PERMITTED_KHR;
// Provided by VK_EXT_buffer_device_address
pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT: vk::Result                                            = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
// Provided by VK_KHR_buffer_device_address
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: vk::Result                                    = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
// Provided by VK_EXT_pipeline_creation_cache_control
pub const VK_PIPELINE_COMPILE_REQUIRED_EXT: vk::Result                                               = VK_PIPELINE_COMPILE_REQUIRED;
// Provided by VK_EXT_pipeline_creation_cache_control
pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT: vk::Result                                         = VK_PIPELINE_COMPILE_REQUIRED;
// } VkResult;

