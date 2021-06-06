use termion::color::{Bg, Fg, Rgb};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;
use crate::terrain::{Biome, Landmass, Feature};
use std::collections::HashSet;
use std::io::{stdout, stdin, Write};

impl Biome {
    fn get_color(&self) -> Rgb {
        match self {
            Biome::Tundra => Rgb(147, 168, 173),
            Biome::BorealForest => Rgb(0, 80, 70),
            Biome::TemperateRainforest => Rgb(25, 55, 0),
            Biome::TemperateSeasonalForest => Rgb(145, 215, 70),
            Biome::Shrubland => Rgb(130, 150, 100),
            Biome::ColdDesert => Rgb(210, 190, 140),
            Biome::TropicalRainforest => Rgb(48, 127, 55),
            Biome::Savanna => Rgb(202, 139, 43),
            Biome::SubtropicalDesert => Rgb(245, 200, 80),
        }
    }
}

impl<const X: usize, const Y: usize> Landmass<X, Y> {

    pub fn tui_render(&self) {
        let stdin = stdin();
        let mut stdout = stdout()
            .into_raw_mode()
            .expect("Failed to enter raw mode for termion.");
        writeln!(stdout, "{}{}", clear::All, cursor::Hide).expect("Failed to writeln!()");

        let offset: u8 = 0;
        let mut used_biome_set = HashSet::<Biome>::new();

        for x in 0..X {
            for y in 0..Y {
                let value = (127.0 * (self.height_map[x][y] + 1.0)) as u8;

                let (tile_color, tile_color_bg, tile_char) = {
                    if let Some(feature) = self.features.get(&(x, y)) {
                        match feature {
                            Feature::RiverSource => {
                                let tc = color::Fg(color::Rgb(0, 255, 255));
                                let tcb =
                                    color::Bg(color::Rgb(0, 0, value.saturating_add(offset)));
                                let tchar = 'o';
                                (tc, tcb, tchar)
                            }
                            Feature::River => {
                                let tc =
                                    color::Fg(color::Rgb(0, 80, value.saturating_add(offset)));
                                let tcb =
                                    color::Bg(color::Rgb(0, 0, value.saturating_add(offset)));
                                let tchar = '~';
                                (tc, tcb, tchar)
                            }
                            Feature::Ocean => {
                                let tc =
                                    color::Fg(color::Rgb(0, 0, value.saturating_add(offset)));
                                let tcb =
                                    color::Bg(color::Rgb(0, 0, value.saturating_add(offset)));
                                let tchar = '~';
                                (tc, tcb, tchar)
                            }
                        }
                    } else {
                        let biome = self.biome_map[x][y];
                        used_biome_set.insert(biome);

                        let (fgc, bgc, tc) = self.get_biome_tile(biome, x, y);
                        (fgc, bgc, tc)
                    }
                };
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
        writeln!(
            stdout,
            "{}{}{}Terrain map",
            cursor::Goto(1, (Y + 1) as u16),
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )
        .expect("Failed to write!()");

        // Print out biome colors
        let mut cntr = 2;
        for b in used_biome_set {
            writeln!(
                stdout,
                "{}{}{}##{}{} {:?}\t",
                cursor::Goto(1, (Y + cntr) as u16),
                Fg(b.get_color()),
                Bg(b.get_color()),
                Fg(color::Reset),
                Bg(color::Reset),
                b,
            )
            .expect("Failed to write!()");
            cntr += 1;
        }

        stdout.flush().expect("Failed to flush stdout");
        for _k in stdin.keys() {
            break;
        }
        println!("{}{}{}\n\r", style::Reset, clear::All, cursor::Show);
    }

    pub fn precip_tui_render(&self) {
        let stdin = stdin();
        let mut stdout = stdout()
            .into_raw_mode()
            .expect("Failed to enter raw mode for termion.");
        writeln!(stdout, "{}{}", clear::All, cursor::Hide).expect("Failed to writeln!()");
        for x in 0..X {
            for y in 0..Y {
                let value = (127.0 * (self.precip_map[x][y] + 1.0)) as u8;
                let tile_color = color::Fg(color::Rgb(value, value, value));
                let tile_color_bg = color::Bg(color::Rgb(value, value, value));
                let tile_char = '#';
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
        writeln!(
            stdout,
            "{}{}{}Precipitation map",
            cursor::Goto(1, (Y + 1) as u16),
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )
        .expect("Failed to write!()");
        stdout.flush().expect("Failed to flush stdout");
        for _k in stdin.keys() {
            break;
        }
        println!("{}{}{}\n\r", style::Reset, clear::All, cursor::Show);
    }

    pub fn temperature_tui_render(&self) {
        let stdin = stdin();
        let mut stdout = stdout()
            .into_raw_mode()
            .expect("Failed to enter raw mode for termion.");
        writeln!(stdout, "{}{}", clear::All, cursor::Hide).expect("Failed to writeln!()");
        for x in 0..X {
            for y in 0..Y {
                let value = (127.0 * (self.temperature_map[x][y] + 1.0)) as u8;
                let tile_color = color::Fg(color::Rgb(value, value, value));
                let tile_color_bg = color::Bg(color::Rgb(value, value, value));
                let tile_char = '#';
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
        writeln!(
            stdout,
            "{}{}{}Temperature map",
            cursor::Goto(1, (Y + 1) as u16),
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )
        .expect("Failed to write!()");
        stdout.flush().expect("Failed to flush stdout");
        for _k in stdin.keys() {
            break;
        }
        println!("{}{}{}\n\r", style::Reset, clear::All, cursor::Show);
    }

    pub fn get_biome_tile(&self, biome: Biome, x: usize, y: usize) -> (Fg<Rgb>, Bg<Rgb>, char) {
        let h = (1.0 + self.height_map[x][y]) / 2.0;
        let rgb = biome.get_color();

        let r: u8 = ((rgb.0 as f64) * h) as u8;
        let g: u8 = ((rgb.1 as f64) * h) as u8;
        let b: u8 = ((rgb.2 as f64) * h) as u8;

        let mut tile_color = Fg(Rgb(r, g, b));
        let tile_color_bg = Bg(Rgb(r, g, b));

        let mut tile_char = '#';
        match biome {
            Biome::TropicalRainforest => {
                tile_color = Fg(Rgb(0, 100, 0));
                tile_char = 't';
            },
            Biome::BorealForest => {
                tile_color = Fg(Rgb(0, 60, 0));
                tile_char = 't';
            },
            Biome::TemperateSeasonalForest => {
                tile_color = Fg(Rgb(70, 100, 40));
                tile_char = 'p';
            },
            _ => {},
        }

        if h > 0.8 {
            tile_color = Fg(Rgb(255, 255, 255));
            tile_char = '^';
        } else if h > 0.7 {
            tile_color = Fg(Rgb(90, 70, 70));
            tile_char = '^';
        }

        (tile_color, tile_color_bg, tile_char)
    }

    pub fn biome_tui_render(&self) {
        let stdin = stdin();
        let mut stdout = stdout()
            .into_raw_mode()
            .expect("Failed to enter raw mode for termion.");
        writeln!(stdout, "{}{}", clear::All, cursor::Hide).expect("Failed to writeln!()");

        let mut used_biome_set = HashSet::<Biome>::new();
        for x in 0..X {
            for y in 0..Y {
                let biome = self.biome_map[x][y];
                let (tile_color, tile_color_bg, tile_char) = self.get_biome_tile(biome, x, y);

                used_biome_set.insert(biome);
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
        writeln!(
            stdout,
            "{}{}{}Biome map",
            cursor::Goto(1, (Y + 1) as u16),
            Fg(color::Reset),
            Bg(color::Reset),
        )
        .expect("Failed to write!()");

        // Print out biome colors
        let mut cntr = 2;
        for b in used_biome_set {
            writeln!(
                stdout,
                "{}{}{}##{}{} {:?}\t",
                cursor::Goto(1, (Y + cntr) as u16),
                Fg(b.get_color()),
                Bg(b.get_color()),
                Fg(color::Reset),
                Bg(color::Reset),
                b,
            )
            .expect("Failed to write!()");
            cntr += 1;
        }

        stdout.flush().expect("Failed to flush stdout");
        for _k in stdin.keys() {
            break;
        }
        println!("{}{}{}\n\r", style::Reset, clear::All, cursor::Show);
    }
}
