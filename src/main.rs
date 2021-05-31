#![feature(min_const_generics)]
mod terrain;
use crate::terrain::terrain::Landmass;

fn main() {
    // Create the new landmass and tell it to autogen the world
    let mut land = Landmass::<141, 79>::new();
    land.render = true;
    land.autogen(4.0);

    // Print the landmass to terminal using ncurses
    //land.termion_print();
}
