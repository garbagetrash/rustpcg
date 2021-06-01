#![feature(min_const_generics)]
mod terrain;
use crate::terrain::terrain::{AutoGenConfig, Landmass};

fn main() {
    // Create the new landmass and tell it to autogen the world
    let mut land = Landmass::<100, 70>::new();
    land.render = true;

    let config = AutoGenConfig {
        landmass_frequency: 4.0,
        precip_frequency: 1.0,
        temperature_frequency: 1.0,
        ocean_height: -1.0,
        river_height_limit: 0.5,
        river_tile_prob: 0.05,
        river_tile_limit: 1000,
        seed: None,
    };

    land.autogen(config);

    // Print the landmass to terminal using ncurses
    //land.termion_print();
}
