use actix_web::{get, web, Responder, Result, error};

use crate::coordinates;

#[get("/api/tile_coordinates/{longitude}/{latitude}")]
pub async fn get_tile_information(path: web::Path<(f64, f64)>) -> Result<impl Responder> {
    let (longitude, latitude) = path.into_inner();
    let original_coordinates = coordinates::Coordinates { longitude, latitude };

    if !coordinates::wgs84_coordinates_are_valid(&original_coordinates) {
        return Err(error::ErrorBadRequest("Invalid coordinates"));
    }

    match coordinates::wgs84_coordinates_to_tile(&original_coordinates) {
        Ok(tile_information) => Ok(web::Json(tile_information)),
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}
