# Sun Map

This app uses map images and height data provided by [the National Land Survey of Finland (NLS)](https://www.maanmittauslaitos.fi/) under CC BY 4.0 license.

## Installation

1. Download 1:10000 background map images from the [file download service](https://www.maanmittauslaitos.fi/en/e-services/open-data-file-download-service) of the NLS, and place the images to `./resources/raw_map_images`.

2. Download corresponding 2m elevation model tiles from the [file download service](https://www.maanmittauslaitos.fi/en/e-services/open-data-file-download-service) of the NLS, and place the files into the `./resources/raw_heightmaps` directory.

3. Run the `./scripts/process_images.py` script to break the downloaded images and heightmaps into smaller tiles.
