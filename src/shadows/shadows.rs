
use crate::{shadows};
use image::{RgbaImage, Rgba};

#[derive(serde::Serialize, Debug)]
#[serde(transparent)]
pub struct ShadowMap {
    data: Vec<u8>,
}

pub fn compute_shadows(heightmap: shadows::Heightmap) -> RgbaImage {
    let mut shadows: RgbaImage = RgbaImage::new(
        heightmap.data.len() as u32,
        heightmap.data.len() as u32
    );

    for m in 0..heightmap.data.len() {
        for n in 0..heightmap.data[m].len() {
            let alpha_channel = if heightmap.data[m][n] > 50 { 120 } else { 0 };
            shadows.put_pixel(n as u32, m as u32, Rgba([0, 0, 0, alpha_channel]));
        }
    }

    shadows
}
