use ash::vk;
use ash::vk::{VertexInputAttributeDescription, VertexInputBindingDescription};
use glam::{Vec3, Vec4};
use memoffset::offset_of;


#[derive(Debug, Clone, Copy, Default)]
pub struct Vertex {
    pub pt: Vec4,
    pub cd: Vec4
}

impl Vertex {
    pub fn new(pt: Vec4, cd: Vec4) -> Self {
        Self {
            pt,
            cd
        }
    }

    pub fn from_arr(pt: [f32; 4], cd: [f32; 4]) -> Self {
        Self {
            pt: Vec4::from_array(pt),
            cd: Vec4::from_array(cd),
        }
    }

    pub fn binding_description() -> [VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription::default()
                .stride(std::mem::size_of::<Vertex>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ]
    }
}


impl From<[f32; 4]> for Vertex {
    fn from(pt: [f32; 4]) -> Self {
        Self {
            pt: Vec4::from_array(pt),
            cd: Vec4::default()
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct Vertices {
    pub pts: Vec<Vertex>,
}


// #[macro_export]
// macro_rules! offset_of {
//     ($base:path, $field:ident) => {{
//         #[allow(unused_unsafe)]
//         unsafe {
//             let b: $base = std::mem::zeroed();
//             std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
//         }
//     }};
// }

impl Vertices {
    pub fn new(pts: Vec<Vertex>) -> Self {
        Self {
            pts
        }
    }

    pub fn attribute_descriptions() -> [VertexInputAttributeDescription; 2] {
        [
            VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(offset_of!(Vertex, pt) as u32),
            VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(offset_of!(Vertex, cd) as u32)

        ]
    }

    pub fn mem_size(&self) -> usize {
        self.pts.len() * std::mem::size_of::<Vertex>()
    }
}