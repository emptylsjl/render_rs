use ash::vk;
use glam::{Mat4, Vec3, vec3, Vec4};
use memoffset::offset_of;
use crate::define::*;


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
            // pt, cd
        }
    }

    pub fn binding_description() -> [vk::VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription::default()
                .stride(std::mem::size_of::<Vertex>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ]
    }
}


// impl From<[f32; 4]> for Vertex {
//     fn from(pt: [f32; 4]) -> Self {
//         Self {
//             pt: Vec4::from_array(pt),
//             cd: Vec4::default()
//         }
//     }
// }


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

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(offset_of!(Vertex, pt) as u32),
            vk::VertexInputAttributeDescription::default()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(offset_of!(Vertex, cd) as u32)
        ]
    }

    pub fn mem_size(&self) -> usize {
        self.pts.len() * std::mem::size_of::<Vertex>()
    }
}


pub fn camera([x, y, w, h]: [f32; 4]) -> Mat4 {
    let [x, y] = [(x-w/2.)/w, (y-h/2.)/h];

    let mut proj = Mat4::perspective_rh_gl(0.45 * 1.745329, W as f32 / H as f32, 0.1, 10.0);
    proj.y_axis[1] *= -1.0;
    [
        proj,
        Mat4::look_at_rh(vec3(2.0, 2.0, 2.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)),
        Mat4::from_rotation_z(x*10.0),
        // Mat4::from_rotation_x(x10),
        // Mat4::from_rotation_y(y10),
        // Mat4::from_scale(vec3(x, x, 1.0)),
        Mat4::from_translation(vec3(x, y, 1.0)),
    ].iter().product()
}