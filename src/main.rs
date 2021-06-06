#![feature(min_const_generics)]
mod terrain;
use crate::terrain::terrain::{AutoGenConfig, Landmass};

fn main() {
    // Create the new landmass and tell it to autogen the world
    let mut land = Landmass::<140, 70>::new();
    land.render = true;

    let config = AutoGenConfig {
        landmass_frequency: 4.0,
        precip_frequency: 2.0,
        precip_offset: 0.0,
        temperature_frequency: 2.0,
        temperature_offset: 0.0,
        ocean_height: -1.0,
        river_height_limit: 0.25,
        river_tile_prob: 0.02,
        river_tile_limit: 500,
        seed: None,
    };

    land.autogen(config);

    // Print the landmass to terminal using ncurses
    //land.termion_print();
}
