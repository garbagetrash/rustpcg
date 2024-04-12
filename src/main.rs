mod render;
mod terrain;
mod render_image;
use crate::terrain::{AutoGenConfig, Landmass};

fn main() {
    // Create the new landmass and tell it to autogen the world
    let mut land = Landmass::<140, 60>::new();
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
    let image: Vec<Vec<u8>> = land.height_map.to_vecs().iter().map(|row| {
        let mut t = vec![];
        for tt in row {
            t.push(((127.0 * tt) + 128.0) as u8);
        }
        t
    }).collect();
    render_image::render_greyscale("output.png", &image);

    // Print the landmass to terminal using ncurses
    land.tui_render();
}
