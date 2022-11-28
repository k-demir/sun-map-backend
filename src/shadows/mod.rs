mod models;
mod shadows;
mod routes;

pub use routes::get_shadows;

use models::Heightmap;
use models::load_heightmap;
use shadows::compute_shadows;
