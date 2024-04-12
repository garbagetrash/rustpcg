mod render;
mod terrain;
use crate::terrain::{AutoGenConfig, Landmass};

fn main() {
    // Create the new landmass and tell it to autogen the world
    let mut land = Landmass::<140, 70>::new();
    land.render = true;

    let config = AutoGenConfig {
        x_scale: 200.,
        y_scale: 100.,
        landmass_frequency: 4.0,
        precip_frequency: 6.0,
        precip_offset: 0.0,
        temperature_frequency: 2.0,
        temperature_offset: 0.0,
        ocean_height: -1.0,
        river_tile_limit: 400,
        seed: None,
    };

    land.autogen(&config);

    // Print the landmass to terminal using ncurses
    land.tui_render();
}
