#![feature(str_split_as_str)]
#![feature(iterator_try_collect)]

mod mobj;
mod define;
mod matrix;

use mobj::Module;
use define::*;
use matrix::*;

use mem::swap;
use std::error::Error;
use std::{env, fs, mem};
use std::f64::consts::PI;
use std::iter::zip;
use std::time::SystemTime;

use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive, Rgb, Rgba, RgbaImage};
use image::codecs::hdr::Rgbe8Pixel;
use once_cell::sync::Lazy;

fn p3d(v: &V4d) -> V3d {
    V3t {
        x: (v.x+1.) * (WID-1) as f64/2.,
        y: (v.y+1.) * (HIG-1) as f64/2.,
        z: (v.z+1.) * (DEP-1) as f64/2.
    }
}

// fn p3i(v: &V4d) -> V3i {
//     V3i {
//         x: ((v[0] + 1.) * (WID - 1) as f64 / 2.) as i32,
//         y: ((v[1] + 1.) * (HIG - 1) as f64 / 2.) as i32,
//         z: ((v[2] + 1.) * (DEP - 1) as f64 / 2.) as i32
//     }
// }

fn p3i(v: &V4d) -> V3i {
    V3t {
        x: ((v.x + 1.) * (WID - 1) as f64 / 2.) as i32,
        y: ((v.y + 1.) * (HIG - 1) as f64 / 2.) as i32,
        z: ((v.z + 1.) * (DEP - 1) as f64 / 2.) as i32
    }
}

fn draw_line<T: Pixel, U: GenericImage + GenericImageView<Pixel = T>>(p0: V3i, p1: V3i, img: &mut U, color: T) {
    let xlen = (p1.x-p0.x).abs();
    let ylen = (p1.y-p0.y).abs();
    let flag = (xlen < ylen) as i32;
    let len = xlen * (flag^1) + ylen * flag;

    for i in 0..len {
        img.put_pixel((p0.x+(p1.x-p0.x)*i/len) as u32, (p0.y+(p1.y-p0.y)*i/len) as u32, color);
    }
}

fn triangle(v012: [V3t<i32>; 3], vt012: [V3t<i32>; 3], zbuf: &mut [u8], img: &mut RgbaImage, ttv: &RgbaImage, rgba: Rgba<u8>) {
    let [mut v0, mut v1, mut v2] = v012;
    let [mut vt0, mut vt1, mut vt2] = vt012;

    if v1.x > v0.x { swap(&mut v0, &mut v1); swap(&mut vt0, &mut vt1); }
    if v2.x > v0.x { swap(&mut v2, &mut v0); swap(&mut vt2, &mut vt0); }
    if v2.x > v1.x { swap(&mut v2, &mut v1); swap(&mut vt2, &mut vt1); }
    let x02l = v0.x - v2.x;
    let x01l = v0.x - v1.x;
    let x12l = v1.x - v2.x;

    let z = ((v0.z + v1.z + v2.z) / 3) as u8;
    let a = 0;
    for i in 0..x02l {

        let x02 = (v2.x - v0.x) * i / x02l + v0.x;
        let y11 = if i < x01l {v0.y+(v1.y-v0.y)*i/x01l} else {v1.y+(v2.y-v1.y)*(i-x01l)/x12l};
        let ylen = (v2.y - v0.y) * i / x02l + v0.y - y11;
        let yneg = (ylen > 0) as i32 - (ylen < 0) as i32;

        let vtx02 = (vt2.x - vt0.x) * i / x02l + vt0.x;
        let vty11 = if i < x01l {vt0.y+(vt1.y-vt0.y)*i/x01l} else {vt1.y+(vt2.y-vt1.y)*(i-x01l)/x12l};
        let vtylen = (vt2.y - vt0.y) * i / x02l + vt0.y - vty11;
        let vtyneg = (vtylen > 0) as i32 - (vtylen < 0) as i32;

        let y11l = ylen as i32 * yneg + 1;
        let vty11l = vtylen as i32 * vtyneg + 1;
        (0..y11l).for_each(|y|{
            let (vx, vy) = (x02 as u32, (y11+y*yneg).unsigned_abs());
            if vx < WID && vy< HIG && zbuf[(vx *HIG+ vy) as usize] < z {
                img.put_pixel(vx, vy, *ttv.get_pixel(vtx02 as u32, (vty11+(y*vty11l*vtyneg/y11l)).unsigned_abs()));
                // img.put_pixel(vx, vy, rgba);
                zbuf[(vx *HIG+ vy) as usize] = z;
            }
            // else if vx < WID && vy< HIG {
            //     img2.put_pixel(vx, vy, *ttv.get_pixel(vtx02 as u32, (vty11+(y*vty11l*vtyneg/y11l)).unsigned_abs()));
            //     // img2.put_pixel(vx, vy, rgba);
            // }
        })
    }
}

