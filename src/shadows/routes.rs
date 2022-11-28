use actix_web::{get, web, Responder, Result, error};
use image;
use std::io::Cursor;
use base64;

use crate::shadows;
use crate::coordinates;

#[get("/api/shadows/{longitude}/{latitude}")]
pub async fn get_shadows(path: web::Path<(i64, i64)>) -> Result<impl Responder> {
    let (longitude, latitude) = path.into_inner();
    let coordinates = coordinates::TileCoordinates { longitude, latitude };

    let heightmap = shadows::load_heightmap(coordinates);

    if heightmap.is_err() {
        return Err(error::ErrorInternalServerError("Internal error"));
    }

    let shadow_image = shadows::compute_shadows(heightmap.unwrap());
    Ok(base64_encode_image(&shadow_image))
}

fn base64_encode_image(image: &image::RgbaImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();
    let _ = &image.write_to(&mut Cursor::new(&mut image_data), image::ImageOutputFormat::Png);
    let image_base64 = base64::encode(image_data);
    format!("data:image/png;base64,{}", image_base64)
}
