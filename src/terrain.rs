use noise::{Fbm, MultiFractal, NoiseFn, Simplex};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};

// https://en.wikipedia.org/wiki/Biome#/media/File:Climate_influence_on_terrestrial_biome.svg
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Biome {
    Tundra,
    BorealForest,
    TemperateRainforest,
    TemperateSeasonalForest,
    Shrubland,
    ColdDesert,
    TropicalRainforest,
    Savanna,
    SubtropicalDesert,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Feature {
    RiverSource,
    River,
    Ocean,
}

#[derive(Clone)]
pub struct Grid<T, const X: usize, const Y: usize> {
    value: [[T; Y]; X],
}

impl<T, const X: usize, const Y: usize> Grid<T, X, Y> {
    pub fn new(grid: [[T; Y]; X]) -> Grid<T, X, Y> {
        Grid { value: grid }
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

impl<T, const X: usize, const Y: usize> Index<usize> for Grid<T, X, Y> {
    type Output = [T; Y];

    fn index(&self, x: usize) -> &Self::Output {
        &self.value[x]
    }
}

impl<T, const X: usize, const Y: usize> IndexMut<usize> for Grid<T, X, Y> {
    fn index_mut(&mut self, x: usize) -> &mut Self::Output {
        &mut self.value[x]
    }
}

pub struct Lake<const X: usize, const Y: usize> {
    height_map: Grid<f64, X, Y>,
    tiles: HashSet<(usize, usize)>,
    perimeter: HashSet<(usize, usize)>,
}

impl<const X: usize, const Y: usize> Lake<X, Y> {
    pub fn new(height_map: Grid<f64, X, Y>) -> Lake<X, Y> {
        Lake {
            height_map: height_map,
            tiles: HashSet::new(),
            perimeter: HashSet::new(),
        }
    }

    pub fn _insert(&mut self, tile: (usize, usize)) {
        self.tiles.insert(tile);
        self.perimeter.remove(&tile);
        let neighbors = self.height_map.get_neighbors(&tile);

        for n in neighbors {
            if !self.tiles.contains(&n) {
                self.perimeter.insert(n);
            }
        }
    }

    pub fn fill(
        &mut self,
        tile: (usize, usize),
        ocean_height: f64,
        river_tile_limit: usize,
    ) -> Vec<(usize, usize)> {
        let mut tile_cntr = 0;

        self._insert(tile);
        tile_cntr += 1;

        loop {
            // Find lowest height perimeter tile
            let mut min_tile = tile;
            let mut min_height = f64::MAX;
            for p in self.perimeter.iter() {
                let h = self.height_map[p.0][p.1];
                if min_height > h {
                    min_tile = p.clone();
                    min_height = h;
                }
            }

            // If we've hit the ocean, we're done
            if min_height < ocean_height {
                break;
            }
            if tile_cntr >= river_tile_limit {
                break;
            }

            self._insert(min_tile);
            tile_cntr += 1;
        }

        self.tiles.remove(&tile);
        self.tiles.iter().map(|x| *x).collect()
    }
}

pub fn temp_map_value_to_degrees_c(value: f64) -> f64 {
    // Input values on range [-1.0, 1.0]... map to [-10.0, 32.0]
    let min_temp = -10.0;
    let max_temp = 32.0;
    let scaler = (max_temp - min_temp) / 2.0;
    scaler * (value + 1.0) + min_temp
}

pub fn precip_map_value_to_cm_rainfall(value: f64, temperature: f64) -> f64 {
    // Input values on range [-1.0, 1.0]... map to [0.0, 450.0].
    // Temperature values on range [-1.0, 1.0] as well.
    // Scale by temperature, basically precip * temp = precip, approximately
    let min_temp = 0.0;
    let max_temp = 450.0;
    let scaler = (max_temp - min_temp) / 2.0;
    (scaler * (value + 1.0) - min_temp) * ((temperature + 1.0) / 2.0)
}

pub struct AutoGenConfig {
    pub x_scale: f64,
    pub y_scale: f64,
    pub landmass_frequency: f64,
    pub precip_frequency: f64,
    pub precip_offset: f64,
    pub temperature_frequency: f64,
    pub temperature_offset: f64,
    pub ocean_height: f64,
    pub river_tile_limit: usize,
    pub seed: Option<u32>,
}

pub struct Landmass<const X: usize, const Y: usize> {
    pub height_map: Grid<f64, X, Y>,
    pub precip_map: Grid<f64, X, Y>,
    pub biome_map: Grid<Biome, X, Y>,
    pub temperature_map: Grid<f64, X, Y>,
    pub features: HashMap<(usize, usize), Feature>,
    pub render: bool,
}

impl<const X: usize, const Y: usize> Landmass<X, Y> {
    pub fn new() -> Landmass<X, Y> {
        Landmass {
            height_map: Grid::<f64, X, Y>::new([[0.0; Y]; X]),
            precip_map: Grid::<f64, X, Y>::new([[0.0; Y]; X]),
            biome_map: Grid::<Biome, X, Y>::new([[Biome::Tundra; Y]; X]),
            temperature_map: Grid::<f64, X, Y>::new([[0.0; Y]; X]),
            features: HashMap::new(),
            render: false,
        }
    }

    pub fn populate_ocean(&mut self, ocean_height: f64) {
        for x in 0..X {
            for y in 0..Y {
                if self.height_map[x][y] < ocean_height {
                    self.features.insert((x, y), Feature::Ocean);
                }
            }
        }
    }

    pub fn generate_height_map(&mut self, config: &AutoGenConfig) {
        // Generate the height_map
        let g = Fbm::<Simplex>::default();
        let g = g.set_frequency(config.landmass_frequency);
        for x in 0..X {
            for y in 0..Y {
                self.height_map[x][y] =
                    g.get([x as f64 / config.x_scale, y as f64 / config.y_scale]);
            }
        }
    }

    pub fn generate_precipitation_map(&mut self, config: &AutoGenConfig) {
        // Generate the precip_map
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen();
        let g = Fbm::<Simplex>::new(seed);
        let g = g.set_frequency(config.precip_frequency);
        for x in 0..X {
            for y in 0..Y {
                self.precip_map[x][y] = 1.5
                    * g.get([x as f64 / config.x_scale, y as f64 / config.y_scale])
                    + config.precip_offset;
                if self.precip_map[x][y] > 1.0 {
                    self.precip_map[x][y] = 1.0;
                } else if self.precip_map[x][y] < -1.0 {
                    self.precip_map[x][y] = -1.0;
                }
            }
        }
    }

    pub fn generate_temperature_map(&mut self, config: &AutoGenConfig) {
        // Generate the temperature_map in degrees C
        let mut rng = rand::thread_rng();
        let g = Fbm::<Simplex>::new(rng.gen());
        let g = g.set_frequency(config.temperature_frequency);
        for x in 0..X {
            for y in 0..Y {
                // Let natural temp be 0.8 at equator, -0.5 at poles
                let mut temp = -2.6 * ((y as f64) / (Y as f64) - 0.5).abs() + 0.8;

                // Consider height map (higher altitude -> lower temp)
                let h = self.height_map[x][y];
                if h > 0.8 {
                    // Tall mountains (range of ~45, so 0.1 = 2.25 deg. C)
                    temp -= 10.0 * (h - 0.8) + 0.9;
                } else if h > 0.7 {
                    // Foothills
                    temp -= 5.0 * (h - 0.7) + 0.4;
                } else if h > 0.0 {
                    temp -= 0.8 * h;
                }

                // Get a random [-0.5, 0.5] value
                temp += g.get([x as f64 / config.x_scale, y as f64 / config.y_scale]) / 2.0;

                // Factor in config
                temp += config.temperature_offset;

                // Saturate to [-1.0, 1.0]
                if temp > 1.0 {
                    temp = 1.0;
                } else if temp < -1.0 {
                    temp = -1.0;
                }

                // Assign when done
                self.temperature_map[x][y] = temp;
            }
        }
    }

    pub fn generate_biome_map(&mut self) {
        // Generate the biome map
        for x in 0..X {
            for y in 0..Y {
                let norm_temp = self.temperature_map[x][y];
                let temp = temp_map_value_to_degrees_c(norm_temp);
                let norm_precip = self.precip_map[x][y];
                let precip = precip_map_value_to_cm_rainfall(norm_precip, norm_temp);

                // Anything below 0.0 C is Tundra, roughly
                let mut biome = Biome::Tundra;

                // Boreal forest
                if temp < 7.0 && temp > 0.0 && precip > 40.0 {
                    biome = Biome::BorealForest;
                }

                // Temperate grassland/Cold desert
                if temp > 0.0 && temp < 22.0 && precip < 50.0 {
                    biome = Biome::ColdDesert;
                }

                // Woodland/Shrubland 50 cm at 7 C, 120 cm at 22 C
                // 120 - 50 = 70, 22 - 7 = 15, slope = 70 / 15 = 4.67
                // y-intercept = 50 - 7 * 4.67 = 50 - 32.67 = 17.33
                if temp > 7.0 && temp < 22.0 && precip > 50.0 && precip < 17.33 + 4.67 * temp {
                    biome = Biome::Shrubland;
                }

                // Temperate seasonal forest 170 cm at 7 C, 230 at 22 C
                // 230 - 170 = 60, 22 - 7 = 15, slope = 60 / 15 = 4
                // y-intercept = 170 - 7 * 4 = 170 - 28 = 142
                if temp > 7.0
                    && temp < 22.0
                    && precip > 17.33 + 4.67 * temp
                    && precip < 170.0 + 4.0 * temp
                {
                    biome = Biome::TemperateSeasonalForest;
                }

                // Temperate rainforest
                if temp > 7.0 && temp < 22.0 && precip > 170.0 + 4.0 * temp {
                    biome = Biome::TemperateSeasonalForest;
                }

                // Subtropical desert 50 cm at 22 C, 100 cm at 32 C
                // 100 - 50 = 50, 32 - 22 = 10, slope = 50 / 10 = 5
                // y-intercept = 50 - 22 * 5 = 50 - 110 = -60
                if temp > 22.0 && precip < 5.0 * temp - 60.0 {
                    biome = Biome::SubtropicalDesert;
                }

                // Tropical seasonal forest/savanna 230 cm at 22 C, 280 cm at 32 C
                // 280 - 230 = 50, 32 - 22 = 10, slope = 50 / 10 = 5
                // y-intercept = 280 - 22 * 5 = 280 - 110 = 170
                if temp > 22.0 && precip > 5.0 * temp - 60.0 && precip < 5.0 * temp + 170.0 {
                    biome = Biome::Savanna;
                }

                // Tropical rainforest
                if temp > 22.0 && precip > 5.0 * temp + 170.0 {
                    biome = Biome::TropicalRainforest;
                }

                self.biome_map[x][y] = biome;
            }
        }
    }

    pub fn river_path(
        &mut self,
        start: (usize, usize),
        ocean_height: f64,
        river_tile_limit: usize,
    ) {
        // Iterate until done
        let mut lake = Lake::<X, Y>::new(self.height_map.clone());
        let new_river_tiles = lake.fill(start, ocean_height, river_tile_limit);
        for tile in new_river_tiles {
            self.features.insert(tile, Feature::River);
        }
    }

    pub fn fill_rivers(&mut self, config: &AutoGenConfig) {
        // Populate river sources
        let mut rng = rand::thread_rng();
        for x in 0..X {
            for y in 0..Y {
                let p = self.precip_map[x][y];

                let mut any_sources = false;
                let neighbors = self.height_map.get_neighbors(&(x, y));

                for n in neighbors {
                    if let Some(feature) = self.features.get(&n) {
                        if *feature == Feature::RiverSource {
                            any_sources = true;
                            break;
                        }
                    }
                }

                // If no neighboring sources, create one with some probability proportional to
                // the precipitation at this tile.
                if !any_sources {
                    if rng.gen::<f64>() > 1.0 - 0.1 * p {
                        // If above some height, x% chance for tile to be a
                        // source.  Later add in contraint keeping sources
                        // away from each other I'd think.
                        self.features.insert((x, y), Feature::RiverSource);
                    }
                }
            }
        }

        // Path the rivers
        let feature_copy = self.features.clone();
        for (k, v) in feature_copy.iter() {
            if *v == Feature::RiverSource {
                self.river_path(*k, config.ocean_height, config.river_tile_limit);
            }
        }
    }

    pub fn autogen(&mut self, config: &AutoGenConfig) {
        self.generate_height_map(config);

        self.generate_precipitation_map(config);

        self.generate_temperature_map(config);

        self.generate_biome_map();

        self.populate_ocean(config.ocean_height);

        self.fill_rivers(config);
    }
}
