import os
import shutil
import json
import gzip
import numpy as np
from typing import Tuple
from PIL import Image


def get_pgw_coordinates(file_path: str) -> Tuple[int, int]:
    """Returns the upper left corner coordinates of a .pgw metadata file.

    The .pgw file is contains data on 6 lines:
        0: pixel size in x-direction
        1: rotation about y-axis
        2: rotation about x-axis
        3: pixel size in y-direction (negative)
        4: x-coordinate of the centre of the upper left pixel
        5: y-coordinate of the centre of the upper left pixel
    """
    with open(file_path, "r") as metadata_file:
        lines = metadata_file.readlines()
        numeric_metadata = [int(float(line.strip())) for line in lines]
        x_coordinate = numeric_metadata[4] - numeric_metadata[0] // 2
        y_coordinate = numeric_metadata[5] - numeric_metadata[3] // 2
        return (x_coordinate, y_coordinate)


def crop_image(file_path: str, x_coordinate: int, y_coordinate: int,
               destination_directory: str) -> None:
    """Crops the image found at the given file path into multiple smaller
    images. The coordinates given as arguments correspond to the upper left
    corner. The new filenames contain the information about the location of
    the image tile.
    """
    IMG_SIZE = 6000
    TILE_SIZE = 200 # `IMG_SIZE % TILE_SIZE` should be `0`.

    with Image.open(file_path) as image:
        if image.size[0] != IMG_SIZE or image.size[1] != IMG_SIZE:
            raise ValueError('Image is not of the expected size.')

        for i in range(0, IMG_SIZE, TILE_SIZE):
            for j in range(0, IMG_SIZE, TILE_SIZE):
                cropped_image = image.crop(
                    (i, j, i + TILE_SIZE, j + TILE_SIZE)
                )
                cropped_image.save(os.path.join(
                    destination_directory,
                    f'{x_coordinate + 2 * i}x{y_coordinate - 2 * j}.png'
                ))


def create_cropped_images(source: str, destination: str) -> None:
    """Creates smaller crops of the png images found in the source directory.
    """
    # Find all .png images and drop the extension from the file names.
    image_names = [f[:-4] for f in os.listdir(source) if f[-4:] == '.png']
    for image_name in image_names:
        x_coordinate, y_coordinate = get_pgw_coordinates(
            os.path.join(source, f'{image_name}.pgw')
        )
        crop_image(
            os.path.join(source, f'{image_name}.png'), x_coordinate,
            y_coordinate, destination
        )


def crop_heightmap(file_path: str, destination: str) -> None:
    """Crops the heightmap into tiles that correspond to the ones created for
    the png images.
    """
    IMG_SIZE = 3000
    TILE_SIZE = 200 # `IMG_SIZE % TILE_SIZE` should be `0`.

    with Image.open(file_path) as image:
        if image.size[0] != IMG_SIZE or image.size[1] != IMG_SIZE:
            raise ValueError('Heightmap is not of the expected size.')

        # The metadata of the GeoTIFF files contain the upper left corner
        # coordinates at indices 3 and 4 of the `33922` tag.
        x, y = [int(value) for value in image.tag.get(33922)[3:5]]
        heightmap_array = np.round(np.array(image)).astype(np.int8)

        for i in range(0, IMG_SIZE, TILE_SIZE):
            for j in range(0, IMG_SIZE, TILE_SIZE):
                cropped_array = heightmap_array[j:j+TILE_SIZE, i:i+TILE_SIZE]
                json_data = json.dumps(cropped_array.tolist(), indent=2)
                encoded_json = json_data.encode('utf-8')
                with open(os.path.join(destination, f'{x + 2*i}x{y - 2*j}'),
                          'wb') as tile_file:
                    tile_file.write(gzip.compress(encoded_json))


def create_cropped_heightmaps(source: str, destination: str) -> None:
    """Creates smaller crops of the heightmaps found in the source directory.
    """
    names = [f[:-4] for f in os.listdir(source) if f[-4:] == '.tif']
    for name in names:
        crop_heightmap(os.path.join(source, f'{name}.tif'), destination)


def crop_data(image_source: str, image_destination: str,
              heightmap_source: str, heightmap_destination: str,
              verbose=True, remove_unnecessary_files=True) -> None:
    """Crops the images and the heightmaps and places the files into the
    destination directories. The `remove_unnecessary_files` parameter removes
    tiles that are not included for both map images and heightmaps.
    """
    if verbose:
        print('Initializing directories...')
    if os.path.exists(image_destination):
        shutil.rmtree(image_destination)
    if os.path.exists(heightmap_destination):
        shutil.rmtree(heightmap_destination)
    os.makedirs(image_destination)
    os.makedirs(heightmap_destination)

    if verbose:
        print('Cropping map tiles...')
    create_cropped_images(image_source, image_destination)
    if verbose:
        print('Cropping heightmaps...')
    create_cropped_heightmaps(heightmap_source, heightmap_destination)
    
    if remove_unnecessary_files:
        if verbose:
            print('Removing unnecessary files...')
        created_images = [f[:-4] for f in os.listdir(image_destination)]
        created_heightmaps = [f for f in os.listdir(heightmap_destination)]
        intersection = list(set(created_images) & set(created_heightmaps))

        for image_file in os.listdir(image_destination):
            if image_file[:-4] in intersection:
                continue
            os.remove(os.path.join(image_destination, image_file))

        for heightmap_file in os.listdir(heightmap_destination):
            if heightmap_file in intersection:
                continue
            os.remove(os.path.join(heightmap_destination, heightmap_file))

    if verbose:
        print('Everything is ready.')


if __name__ == '__main__':
    script_directory = os.path.dirname(__file__)

    crop_data(
        os.path.join(script_directory, '../assets/raw_map_images'),
        os.path.join(script_directory, '../assets/map_images'),
        os.path.join(script_directory, '../assets/raw_heightmaps'),
        os.path.join(script_directory, '../assets/heightmaps'),
        verbose=True,
        remove_unnecessary_files=True
    )
