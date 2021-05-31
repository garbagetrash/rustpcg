pub mod terrain {
    use noise::{Fbm, MultiFractal, NoiseFn};
    use rand::prelude::*;
    use std::collections::HashMap;
    use std::io::{stdin, stdout, Write};
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use termion::*;

    #[derive(Debug)]
    pub enum Feature {
        RiverSource,
        Ocean,
    }

    pub struct Landmass<const X: usize, const Y: usize> {
        pub height: [[f64; Y]; X],
        pub features: HashMap<(usize, usize), Feature>,
        pub ocean_height: f64,
    }

    impl<const X: usize, const Y: usize> Landmass<X, Y> {
        pub fn new() -> Landmass<X, Y> {
            Landmass {
                height: [[0_f64; Y]; X],
                features: HashMap::new(),
                ocean_height: -0.25,
            }
        }

        pub fn width(&self) -> usize {
            X
        }

        pub fn height(&self) -> usize {
            Y
        }

        pub fn populate_ocean(&mut self) {
            for x in 0..X {
                for y in 0..Y {
                    if self.height[x][y] < self.ocean_height {
                        self.features.insert((x, y), Feature::Ocean);
                    }
                }
            }
        }

        pub fn get_neighbors(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
            let mut output = vec![];

            if point.0 < X {
                output.push((point.0 + 1, point.1));
                if point.1 < Y {
                    output.push((point.0 + 1, point.1 + 1));
                }
                if point.1 > 0 {
                    output.push((point.0 + 1, point.1 - 1));
                }
            }
            if point.0 > 0 {
                output.push((point.0 - 1, point.1));
                if point.1 < Y {
                    output.push((point.0 - 1, point.1 + 1));
                }
                if point.1 > 0 {
                    output.push((point.0 - 1, point.1 - 1));
                }
            }
            if point.1 > 0 {
                output.push((point.0, point.1 - 1));
            }
            if point.1 < Y {
                output.push((point.0, point.1 + 1));
            }

            output
        }

        pub fn autogen(&mut self, scale: f64) {
            // Generate the heightmap
            let g = Fbm::new();
            let g = g.set_frequency(scale);
            for x in 0..X {
                for y in 0..Y {
                    self.height[x][y] = g.get([x as f64 / X as f64, y as f64 / Y as f64]);
                }
            }

            // Populate ocean tiles
            self.populate_ocean();

            // Populate river sources
            let mut rng = rand::thread_rng();
            for x in 0..X {
                for y in 0..Y {
                    let h = self.height[x][y];

                    // Above some height, eligible to be a source
                    if h > 0.6 {
                        if rng.gen::<f64>() > 0.92_f64 {
                            // If above some height, 5% chance for tile to be a
                            // source.  Later add in contraint keeping sources
                            // away from each other I'd think.
                            self.features.insert((x, y), Feature::RiverSource);
                        }
                    }
                }
            }

            // TODO: Path the rivers
            /*
            for (k, _v) in self.features.iter() {
                let river = self.river_path(k);
            }
            */
        }

        pub fn print(&self) {
            for x in 0..X {
                for y in 0..Y {
                    print!("{}\t", self.height[x][y]);
                }
                print!("\n");
            }
        }

        pub fn termion_print(&self) {
            let stdin = stdin();
            let mut stdout = stdout()
                .into_raw_mode()
                .expect("Failed to enter raw mode for termion.");
            writeln!(stdout, "{}", clear::All).expect("Failed to writeln!()");
            let offset: u8 = 50;
            for x in 0..X {
                for y in 0..Y {
                    let value = (127.0 * (self.height[x][y] + 1.0)) as u8;
                    let mut tile_color = color::Fg(color::Rgb(value, value, value));
                    let mut tile_color_bg = color::Bg(color::Rgb(value, value, value));

                    if let Some(feature) = self.features.get(&(x, y)) {
                        match feature {
                            Feature::RiverSource => {
                                tile_color =
                                    color::Fg(color::Rgb(0, 0, value.saturating_add(offset)));
                                tile_color_bg =
                                    color::Bg(color::Rgb(0, 0, value.saturating_add(offset)));
                            }
                            Feature::Ocean => {
                                tile_color =
                                    color::Fg(color::Rgb(0, 0, value.saturating_add(offset)));
                                tile_color_bg =
                                    color::Bg(color::Rgb(0, 0, value.saturating_add(offset)));
                            }
                        }
                    }
                    write!(
                        stdout,
                        "{goto}{color}{bg}#",
                        goto = cursor::Goto((x + 1) as u16, (y + 1) as u16),
                        color = tile_color,
                        bg = tile_color_bg
                    )
                    .expect("Failed to write!()");
                }
            }
            stdout.flush().expect("Failed to flush stdout");
            for _k in stdin.keys() {
                break;
            }
            writeln!(stdout, "{}{}", style::Reset, clear::All).expect("Failed to writeln!()");
        }
    }
}
