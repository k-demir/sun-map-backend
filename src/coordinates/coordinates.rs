use proj::{Proj, ProjError};

pub const TILE_SIZE: u32 = 400;

#[derive(Debug)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, PartialEq)]
pub struct TileCoordinates {
    pub longitude: i64,
    pub latitude: i64,
}

// Transforms WGS84 coordinates to TM35FIN coordinates.
pub fn transform(coordinates: &Coordinates) -> Result<Coordinates, ProjError> {
    const FROM: &'static str = "EPSG:4326";
    const TO: &'static str = "EPSG:3067";
    let transformation = Proj::new_known_crs(&FROM, &TO, None).unwrap();

    match transformation.convert((coordinates.longitude, coordinates.latitude)) {
        Ok((a, b)) => Ok(Coordinates { longitude: a, latitude: b }),
        Err(e) => Err(e),
    }
}

// Retrieves the coordinates of the tile that contains the input coordinates. Each map tile has its
// start and end coordinates as a multiple of `TILE_SIZE`.
pub fn get_tile_coordinates(coordinates: &Coordinates) -> TileCoordinates {
    let tile_size = i64::from(TILE_SIZE);
    let find_coordinate =
        |coordinate: f64| (round(coordinate) + tile_size / 2) / tile_size * tile_size;

    TileCoordinates {
        longitude: find_coordinate(coordinates.longitude),
        latitude: find_coordinate(coordinates.latitude)
    }
}

// Rounds a coordinate by flooring it because the values increase towards the north and the east: a
// point (x, 399.9) would be in a tile with latitude in range [0, 400) rather than [400, 800).
pub fn round(coordinate: f64) -> i64 {
    coordinate.floor() as i64
}

// The TM35FIN coordinates are defined only for a subsection of the WGS84 coordinates. This
// function ensures that the WGS84 coordinates are within those bounds.
pub fn wgs84_coordinates_are_valid(coordinates: &Coordinates) -> bool {
    -16.1 <= coordinates.longitude && coordinates.longitude <= 32.88 &&
    40.18 <= coordinates.latitude && coordinates.latitude <= 84.73
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_coordinate_transforms_work() {
        let original_coordinates = Coordinates { longitude: 22.268274, latitude: 60.449822 };
        // These values should be roughly correct.
        let correct_result = Coordinates { longitude: 239818.43710216, latitude: 6710862.4516306 };

        let coordinates = transform(&original_coordinates).unwrap();

        assert!((coordinates.latitude - correct_result.latitude).abs() < 0.001);
        assert!((coordinates.longitude - correct_result.longitude).abs() < 0.001);
    }

    #[test]
    fn invalid_coordinates_result_in_error() {
        let invalid_coordinates = Coordinates { longitude: 12000., latitude: 42.0 };
        let coordinates = transform(&invalid_coordinates);
        assert!(coordinates.is_err());
    }

    #[test]
    fn tile_coordinates_are_correctly_found() {
        let coordinates = Coordinates { longitude: 247_550.1, latitude: 6_719_400.0 };
        let tile_coordinates = get_tile_coordinates(&coordinates);
        assert_eq!(tile_coordinates, TileCoordinates { longitude: 247_600, latitude: 6_719_600 })
    }
}
