use std::{fs, io::Read};
use flate2::read::GzDecoder;
use serde;
use serde_json;

use crate::coordinates;

#[derive(serde::Deserialize, Debug)]
#[serde(transparent)]
pub struct Heightmap {
    data: Vec<Vec<i32>>,
}

pub fn load_heightmap(
    coordinates: coordinates::TileCoordinates
) -> Result<Heightmap, Box<dyn std::error::Error>> {
    let bytes = fs::read(format!(
        "assets/heightmaps/{}x{}",
        coordinates.longitude,
        coordinates.latitude
    ))?;

    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decoded_data = String::new();
    decoder.read_to_string(&mut decoded_data)?;

    let json: Vec<Vec<i32>> = serde_json::from_str(&decoded_data)?;

    // TODO Validate the heightmap dimensions before returning it.
    Ok(Heightmap { data: json })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heightmap_is_loaded() {
        let directory_iterator = fs::read_dir("assets/heightmaps");
        assert!(directory_iterator.is_ok());

        let file_name = String::from(
            directory_iterator.unwrap().next().unwrap().unwrap().file_name().to_str().unwrap()
        );
        let mut split_file_name = file_name.split("x");
        let coordinates = coordinates::TileCoordinates {
            longitude: split_file_name.next().unwrap().parse::<i64>().unwrap(),
            latitude: split_file_name.next().unwrap().parse::<i64>().unwrap(),
        };

        let heightmap = load_heightmap(coordinates);

        assert!(heightmap.is_ok());
        assert!((heightmap.unwrap().data.len() as u32) == coordinates::TILE_SIZE / 2);
    }
}
