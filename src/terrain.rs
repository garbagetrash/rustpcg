pub mod terrain {
    use ncurses::*;
    use noise::{Fbm, MultiFractal, NoiseFn};
    use rand::prelude::*;
    use std::collections::HashMap;

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

        pub fn curses_start(&self) {
            initscr();
            start_color();
            keypad(stdscr(), true);
            noecho();

            // Get the number of colors supported by terminal
            let n_colors = COLORS() as i16;

            // Max value specified as 1000 by curses.
            let scaler: i16 = 1000 / n_colors;

            for i in 0..n_colors - 1 {
                init_color(i, scaler * i, scaler * i, scaler * i);
                init_pair(i, i as i16, i as i16);
            }

            // River Sources
            init_color(n_colors - 1, 0, 0, 1000);
            init_pair(n_colors - 1, n_colors - 1, n_colors - 1);

            curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        }

        pub fn curses_print(&self) {
            clear();

            let n_colors = COLORS() as i16;
            let half_value = COLORS() as f64 / 2.0;
            for x in 0..X {
                for y in 0..Y {
                    let mut c = (half_value * (self.height[x][y] + 1.0)) as i16;
                    if let Some(feature) = self.features.get(&(x, y)) {
                        match feature {
                            Feature::RiverSource => c = n_colors - 1,
                            Feature::Ocean => c = n_colors - 1,
                        }
                    }
                    attron(COLOR_PAIR(c));
                    mvprintw(y as i32, x as i32, "#");
                    attroff(COLOR_PAIR(c));
                }
            }

            refresh();
            let _ch = getch();
        }

        pub fn curses_stop(&self) {
            curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
            endwin();
        }
    }
}
