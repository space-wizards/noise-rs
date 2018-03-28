//! An example of using perlin noise

extern crate noise;

use noise::{Seedable, Simplex};
use noise::utils::*;

fn main() {
    let mut simplex = Simplex::new();

    PlaneMapBuilder::new(&simplex)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("simplex.png");

    simplex = simplex.set_seed(1);

    PlaneMapBuilder::new(&simplex)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("simplex_seed=1.png");
}
