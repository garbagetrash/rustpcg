pub mod terrain {
    use noise::Seedable;
    use noise::{Fbm, MultiFractal, NoiseFn};
    use rand::prelude::*;
    use std::collections::{HashMap, HashSet};
    use std::io::{stdin, stdout, Write};
    use std::ops::{Index, IndexMut};
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use termion::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Feature {
        RiverSource,
        River,
        Ocean,
    }

    #[derive(Clone)]
    pub struct Grid<const X: usize, const Y: usize> {
        value: [[f64; Y]; X],
    }

    impl<const X: usize, const Y: usize> Grid<X, Y> {
        pub fn new() -> Grid<X, Y> {
            Grid {
                value: [[0_f64; Y]; X],
            }
        }

        pub fn width(&self) -> usize {
            X
        }

        pub fn height(&self) -> usize {
            Y
        }

        pub fn get_neighbors(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
            let mut output = vec![];

            if point.0 + 1 < X {
                output.push((point.0 + 1, point.1));
                if point.1 + 1 < Y {
                    output.push((point.0 + 1, point.1 + 1));
                }
                if point.1 > 0 {
                    output.push((point.0 + 1, point.1 - 1));
                }
            }
            if point.0 > 0 {
                output.push((point.0 - 1, point.1));
                if point.1 + 1 < Y {
                    output.push((point.0 - 1, point.1 + 1));
                }
                if point.1 > 0 {
                    output.push((point.0 - 1, point.1 - 1));
                }
            }
            if point.1 > 0 {
                output.push((point.0, point.1 - 1));
            }
            if point.1 + 1 < Y {
                output.push((point.0, point.1 + 1));
            }

            output
        }
    }

    impl<const X: usize, const Y: usize> Index<usize> for Grid<X, Y> {
        type Output = [f64; Y];

        fn index(&self, x: usize) -> &Self::Output {
            &self.value[x]
        }
    }

    impl<const X: usize, const Y: usize> IndexMut<usize> for Grid<X, Y> {
        fn index_mut(&mut self, x: usize) -> &mut Self::Output {
            &mut self.value[x]
        }
    }

    pub struct Lake<const X: usize, const Y: usize> {
        heightmap: Grid<X, Y>,
        tiles: HashSet<(usize, usize)>,
        perimeter: HashSet<(usize, usize)>,
    }

    impl<const X: usize, const Y: usize> Lake<X, Y> {
        pub fn new(heightmap: Grid<X, Y>) -> Lake<X, Y> {
            Lake {
                heightmap: heightmap,
                tiles: HashSet::new(),
                perimeter: HashSet::new(),
            }
        }

        pub fn _insert(&mut self, tile: (usize, usize)) {
            self.tiles.insert(tile);
            self.perimeter.remove(&tile);
            let mut neighbors = self.heightmap.get_neighbors(&tile);

            for n in neighbors {
                if !self.tiles.contains(&n) {
                    self.perimeter.insert(n);
                }
            }
        }

        pub fn fill(&mut self, tile: (usize, usize), ocean_height: f64) -> Vec<(usize, usize)> {
            self._insert(tile);

            let mut lake_height = self.heightmap[tile.0][tile.1];
            println!("lake_height: {}", lake_height);

            loop {
                // Find lowest height perimeter tile
                let mut min_tile = tile;
                let mut min_height = f64::MAX;
                for p in self.perimeter.iter() {
                    let h = self.heightmap[p.0][p.1];
                    if min_height > h {
                        min_tile = p.clone();
                        min_height = h;
                    }
                }

                // If we've hit the ocean, we're done
                println!("min_height: {}", min_height);
                println!("ocean_height: {}", ocean_height);
                if min_height < ocean_height {
                    break;
                }

                lake_height = min_height;
                println!("lake_height: {}", lake_height);

                self._insert(min_tile);
            }

            self.tiles.remove(&tile);
            self.tiles.iter().map(|x| *x).collect()
        }
    }

    pub struct Landmass<const X: usize, const Y: usize> {
        pub heightmap: Grid<X, Y>,
        pub features: HashMap<(usize, usize), Feature>,
        pub ocean_height: f64,
        pub render: bool,
    }

    impl<const X: usize, const Y: usize> Landmass<X, Y> {
        pub fn new() -> Landmass<X, Y> {
            Landmass {
                heightmap: Grid::<X, Y>::new(),
                features: HashMap::new(),
                ocean_height: -0.25,
                render: false,
            }
        }

        pub fn populate_ocean(&mut self) {
            for x in 0..X {
                for y in 0..Y {
                    if self.heightmap[x][y] < self.ocean_height {
                        self.features.insert((x, y), Feature::Ocean);
                    }
                }
            }
        }

        pub fn autogen(&mut self, scale: f64) {
            // Generate the heightmap
            let g = Fbm::new();
            let mut rng = rand::thread_rng();
            let seed: u32 = rng.gen();
            let g = g.set_seed(seed);
            let g = g.set_frequency(scale);
            for x in 0..X {
                for y in 0..Y {
                    self.heightmap[x][y] = g.get([x as f64 / X as f64, y as f64 / Y as f64]);
                }
            }

            if (self.render) {
                self.tui_render();
            }

            // Populate ocean tiles
            self.populate_ocean();

            if (self.render) {
                self.tui_render();
            }

            // Populate river sources
            let mut rng = rand::thread_rng();
            for x in 0..X {
                for y in 0..Y {
                    let h = self.heightmap[x][y];

                    // Above some height, eligible to be a source
                    if h > 0.05 {
                        if rng.gen::<f64>() > 0.998_f64 {
                            // If above some height, 5% chance for tile to be a
                            // source.  Later add in contraint keeping sources
                            // away from each other I'd think.
                            self.features.insert((x, y), Feature::RiverSource);
                        }
                    }
                }
            }

            if (self.render) {
                self.tui_render();
            }

            // Path the rivers
            let feature_copy = self.features.clone();
            println!("{}", clear::All);
            for (k, v) in feature_copy.iter() {
                if *v == Feature::RiverSource {
                    println!("{:?}", k);
                    self.river_path(*k);
                }
            }

            if (self.render) {
                self.tui_render();
            }
        }

        pub fn river_path(&mut self, start: (usize, usize)) {
            // Iterate until done
            println!("Calling lake.fill()...");
            let mut lake = Lake::<X, Y>::new(self.heightmap.clone());
            let new_river_tiles = lake.fill(start, self.ocean_height);
            for tile in new_river_tiles {
                self.features.insert(tile, Feature::River);
            }
        }

        pub fn tui_render(&self) {
            let stdin = stdin();
            let mut stdout = stdout()
                .into_raw_mode()
                .expect("Failed to enter raw mode for termion.");
            writeln!(stdout, "{}", clear::All).expect("Failed to writeln!()");
            let offset: u8 = 0;
            for x in 0..X {
                for y in 0..Y {
                    let value = (127.0 * (self.heightmap[x][y] + 1.0)) as u8;
                    let mut tile_color = color::Fg(color::Rgb(value, value, value));
                    let mut tile_color_bg = color::Bg(color::Rgb(value, value, value));
                    let mut tile_char = '#';

                    if let Some(feature) = self.features.get(&(x, y)) {
                        match feature {
                            Feature::RiverSource => {
                                tile_color = color::Fg(color::Rgb(0, 255, 255));
                                tile_color_bg =
                                    color::Bg(color::Rgb(0, 0, value.saturating_add(offset)));
                                tile_char = 'O';
                            }
                            Feature::River => {
                                tile_color =
                                    color::Fg(color::Rgb(0, 255, value.saturating_add(offset)));
                                tile_color_bg =
                                    color::Bg(color::Rgb(0, 0, value.saturating_add(offset)));
                                tile_char = '~';
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
                        "{goto}{color}{bg}{char}",
                        goto = cursor::Goto((x + 1) as u16, (y + 1) as u16),
                        color = tile_color,
                        bg = tile_color_bg,
                        char = tile_char,
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
