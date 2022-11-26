use proj::ProjError;
use serde::Serialize;

use crate::coordinates;

#[derive(Serialize, Debug)]
pub struct TileInformation {
    longitude: i64,
    latitude: i64,
    tile_height: u32,
    tile_width: u32,
    x: i64, // Distance of the current point from the left side of the tile.
    y: i64, // Distance of the current point from the bottom side of the tile.
}

pub fn wgs84_coordinates_to_tile(
    coordinates: &coordinates::Coordinates
) -> Result<TileInformation, ProjError> {
    let transformed_coordinates = coordinates::transform(&coordinates)?;
    let tile_coordinates = coordinates::get_tile_coordinates(&transformed_coordinates);

    Ok(TileInformation {
        longitude: tile_coordinates.longitude,
        latitude: tile_coordinates.latitude,
        tile_height: coordinates::TILE_SIZE,
        tile_width: coordinates::TILE_SIZE,
        x: coordinates::round(transformed_coordinates.longitude) - tile_coordinates.longitude,
        y: coordinates::round(transformed_coordinates.latitude) - tile_coordinates.latitude,
    })
}