trait PixOps {
    fn revs(&mut self);
}

impl PixOps for RgbaImage {
    fn revs(&mut self) {
        let w = self.width();
        let h = self.height();
        for x0 in 0..w/2 {
            for y0 in 0..h {
                let x1 = w - 1 - x0;
                let y1 = h - 1 - y0;
                let temp = *self.get_pixel(x0, y0);
                self.put_pixel(x0, y0, *self.get_pixel(x1, y1));
                self.put_pixel(x1, y1, temp);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{

    let mut a = vec![1,2,3];
    let b = vec![1,2,3];
    println!("{b:?}");
    println!("{a:?}, {:?}", a.as_ptr());
    a.clear();
    a.resize(2, 1);
    println!("{a:?}, {:?}", a.as_ptr());
    let c = CWD.clone();

    let md = Module::new(CWD.clone() + "/res/obj/african_head/african_head.obj").unwrap();
    let mut ttv = image::open(CWD.clone() + "/res/obj/african_head/african_head_diffuse.png").unwrap().to_rgba8();
    let mut img = image::RgbaImage::new(WID, HIG);
    let mut zbuf: Vec<u8> = vec![0; (HIG * WID) as usize];
    let (ttvx, ttvy) = (ttv.width() as f64, ttv.height() as f64);
    ttv.revs();

    let start = SystemTime::now();

    let mut m = vec![
        // Mx4d::proj(5.1),
        Mx4d::rot(XAXIS, PI*0.54),
        // Mx4d::rot(XAXIS, PI/4.),
        Mx4d::rot(YAXIS, PI*0.12),
        // Mx4d::rot(ZAXIS, PI/4.),
        // Mx4d::scale(0.6, 0.6 ,0.6),
        // Mx4d::trans(0., 0., 0.),
    ];
    let mut t = Mx4d::rot(XAXIS, PI);
    m.iter().for_each(|x| t = t * *x);
    // let t = Mx4d::identity();

    let light_dir = V3d {x: 0., y: 0., z: 1.};
    let transV = md.v.iter().map(|v| t * v).collect::<Vec<V4d>>();

    for f in md.f {
        // let (v0, v1, v2) = (md.v[f[0]-1], md.v[f[3]-1], md.v[f[6]-1]);
        let (v0, v1, v2) = (transV[f[0]-1], transV[f[3]-1], transV[f[6]-1]);
        let vt012 = [&md.vt[f[1]-1], &md.vt[f[4]-1], &md.vt[f[7]-1]].map(|x| V3i{x: (x.x*ttvx) as i32, y: (x.y*ttvy) as i32, z: 0});
        let intensity = light_dir.dot((v0 - v2).v3t().cross((v1 - v2).v3t()).normalize().scale(-1.));
        //
        // let a = (V3t {x:1.,y:2.,z:3.}).normalize();
        //
        let l = (intensity * 255f64) as u8;
        // let l = 9u8;
        if intensity > 0f64 {
            // triangle([p3i(&(t *v0)), p3i(&(t *v1)), p3i(&(t *v2))], vt012, &mut zbuf, &mut img, &ttv, Rgba([l,l,l,255]));
            triangle([p3i(&v0), p3i(&v1), p3i(&v2)], vt012, &mut zbuf, &mut img, &ttv, Rgba([l,l,l,255]));
        }
        // else {
        //     triangle([p3i(&v0), p3i(&v1), p3i(&v2)], vt012, &mut zbuf, &mut img2, &ttv, Rgba([l,l,l,255]));
        // }
    }
    let diff = SystemTime::now().duration_since(start).unwrap();
    println!("{:?}", diff.as_nanos());

    // fs::write(CWD.clone() + "/res/img/a.txt", format!("{:?}", zbuf)).unwrap();
    img.save(CWD.clone() + "/res/img/a.png").unwrap();
    // img2.revs();
    // img2.save(CWD.clone() + "/res/img/a2.png").unwrap();
    Ok(())
}